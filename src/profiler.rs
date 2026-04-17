//! # 🔍 Cross-Module Call Tracer (Profiler)
//!
//! The Profiler records detailed execution traces of cross-module bridge calls,
//! enabling developers to understand the flow of data and control between
//! language modules. Generates flame graphs, call trees, and timeline views.
//!
//! ## Output Formats
//!
//! - Chrome Tracing JSON (viewable in chrome://tracing)
//! - Flame graph (collapsed stack format)
//! - Call tree (text-based)
//! - Timeline (sequential events)

use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::info;

/// A single trace span representing one function call.
#[derive(Debug, Clone)]
pub struct TraceSpan {
    /// Unique span ID.
    pub id: u64,
    /// Parent span ID (None for root spans).
    pub parent_id: Option<u64>,
    /// Module that was called.
    pub module: String,
    /// Function that was called.
    pub function: String,
    /// Module that initiated the call.
    pub caller_module: Option<String>,
    /// When this span started.
    pub start_time: Instant,
    /// Duration of this span.
    pub duration: Duration,
    /// Whether the call succeeded.
    pub success: bool,
    /// Size of arguments (bytes).
    pub args_size: usize,
    /// Size of result (bytes).
    pub result_size: usize,
    /// Child span IDs.
    pub children: Vec<u64>,
    /// Custom metadata.
    pub metadata: HashMap<String, String>,
}

/// A complete trace session.
#[derive(Debug)]
pub struct TraceSession {
    /// Session identifier.
    pub session_id: String,
    /// When this session started.
    pub start_time: Instant,
    /// All spans in this session.
    pub spans: Vec<TraceSpan>,
    /// Next span ID.
    next_span_id: u64,
    /// Active span stack (for nesting).
    active_stack: Vec<u64>,
    /// Maximum observed depth.
    max_depth: usize,
}

