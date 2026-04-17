//! # 🔄 Task Scheduler & Execution Pipeline
//!
//! The Scheduler manages the execution of cross-module calls, handling:
//! - Task queuing and prioritization
//! - Async execution with configurable parallelism  
//! - Dependency resolution between module calls
//! - Timeout and cancellation support
//! - Execution pipeline stages (validate → prepare → execute → cleanup)
//!
//! ## Execution Model
//!
//! ```text
//! ┌──────────┐    ┌───────────┐    ┌─────────┐    ┌─────────┐
//! │ Validate │ →  │  Prepare  │ →  │ Execute │ →  │ Cleanup │
//! │ (check   │    │ (marshal  │    │ (run    │    │ (free   │
//! │  args)   │    │  memory)  │    │  func)  │    │  temp)  │
//! └──────────┘    └───────────┘    └─────────┘    └─────────┘
//! ```

use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::Ordering as CmpOrdering;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Unique identifier for a scheduled task.
pub type TaskId = u64;

/// Priority level for task execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TaskPriority {
    /// Critical system tasks (memory management, GC).
    Critical = 0,
    /// High priority (real-time data processing).
    High = 1,
    /// Normal priority (standard bridge calls).
    Normal = 2,
    /// Low priority (background optimization, caching).
    Low = 3,
    /// Idle tasks (metrics collection, cleanup).
    Idle = 4,
}

impl PartialOrd for TaskPriority {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Ord for TaskPriority {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        // Lower number = higher priority
        (*other as u8).cmp(&(*self as u8))
    }
}

/// Current state of a scheduled task.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum TaskState {
    /// Task is queued and waiting for execution.
    Queued,
    /// Task is being validated (checking arguments, permissions).
    Validating,
    /// Task is preparing (marshaling memory, resolving modules).
    Preparing,
    /// Task is actively executing.
    Executing,
    /// Task completed successfully.
    Completed {
        result_handle: u32,
        duration: Duration,
    },
    /// Task failed with an error.
    Failed { error: String, duration: Duration },
    /// Task was cancelled.
    Cancelled,
    /// Task timed out.
    TimedOut { elapsed: Duration },
}

/// A scheduled task representing a cross-module function call.
#[derive(Debug, serde::Serialize)]
pub struct Task {
    /// Unique task ID.
    pub id: TaskId,
    /// Priority of this task.
    pub priority: TaskPriority,
    /// Target module name.
    pub target_module: String,
    /// Target function name.
    pub target_function: String,
    /// Handle to arguments in the memory pool.
    pub args_handle: u32,
    /// Current state of the task.
    pub state: TaskState,
    /// When this task was created.
    #[serde(skip)]
    pub created_at: Instant,
    /// Maximum execution time before timeout.
    pub timeout: Duration,
    /// Which module initiated this call.
    pub caller_module: Option<String>,
    /// Parent task ID (for nested bridge calls).
    pub parent_task_id: Option<TaskId>,
    /// Dependencies — task IDs that must complete before this task runs.
    pub dependencies: Vec<TaskId>,
    /// Metadata for tracing and debugging.
    pub metadata: HashMap<String, String>,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        // Higher priority first, then earlier creation time
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.created_at.cmp(&self.created_at))
    }
}

/// The Task Scheduler — manages execution of cross-module calls.
pub struct Scheduler {
    /// Priority queue of pending tasks.
    pending: BinaryHeap<Task>,
    /// Currently executing tasks.
    active: HashMap<TaskId, Task>,
    /// Completed/failed task history (recent).
    history: VecDeque<Task>,
    /// Maximum history size.
    max_history: usize,
    /// Next task ID to assign.
    next_task_id: TaskId,
    /// Maximum number of concurrent tasks.
    max_concurrent: usize,
    /// Total tasks processed (lifetime).
    total_processed: u64,
    /// Total tasks failed (lifetime).
    total_failed: u64,
    /// Default task timeout.
    default_timeout: Duration,
}

