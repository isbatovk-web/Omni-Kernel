//! # 📊 Metrics & Telemetry Engine
//!
//! Real-time performance monitoring for the Omni-Kernel runtime.
//! Tracks memory usage, bridge call latency, module performance,
//! and system-wide health indicators.
//!
//! ## Collected Metrics
//!
//! - **Memory**: allocations, frees, fragmentation, peak usage
//! - **Bridge**: call count, latency (min/max/avg/p99), error rate
//! - **Modules**: load time, call count, execution time per function
//! - **System**: uptime, CPU usage, total throughput

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tracing::info;

/// Global metrics collector for the Omni-Kernel runtime.
pub struct MetricsEngine {
    /// When the runtime was started.
    start_time: Instant,
    /// Memory-related metrics.
    pub memory: MemoryMetrics,
    /// Bridge call metrics.
    pub bridge: BridgeMetrics,
    /// Per-module metrics.
    pub modules: HashMap<String, ModuleMetrics>,
    /// System-wide metrics.
    pub system: SystemMetrics,
}

/// Memory subsystem metrics.
pub struct MemoryMetrics {
    /// Total number of allocations (lifetime).
    pub total_allocations: AtomicU64,
    /// Total number of frees (lifetime).
    pub total_frees: AtomicU64,
    /// Current number of active allocations.
    pub active_allocations: AtomicU64,
    /// Total bytes allocated (lifetime).
    pub total_bytes_allocated: AtomicU64,
    /// Peak bytes in use at any point.
    pub peak_bytes_in_use: AtomicU64,
    /// Current bytes in use.
    pub current_bytes_in_use: AtomicU64,
    /// Number of out-of-memory errors.
    pub oom_errors: AtomicU64,
    /// Number of bounds-check violations.
    pub bounds_violations: AtomicU64,
}

/// Bridge call metrics.
pub struct BridgeMetrics {
    /// Total number of bridge calls.
    pub total_calls: AtomicU64,
    /// Number of successful calls.
    pub successful_calls: AtomicU64,
    /// Number of failed calls.
    pub failed_calls: AtomicU64,
    /// Total call latency in microseconds (for average calculation).
    pub total_latency_us: AtomicU64,
    /// Minimum call latency in microseconds.
    pub min_latency_us: AtomicU64,
    /// Maximum call latency in microseconds.
    pub max_latency_us: AtomicU64,
    /// Call latency histogram buckets (microseconds).
    pub latency_histogram: LatencyHistogram,
    /// Number of timeout errors.
    pub timeout_errors: AtomicU64,
    /// Current call depth (for recursive call tracking).
    pub current_call_depth: AtomicU64,
    /// Maximum call depth observed.
    pub max_call_depth_observed: AtomicU64,
}

/// Per-module performance metrics.
#[derive(Debug)]
pub struct ModuleMetrics {
    /// Module name.
    pub name: String,
    /// Time to load/compile the module.
    pub load_time_ms: f64,
    /// Total number of function calls.
    pub total_calls: u64,
    /// Total execution time in microseconds.
    pub total_execution_us: u64,
    /// Per-function call statistics.
    pub function_stats: HashMap<String, FunctionStats>,
    /// Time the module was loaded.
    pub loaded_at: Instant,
    /// Number of memory blocks owned by this module.
    pub owned_blocks: u64,
    /// Total bytes owned by this module.
    pub owned_bytes: u64,
}

/// Per-function performance statistics.
#[derive(Debug, Clone)]
pub struct FunctionStats {
    /// Function name.
    pub name: String,
    /// Number of invocations.
    pub call_count: u64,
    /// Total execution time (microseconds).
    pub total_time_us: u64,
    /// Minimum execution time (microseconds).
    pub min_time_us: u64,
    /// Maximum execution time (microseconds).
    pub max_time_us: u64,
    /// Number of errors.
    pub error_count: u64,
}

/// System-wide metrics.
pub struct SystemMetrics {
    /// Total number of modules loaded (lifetime).
    pub modules_loaded: AtomicU64,
    /// Total number of modules unloaded (lifetime).
    pub modules_unloaded: AtomicU64,
    /// Total data throughput through the bridge (bytes).
    pub total_bridge_throughput: AtomicU64,
    /// Number of hot-reload events.
    pub hot_reloads: AtomicU64,
    /// Number of security policy violations.
    pub security_violations: AtomicU64,
}

/// Latency histogram with configurable buckets.
pub struct LatencyHistogram {
    /// Bucket boundaries in microseconds.
    pub boundaries: Vec<u64>,
    /// Count per bucket.
    pub counts: Vec<AtomicU64>,
}