impl TraceSession {
    /// Create a new trace session.
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            start_time: Instant::now(),
            spans: Vec::new(),
            next_span_id: 1,
            active_stack: Vec::new(),
            max_depth: 0,
        }
    }

    /// Begin a new trace span.
    pub fn begin_span(
        &mut self,
        module: &str,
        function: &str,
        caller_module: Option<&str>,
        args_size: usize,
    ) -> u64 {
        let id = self.next_span_id;
        self.next_span_id += 1;

        let parent_id = self.active_stack.last().copied();

        let span = TraceSpan {
            id,
            parent_id,
            module: module.to_string(),
            function: function.to_string(),
            caller_module: caller_module.map(|s| s.to_string()),
            start_time: Instant::now(),
            duration: Duration::ZERO,
            success: true,
            args_size,
            result_size: 0,
            children: Vec::new(),
            metadata: HashMap::new(),
        };

        self.spans.push(span);
        self.active_stack.push(id);

        if self.active_stack.len() > self.max_depth {
            self.max_depth = self.active_stack.len();
        }

        // Register as child of parent
        if let Some(parent) = parent_id {
            if let Some(parent_span) = self.spans.iter_mut().find(|s| s.id == parent) {
                parent_span.children.push(id);
            }
        }

        id
    }

    /// End a trace span.
    pub fn end_span(&mut self, span_id: u64, success: bool, result_size: usize) {
        if let Some(span) = self.spans.iter_mut().find(|s| s.id == span_id) {
            span.duration = span.start_time.elapsed();
            span.success = success;
            span.result_size = result_size;
        }

        if self.active_stack.last() == Some(&span_id) {
            self.active_stack.pop();
        }
    }

    /// Get the total duration of the session.
    pub fn total_duration(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get the maximum call depth observed.
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    /// Get root spans (spans with no parent).
    pub fn root_spans(&self) -> Vec<&TraceSpan> {
        self.spans.iter().filter(|s| s.parent_id.is_none()).collect()
    }

    /// Get children of a span.
    pub fn children_of(&self, span_id: u64) -> Vec<&TraceSpan> {
        let child_ids: Vec<u64> = self
            .spans
            .iter()
            .find(|s| s.id == span_id)
            .map(|s| s.children.clone())
            .unwrap_or_default();

        self.spans.iter().filter(|s| child_ids.contains(&s.id)).collect()
    }

    /// Generate a call tree string representation.
    pub fn call_tree(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("📊 Trace Session: {}\n", self.session_id));
        output.push_str(&format!(
            "   Duration: {:.3}ms | Spans: {} | Max Depth: {}\n\n",
            self.total_duration().as_secs_f64() * 1000.0,
            self.spans.len(),
            self.max_depth,
        ));

        for root in self.root_spans() {
            self.format_span(&mut output, root, 0);
        }

        output
    }

    /// Recursively format a span and its children.
    fn format_span(&self, output: &mut String, span: &TraceSpan, depth: usize) {
        let indent = "  ".repeat(depth);
        let status = if span.success { "✅" } else { "❌" };
        let caller = span
            .caller_module
            .as_deref()
            .unwrap_or("host");

        output.push_str(&format!(
            "{}{} {}::{}() [{} → {}] {:.3}ms ({}B → {}B)\n",
            indent,
            status,
            span.module,
            span.function,
            caller,
            span.module,
            span.duration.as_secs_f64() * 1000.0,
            span.args_size,
            span.result_size,
        ));

        for child in self.children_of(span.id) {
            self.format_span(output, child, depth + 1);
        }
    }

    /// Export as Chrome Tracing JSON format.
    pub fn to_chrome_tracing_json(&self) -> String {
        let mut events = Vec::new();

        for span in &self.spans {
            let start_us = span
                .start_time
                .duration_since(self.start_time)
                .as_micros();

            events.push(format!(
                r#"{{"name":"{}::{}","cat":"bridge","ph":"X","ts":{},"dur":{},"pid":1,"tid":{},"args":{{"module":"{}","function":"{}","success":{},"args_size":{},"result_size":{}}}}}"#,
                span.module,
                span.function,
                start_us,
                span.duration.as_micros(),
                span.parent_id.unwrap_or(0),
                span.module,
                span.function,
                span.success,
                span.args_size,
                span.result_size,
            ));
        }

        format!("[{}]", events.join(","))
    }

    /// Generate collapsed stack format (for flame graphs).
    pub fn to_collapsed_stacks(&self) -> String {
        let mut output = String::new();

        for span in &self.spans {
            let mut stack = Vec::new();

            // Build stack trace by walking up to root
            let mut current_id = Some(span.id);
            while let Some(id) = current_id {
                if let Some(s) = self.spans.iter().find(|s| s.id == id) {
                    stack.push(format!("{}::{}", s.module, s.function));
                    current_id = s.parent_id;
                } else {
                    break;
                }
            }

            stack.reverse();
            output.push_str(&format!(
                "{} {}\n",
                stack.join(";"),
                span.duration.as_micros()
            ));
        }

        output
    }

    /// Get per-module statistics.
    pub fn module_stats(&self) -> HashMap<String, ModuleStats> {
        let mut stats: HashMap<String, ModuleStats> = HashMap::new();

        for span in &self.spans {
            let entry = stats
                .entry(span.module.clone())
                .or_insert_with(|| ModuleStats {
                    module: span.module.clone(),
                    total_calls: 0,
                    total_time: Duration::ZERO,
                    min_time: Duration::from_secs(u64::MAX),
                    max_time: Duration::ZERO,
                    total_args_bytes: 0,
                    total_result_bytes: 0,
                    error_count: 0,
                });

            entry.total_calls += 1;
            entry.total_time += span.duration;
            entry.total_args_bytes += span.args_size;
            entry.total_result_bytes += span.result_size;

            if span.duration < entry.min_time {
                entry.min_time = span.duration;
            }
            if span.duration > entry.max_time {
                entry.max_time = span.duration;
            }
            if !span.success {
                entry.error_count += 1;
            }
        }

        stats
    }

    /// Print a formatted profile report.
    pub fn print_report(&self) {
        println!("{}", self.call_tree());

        let stats = self.module_stats();
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  📊 Per-Module Performance                           │");
        println!("  ├──────────────────────────────────────────────────────┤");

        for (module, stat) in &stats {
            let avg = if stat.total_calls > 0 {
                stat.total_time.as_micros() / stat.total_calls as u128
            } else {
                0
            };

            println!(
                "  │  {} — {} calls, avg {:.1}µs, total {:.3}ms",
                module,
                stat.total_calls,
                avg,
                stat.total_time.as_secs_f64() * 1000.0,
            );
            println!(
                "  │    Data: {}B args, {}B results, {} errors",
                stat.total_args_bytes, stat.total_result_bytes, stat.error_count,
            );
        }

        println!("  └──────────────────────────────────────────────────────┘");
    }
}

/// Per-module statistics from a trace session.
#[derive(Debug)]
pub struct ModuleStats {
    pub module: String,
    pub total_calls: u64,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub total_args_bytes: usize,
    pub total_result_bytes: usize,
    pub error_count: u64,
}