impl Scheduler {
    /// Create a new scheduler.
    pub fn new(max_concurrent: usize) -> Self {
        info!(max_concurrent, "Task Scheduler initialized");
        Self {
            pending: BinaryHeap::new(),
            active: HashMap::new(),
            history: VecDeque::new(),
            max_history: 1000,
            next_task_id: 1,
            max_concurrent,
            total_processed: 0,
            total_failed: 0,
            default_timeout: Duration::from_secs(30),
        }
    }

    /// Submit a new task to the scheduler.
    pub fn submit(
        &mut self,
        target_module: &str,
        target_function: &str,
        args_handle: u32,
        priority: TaskPriority,
        caller_module: Option<&str>,
    ) -> TaskId {
        let id = self.next_task_id;
        self.next_task_id += 1;

        let task = Task {
            id,
            priority,
            target_module: target_module.to_string(),
            target_function: target_function.to_string(),
            args_handle,
            state: TaskState::Queued,
            created_at: Instant::now(),
            timeout: self.default_timeout,
            caller_module: caller_module.map(|s| s.to_string()),
            parent_task_id: None,
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        };

        debug!(
            task_id = id,
            module = target_module,
            function = target_function,
            priority = ?priority,
            "Task submitted"
        );

        self.pending.push(task);
        id
    }

    /// Submit a task with dependencies.
    pub fn submit_with_deps(
        &mut self,
        target_module: &str,
        target_function: &str,
        args_handle: u32,
        priority: TaskPriority,
        dependencies: Vec<TaskId>,
    ) -> TaskId {
        let id = self.next_task_id;
        self.next_task_id += 1;

        let task = Task {
            id,
            priority,
            target_module: target_module.to_string(),
            target_function: target_function.to_string(),
            args_handle,
            state: TaskState::Queued,
            created_at: Instant::now(),
            timeout: self.default_timeout,
            caller_module: None,
            parent_task_id: None,
            dependencies,
            metadata: HashMap::new(),
        };

        debug!(
            task_id = id,
            deps = ?task.dependencies,
            "Task submitted with dependencies"
        );

        self.pending.push(task);
        id
    }

    /// Get the next task ready for execution.
    /// Returns None if no tasks are ready or max concurrency is reached.
    pub fn next_ready(&mut self) -> Option<&Task> {
        if self.active.len() >= self.max_concurrent {
            return None;
        }

        // Check if the top task has all dependencies satisfied
        if let Some(task) = self.pending.peek() {
            let deps_met = task.dependencies.iter().all(|dep_id| {
                self.history.iter().any(|t| {
                    t.id == *dep_id && matches!(t.state, TaskState::Completed { .. })
                })
            });

            if deps_met {
                return self.pending.peek();
            }
        }

        None
    }

    /// Dequeue and start executing the next ready task.
    pub fn start_next(&mut self) -> Option<TaskId> {
        if self.active.len() >= self.max_concurrent {
            return None;
        }

        // Find a task with satisfied dependencies
        let mut ready_task = None;
        let mut remaining = BinaryHeap::new();

        while let Some(task) = self.pending.pop() {
            let deps_met = task.dependencies.iter().all(|dep_id| {
                self.history.iter().any(|t| {
                    t.id == *dep_id && matches!(t.state, TaskState::Completed { .. })
                })
            });

            if deps_met && ready_task.is_none() {
                ready_task = Some(task);
            } else {
                remaining.push(task);
            }
        }

        self.pending = remaining;

        if let Some(mut task) = ready_task {
            let id = task.id;
            task.state = TaskState::Executing;
            debug!(task_id = id, "Task started executing");
            self.active.insert(id, task);
            Some(id)
        } else {
            None
        }
    }