impl LatencyHistogram {
    /// Create a new histogram with default buckets.
    /// Buckets: 1µs, 5µs, 10µs, 50µs, 100µs, 500µs, 1ms, 5ms, 10ms, 50ms, 100ms
    pub fn new() -> Self {
        let boundaries = vec![1, 5, 10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000];
        let counts = boundaries.iter().map(|_| AtomicU64::new(0)).collect();
        Self {
            boundaries,
            counts,
        }
    }

    /// Record a latency value.
    pub fn record(&self, latency_us: u64) {
        for (i, boundary) in self.boundaries.iter().enumerate() {
            if latency_us <= *boundary {
                self.counts[i].fetch_add(1, Ordering::Relaxed);
                return;
            }
        }
        // Above all boundaries — goes in the last bucket
        if let Some(last) = self.counts.last() {
            last.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Get the approximate p99 latency in microseconds.
    pub fn p99_latency(&self) -> u64 {
        let total: u64 = self.counts.iter().map(|c| c.load(Ordering::Relaxed)).sum();
        if total == 0 {
            return 0;
        }

        let threshold = (total as f64 * 0.99) as u64;
        let mut cumulative = 0u64;

        for (i, count) in self.counts.iter().enumerate() {
            cumulative += count.load(Ordering::Relaxed);
            if cumulative >= threshold {
                return self.boundaries[i];
            }
        }

        *self.boundaries.last().unwrap_or(&0)
    }
}

impl MetricsEngine {
    /// Create a new metrics engine.
    pub fn new() -> Self {
        info!("Metrics engine initialized");
        Self {
            start_time: Instant::now(),
            memory: MemoryMetrics::new(),
            bridge: BridgeMetrics::new(),
            modules: HashMap::new(),
            system: SystemMetrics::new(),
        }
    }

    /// Get runtime uptime.
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Record a memory allocation event.
    pub fn record_alloc(&self, size: usize) {
        self.memory
            .total_allocations
            .fetch_add(1, Ordering::Relaxed);
        self.memory
            .active_allocations
            .fetch_add(1, Ordering::Relaxed);
        self.memory
            .total_bytes_allocated
            .fetch_add(size as u64, Ordering::Relaxed);
        self.memory
            .current_bytes_in_use
            .fetch_add(size as u64, Ordering::Relaxed);

        // Update peak
        let current = self.memory.current_bytes_in_use.load(Ordering::Relaxed);
        let mut peak = self.memory.peak_bytes_in_use.load(Ordering::Relaxed);
        while current > peak {
            match self.memory.peak_bytes_in_use.compare_exchange_weak(
                peak,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => peak = actual,
            }
        }
    }

    /// Record a memory free event.
    pub fn record_free(&self, size: usize) {
        self.memory.total_frees.fetch_add(1, Ordering::Relaxed);
        self.memory
            .active_allocations
            .fetch_sub(1, Ordering::Relaxed);
        self.memory
            .current_bytes_in_use
            .fetch_sub(size as u64, Ordering::Relaxed);
    }

    /// Record a bridge call with its latency.
    pub fn record_bridge_call(&self, latency: Duration, success: bool) {
        self.bridge.total_calls.fetch_add(1, Ordering::Relaxed);

        let latency_us = latency.as_micros() as u64;
        self.bridge
            .total_latency_us
            .fetch_add(latency_us, Ordering::Relaxed);

        if success {
            self.bridge
                .successful_calls
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.bridge.failed_calls.fetch_add(1, Ordering::Relaxed);
        }

        // Update min latency
        let mut min = self.bridge.min_latency_us.load(Ordering::Relaxed);
        while latency_us < min {
            match self.bridge.min_latency_us.compare_exchange_weak(
                min,
                latency_us,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => min = actual,
            }
        }

        // Update max latency
        let mut max = self.bridge.max_latency_us.load(Ordering::Relaxed);
        while latency_us > max {
            match self.bridge.max_latency_us.compare_exchange_weak(
                max,
                latency_us,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => max = actual,
            }
        }

        // Record in histogram
        self.bridge.latency_histogram.record(latency_us);
    }

    /// Record a function call for a specific module.
    pub fn record_function_call(
        &mut self,
        module: &str,
        function: &str,
        duration: Duration,
        success: bool,
    ) {
        let module_metrics = self
            .modules
            .entry(module.to_string())
            .or_insert_with(|| ModuleMetrics::new(module));

        module_metrics.total_calls += 1;
        module_metrics.total_execution_us += duration.as_micros() as u64;

        let func_stats = module_metrics
            .function_stats
            .entry(function.to_string())
            .or_insert_with(|| FunctionStats::new(function));

        func_stats.call_count += 1;
        let time_us = duration.as_micros() as u64;
        func_stats.total_time_us += time_us;

        if time_us < func_stats.min_time_us {
            func_stats.min_time_us = time_us;
        }
        if time_us > func_stats.max_time_us {
            func_stats.max_time_us = time_us;
        }
        if !success {
            func_stats.error_count += 1;
        }
    }

    /// Generate a comprehensive metrics report.
    pub fn generate_report(&self) -> MetricsReport {
        let total_calls = self.bridge.total_calls.load(Ordering::Relaxed);
        let total_latency = self.bridge.total_latency_us.load(Ordering::Relaxed);

        MetricsReport {
            uptime: self.uptime(),
            memory: MemoryReport {
                total_allocations: self.memory.total_allocations.load(Ordering::Relaxed),
                total_frees: self.memory.total_frees.load(Ordering::Relaxed),
                active_allocations: self.memory.active_allocations.load(Ordering::Relaxed),
                current_bytes_in_use: self.memory.current_bytes_in_use.load(Ordering::Relaxed),
                peak_bytes_in_use: self.memory.peak_bytes_in_use.load(Ordering::Relaxed),
                oom_errors: self.memory.oom_errors.load(Ordering::Relaxed),
            },
            bridge: BridgeReport {
                total_calls,
                successful_calls: self.bridge.successful_calls.load(Ordering::Relaxed),
                failed_calls: self.bridge.failed_calls.load(Ordering::Relaxed),
                avg_latency_us: if total_calls > 0 {
                    total_latency / total_calls
                } else {
                    0
                },
                min_latency_us: self.bridge.min_latency_us.load(Ordering::Relaxed),
                max_latency_us: self.bridge.max_latency_us.load(Ordering::Relaxed),
                p99_latency_us: self.bridge.latency_histogram.p99_latency(),
            },
            modules_loaded: self.system.modules_loaded.load(Ordering::Relaxed),
            total_throughput: self
                .system
                .total_bridge_throughput
                .load(Ordering::Relaxed),
        }
    }

    /// Print a formatted metrics dashboard to stdout.
    pub fn print_dashboard(&self) {
        let report = self.generate_report();
        let uptime = report.uptime;

        println!();
        println!("  ╔══════════════════════════════════════════════════════════╗");
        println!("  ║  📊 OMNI-KERNEL METRICS DASHBOARD                       ║");
        println!("  ╠══════════════════════════════════════════════════════════╣");
        println!(
            "  ║  ⏱️  Uptime: {:.1}s                                       ║",
            uptime.as_secs_f64()
        );
        println!("  ╠══════════════════════════════════════════════════════════╣");
        println!("  ║  🧠 MEMORY POOL                                         ║");
        println!("  ║  ─────────────────────────────────────                   ║");
        println!(
            "  ║  Allocations: {} total, {} active                    ║",
            report.memory.total_allocations, report.memory.active_allocations
        );
        println!(
            "  ║  Usage: {} bytes / Peak: {} bytes              ║",
            report.memory.current_bytes_in_use, report.memory.peak_bytes_in_use
        );
        println!(
            "  ║  OOM Errors: {}                                        ║",
            report.memory.oom_errors
        );
        println!("  ╠══════════════════════════════════════════════════════════╣");
        println!("  ║  🌉 BRIDGE CALLS                                        ║");
        println!("  ║  ─────────────────────────────────────                   ║");
        println!(
            "  ║  Total: {} (✅ {} / ❌ {})                          ║",
            report.bridge.total_calls, report.bridge.successful_calls, report.bridge.failed_calls
        );
        println!(
            "  ║  Latency: avg={}µs min={}µs max={}µs p99={}µs     ║",
            report.bridge.avg_latency_us,
            report.bridge.min_latency_us,
            report.bridge.max_latency_us,
            report.bridge.p99_latency_us
        );
        println!("  ╠══════════════════════════════════════════════════════════╣");
        println!(
            "  ║  Modules Loaded: {}                                    ║",
            report.modules_loaded
        );
        println!(
            "  ║  Total Throughput: {} bytes                        ║",
            report.total_throughput
        );
        println!("  ╚══════════════════════════════════════════════════════════╝");
        println!();
    }
}

/// Snapshot of all metrics for reporting.
#[derive(Debug, serde::Serialize)]
pub struct MetricsReport {
    pub uptime: Duration,
    pub memory: MemoryReport,
    pub bridge: BridgeReport,
    pub modules_loaded: u64,
    pub total_throughput: u64,
}

/// Memory metrics snapshot.
#[derive(Debug, serde::Serialize)]
pub struct MemoryReport {
    pub total_allocations: u64,
    pub total_frees: u64,
    pub active_allocations: u64,
    pub current_bytes_in_use: u64,
    pub peak_bytes_in_use: u64,
    pub oom_errors: u64,
}

/// Bridge metrics snapshot.
#[derive(Debug, serde::Serialize)]
pub struct BridgeReport {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub avg_latency_us: u64,
    pub min_latency_us: u64,
    pub max_latency_us: u64,
    pub p99_latency_us: u64,
}

// ── Constructors ──────────────────────────────────────────────

impl MemoryMetrics {
    pub fn new() -> Self {
        Self {
            total_allocations: AtomicU64::new(0),
            total_frees: AtomicU64::new(0),
            active_allocations: AtomicU64::new(0),
            total_bytes_allocated: AtomicU64::new(0),
            peak_bytes_in_use: AtomicU64::new(0),
            current_bytes_in_use: AtomicU64::new(0),
            oom_errors: AtomicU64::new(0),
            bounds_violations: AtomicU64::new(0),
        }
    }
}

impl BridgeMetrics {
    pub fn new() -> Self {
        Self {
            total_calls: AtomicU64::new(0),
            successful_calls: AtomicU64::new(0),
            failed_calls: AtomicU64::new(0),
            total_latency_us: AtomicU64::new(0),
            min_latency_us: AtomicU64::new(u64::MAX),
            max_latency_us: AtomicU64::new(0),
            latency_histogram: LatencyHistogram::new(),
            timeout_errors: AtomicU64::new(0),
            current_call_depth: AtomicU64::new(0),
            max_call_depth_observed: AtomicU64::new(0),
        }
    }
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self {
            modules_loaded: AtomicU64::new(0),
            modules_unloaded: AtomicU64::new(0),
            total_bridge_throughput: AtomicU64::new(0),
            hot_reloads: AtomicU64::new(0),
            security_violations: AtomicU64::new(0),
        }
    }
}

impl ModuleMetrics {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            load_time_ms: 0.0,
            total_calls: 0,
            total_execution_us: 0,
            function_stats: HashMap::new(),
            loaded_at: Instant::now(),
            owned_blocks: 0,
            owned_bytes: 0,
        }
    }
}