/// Global profiler that manages multiple trace sessions.
pub struct Profiler {
    /// Active trace session.
    active_session: Option<TraceSession>,
    /// Historical sessions.
    history: Vec<TraceSession>,
    /// Maximum number of historical sessions to keep.
    max_history: usize,
    /// Whether profiling is enabled.
    enabled: bool,
}

impl Profiler {
    /// Create a new profiler.
    pub fn new(enabled: bool) -> Self {
        info!(enabled, "Profiler initialized");
        Self {
            active_session: None,
            history: Vec::new(),
            max_history: 100,
            enabled,
        }
    }

    /// Start a new profiling session.
    pub fn start_session(&mut self, session_id: &str) {
        if !self.enabled {
            return;
        }

        // Archive previous session
        if let Some(prev) = self.active_session.take() {
            self.history.push(prev);
            if self.history.len() > self.max_history {
                self.history.remove(0);
            }
        }

        self.active_session = Some(TraceSession::new(session_id));
        info!(session = session_id, "Profiling session started");
    }

    /// Begin a span in the active session.
    pub fn begin_span(
        &mut self,
        module: &str,
        function: &str,
        caller: Option<&str>,
        args_size: usize,
    ) -> Option<u64> {
        if !self.enabled {
            return None;
        }

        self.active_session
            .as_mut()
            .map(|s| s.begin_span(module, function, caller, args_size))
    }

    /// End a span in the active session.
    pub fn end_span(&mut self, span_id: u64, success: bool, result_size: usize) {
        if let Some(session) = self.active_session.as_mut() {
            session.end_span(span_id, success, result_size);
        }
    }

    /// Stop the active session and return it.
    pub fn stop_session(&mut self) -> Option<&TraceSession> {
        if let Some(session) = self.active_session.take() {
            self.history.push(session);
            self.history.last()
        } else {
            None
        }
    }

    /// Get the active session.
    pub fn active_session(&self) -> Option<&TraceSession> {
        self.active_session.as_ref()
    }

    /// Get historical sessions.
    pub fn history(&self) -> &[TraceSession] {
        &self.history
    }

    /// Enable or disable profiling.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_trace_session() {
        let mut session = TraceSession::new("test-session");

        let span1 = session.begin_span("orchestrator", "run_pipeline", None, 32);
        let span2 = session.begin_span("calc-engine", "add", Some("orchestrator"), 8);

        thread::sleep(Duration::from_millis(1));
        session.end_span(span2, true, 4);

        let span3 = session.begin_span("calc-engine", "multiply", Some("orchestrator"), 8);
        thread::sleep(Duration::from_millis(1));
        session.end_span(span3, true, 4);

        session.end_span(span1, true, 16);

        assert_eq!(session.spans.len(), 3);
        assert_eq!(session.max_depth(), 2);
        assert_eq!(session.root_spans().len(), 1);
    }

    #[test]
    fn test_call_tree_output() {
        let mut session = TraceSession::new("test");
        let s1 = session.begin_span("host", "main", None, 0);
        let s2 = session.begin_span("calc", "add", Some("host"), 8);
        session.end_span(s2, true, 4);
        session.end_span(s1, true, 4);

        let tree = session.call_tree();
        assert!(tree.contains("host::main"));
        assert!(tree.contains("calc::add"));
    }

    #[test]
    fn test_chrome_tracing_export() {
        let mut session = TraceSession::new("test");
        let s1 = session.begin_span("calc", "add", None, 8);
        session.end_span(s1, true, 4);

        let json = session.to_chrome_tracing_json();
        assert!(json.starts_with("["));
        assert!(json.ends_with("]"));
        assert!(json.contains("calc::add"));
    }

    #[test]
    fn test_module_stats() {
        let mut session = TraceSession::new("test");

        for _ in 0..5 {
            let s = session.begin_span("calc", "add", None, 8);
            session.end_span(s, true, 4);
        }

        let s = session.begin_span("calc", "multiply", None, 8);
        session.end_span(s, false, 0);

        let stats = session.module_stats();
        let calc = stats.get("calc").unwrap();
        assert_eq!(calc.total_calls, 6);
        assert_eq!(calc.error_count, 1);
    }

    #[test]
    fn test_profiler_disabled() {
        let mut profiler = Profiler::new(false);
        profiler.start_session("test");
        let span = profiler.begin_span("calc", "add", None, 8);
        assert!(span.is_none()); // Should be None when disabled
    }
}