    /// Mark a task as completed.
    pub fn complete(&mut self, task_id: TaskId, result_handle: u32) {
        if let Some(mut task) = self.active.remove(&task_id) {
            let duration = task.created_at.elapsed();
            task.state = TaskState::Completed {
                result_handle,
                duration,
            };
            self.total_processed += 1;

            debug!(
                task_id,
                duration_us = duration.as_micros(),
                "Task completed"
            );

            // Add to history
            self.history.push_back(task);
            if self.history.len() > self.max_history {
                self.history.pop_front();
            }
        }
    }

    /// Mark a task as failed.
    pub fn fail(&mut self, task_id: TaskId, error: &str) {
        if let Some(mut task) = self.active.remove(&task_id) {
            let duration = task.created_at.elapsed();
            task.state = TaskState::Failed {
                error: error.to_string(),
                duration,
            };
            self.total_failed += 1;

            warn!(task_id, error, "Task failed");

            self.history.push_back(task);
            if self.history.len() > self.max_history {
                self.history.pop_front();
            }
        }
    }

    /// Cancel a pending or active task.
    pub fn cancel(&mut self, task_id: TaskId) -> bool {
        // Check active tasks
        if let Some(mut task) = self.active.remove(&task_id) {
            task.state = TaskState::Cancelled;
            self.history.push_back(task);
            debug!(task_id, "Active task cancelled");
            return true;
        }

        // Check pending tasks
        let mut found = false;
        let mut remaining = BinaryHeap::new();
        while let Some(mut task) = self.pending.pop() {
            if task.id == task_id {
                task.state = TaskState::Cancelled;
                self.history.push_back(task);
                found = true;
                debug!(task_id, "Pending task cancelled");
            } else {
                remaining.push(task);
            }
        }
        self.pending = remaining;

        found
    }

    /// Check for timed-out tasks and mark them.
    pub fn check_timeouts(&mut self) -> Vec<TaskId> {
        let mut timed_out = Vec::new();

        let expired: Vec<TaskId> = self
            .active
            .iter()
            .filter(|(_, task)| task.created_at.elapsed() > task.timeout)
            .map(|(id, _)| *id)
            .collect();

        for id in expired {
            if let Some(mut task) = self.active.remove(&id) {
                let elapsed = task.created_at.elapsed();
                task.state = TaskState::TimedOut { elapsed };
                self.total_failed += 1;
                warn!(task_id = id, elapsed_ms = elapsed.as_millis(), "Task timed out");
                self.history.push_back(task);
                timed_out.push(id);
            }
        }

        timed_out
    }

    /// Get a snapshot of scheduler statistics.
    pub fn stats(&self) -> SchedulerStats {
        SchedulerStats {
            pending_tasks: self.pending.len(),
            active_tasks: self.active.len(),
            total_processed: self.total_processed,
            total_failed: self.total_failed,
            history_size: self.history.len(),
            max_concurrent: self.max_concurrent,
        }
    }

    /// Get the state of a specific task.
    pub fn task_state(&self, task_id: TaskId) -> Option<&TaskState> {
        // Check active
        if let Some(task) = self.active.get(&task_id) {
            return Some(&task.state);
        }

        // Check pending
        for task in self.pending.iter() {
            if task.id == task_id {
                return Some(&task.state);
            }
        }

        // Check history
        for task in self.history.iter() {
            if task.id == task_id {
                return Some(&task.state);
            }
        }

        None
    }

    /// Print scheduler status.
    pub fn print_status(&self) {
        let stats = self.stats();
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  🔄 Scheduler Status                                 │");
        println!("  ├──────────────────────────────────────────────────────┤");
        println!(
            "  │  Pending: {} │ Active: {} │ Max: {}                 │",
            stats.pending_tasks, stats.active_tasks, stats.max_concurrent
        );
        println!(
            "  │  Processed: {} │ Failed: {}                        │",
            stats.total_processed, stats.total_failed
        );
        println!("  └──────────────────────────────────────────────────────┘");
    }
}