impl FunctionStats {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            call_count: 0,
            total_time_us: 0,
            min_time_us: u64::MAX,
            max_time_us: 0,
            error_count: 0,
        }
    }

    /// Get average execution time in microseconds.
    pub fn avg_time_us(&self) -> u64 {
        if self.call_count > 0 {
            self.total_time_us / self.call_count
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_recording() {
        let metrics = MetricsEngine::new();

        metrics.record_alloc(1024);
        metrics.record_alloc(2048);
        metrics.record_free(1024);

        assert_eq!(
            metrics.memory.total_allocations.load(Ordering::Relaxed),
            2
        );
        assert_eq!(
            metrics.memory.active_allocations.load(Ordering::Relaxed),
            1
        );
        assert_eq!(
            metrics
                .memory
                .current_bytes_in_use
                .load(Ordering::Relaxed),
            2048
        );
        assert_eq!(
            metrics.memory.peak_bytes_in_use.load(Ordering::Relaxed),
            3072
        );
    }

    #[test]
    fn test_bridge_latency() {
        let metrics = MetricsEngine::new();

        metrics.record_bridge_call(Duration::from_micros(100), true);
        metrics.record_bridge_call(Duration::from_micros(200), true);
        metrics.record_bridge_call(Duration::from_micros(50), false);

        assert_eq!(metrics.bridge.total_calls.load(Ordering::Relaxed), 3);
        assert_eq!(
            metrics.bridge.successful_calls.load(Ordering::Relaxed),
            2
        );
        assert_eq!(metrics.bridge.failed_calls.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.bridge.min_latency_us.load(Ordering::Relaxed), 50);
        assert_eq!(metrics.bridge.max_latency_us.load(Ordering::Relaxed), 200);
    }

    #[test]
    fn test_function_stats() {
        let mut metrics = MetricsEngine::new();

        metrics.record_function_call("calc", "add", Duration::from_micros(10), true);
        metrics.record_function_call("calc", "add", Duration::from_micros(20), true);
        metrics.record_function_call("calc", "multiply", Duration::from_micros(15), true);

        let calc = metrics.modules.get("calc").unwrap();
        assert_eq!(calc.total_calls, 3);

        let add_stats = calc.function_stats.get("add").unwrap();
        assert_eq!(add_stats.call_count, 2);
        assert_eq!(add_stats.avg_time_us(), 15);
    }

    #[test]
    fn test_histogram() {
        let histogram = LatencyHistogram::new();

        for _ in 0..99 {
            histogram.record(10); // 99 calls at 10µs
        }
        histogram.record(90000); // 1 call at 90ms

        // p99 should be in the 10µs bucket
        assert!(histogram.p99_latency() <= 10);
    }
}