/// Scheduler statistics snapshot.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SchedulerStats {
    pub pending_tasks: usize,
    pub active_tasks: usize,
    pub total_processed: u64,
    pub total_failed: u64,
    pub history_size: usize,
    pub max_concurrent: usize,
}

/// Builder for creating tasks with a fluent API.
pub struct TaskBuilder {
    target_module: String,
    target_function: String,
    args_handle: u32,
    priority: TaskPriority,
    timeout: Option<Duration>,
    caller_module: Option<String>,
    parent_task_id: Option<TaskId>,
    dependencies: Vec<TaskId>,
    metadata: HashMap<String, String>,
}

impl TaskBuilder {
    pub fn new(target_module: &str, target_function: &str, args_handle: u32) -> Self {
        Self {
            target_module: target_module.to_string(),
            target_function: target_function.to_string(),
            args_handle,
            priority: TaskPriority::Normal,
            timeout: None,
            caller_module: None,
            parent_task_id: None,
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn caller(mut self, module: &str) -> Self {
        self.caller_module = Some(module.to_string());
        self
    }

    pub fn parent(mut self, task_id: TaskId) -> Self {
        self.parent_task_id = Some(task_id);
        self
    }

    pub fn depends_on(mut self, task_id: TaskId) -> Self {
        self.dependencies.push(task_id);
        self
    }

    pub fn meta(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Submit this task to a scheduler.
    pub fn submit(self, scheduler: &mut Scheduler) -> TaskId {
        scheduler.submit(
            &self.target_module,
            &self.target_function,
            self.args_handle,
            self.priority,
            self.caller_module.as_deref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_submission_and_execution() {
        let mut scheduler = Scheduler::new(4);

        let id1 = scheduler.submit("calc", "add", 1, TaskPriority::Normal, None);
        let id2 = scheduler.submit("calc", "multiply", 2, TaskPriority::High, None);

        // High priority should be started first
        let started = scheduler.start_next().unwrap();
        assert_eq!(started, id2);

        scheduler.complete(started, 42);
        assert_eq!(scheduler.stats().total_processed, 1);

        let started2 = scheduler.start_next().unwrap();
        assert_eq!(started2, id1);
    }

    #[test]
    fn test_max_concurrency() {
        let mut scheduler = Scheduler::new(2);

        scheduler.submit("a", "f1", 1, TaskPriority::Normal, None);
        scheduler.submit("b", "f2", 2, TaskPriority::Normal, None);
        scheduler.submit("c", "f3", 3, TaskPriority::Normal, None);

        scheduler.start_next();
        scheduler.start_next();

        // Third should be blocked
        assert!(scheduler.start_next().is_none());
        assert_eq!(scheduler.stats().active_tasks, 2);
    }

    #[test]
    fn test_task_cancellation() {
        let mut scheduler = Scheduler::new(4);

        let id = scheduler.submit("calc", "add", 1, TaskPriority::Normal, None);
        assert!(scheduler.cancel(id));
        assert_eq!(
            scheduler.task_state(id),
            Some(&TaskState::Cancelled)
        );
    }

    #[test]
    fn test_priority_ordering() {
        let mut scheduler = Scheduler::new(1);

        scheduler.submit("a", "f1", 1, TaskPriority::Low, None);
        scheduler.submit("b", "f2", 2, TaskPriority::Critical, None);
        scheduler.submit("c", "f3", 3, TaskPriority::Normal, None);

        // Critical should come first
        let first = scheduler.start_next().unwrap();
        // The critical task (id=2) should be first
        assert_eq!(first, 2);
    }

    #[test]
    fn test_task_builder() {
        let mut scheduler = Scheduler::new(4);

        let id = TaskBuilder::new("calc", "fibonacci", 1)
            .priority(TaskPriority::High)
            .caller("orchestrator")
            .timeout(Duration::from_secs(5))
            .meta("reason", "pipeline-step-3")
            .submit(&mut scheduler);

        assert!(id > 0);
        assert_eq!(scheduler.stats().pending_tasks, 1);
    }
}
