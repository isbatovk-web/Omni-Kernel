//! # ⚙️ Configuration System
//!
//! Centralized configuration for the Omni-Kernel runtime. Supports loading
//! from TOML files, environment variables, and CLI arguments with sensible
//! defaults for every setting.
//!
//! ## Configuration Hierarchy (highest priority first)
//!
//! 1. CLI arguments
//! 2. Environment variables (OMNI_*)
//! 3. Config file (omni-kernel.toml)
//! 4. Built-in defaults

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

/// Top-level configuration for the Omni-Kernel runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelConfig {
    /// General runtime settings.
    pub runtime: RuntimeConfig,
    /// Memory pool configuration.
    pub memory: MemoryConfig,
    /// Bridge configuration.
    pub bridge: BridgeConfig,
    /// Security and sandboxing settings.
    pub security: SecurityConfig,
    /// Logging and diagnostics.
    pub logging: LoggingConfig,
    /// Module loading settings.
    pub modules: ModuleLoadConfig,
    /// Performance tuning.
    pub performance: PerformanceConfig,
}

/// Runtime engine configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Name of this runtime instance (for multi-instance setups).
    pub instance_name: String,
    /// Maximum number of WASM modules that can be loaded simultaneously.
    pub max_modules: usize,
    /// Enable the WASM Component Model (WIT interfaces).
    pub enable_component_model: bool,
    /// Enable WASI (WebAssembly System Interface) for guest modules.
    pub enable_wasi: bool,
    /// Working directory for the runtime.
    pub work_dir: PathBuf,
    /// Enable async/parallel module execution.
    pub enable_async: bool,
    /// Maximum execution time per function call (milliseconds).
    pub max_execution_time_ms: u64,
    /// Enable fuel-based execution metering.
    pub enable_fuel_metering: bool,
    /// Default fuel limit per function call.
    pub default_fuel_limit: u64,
}

/// Memory pool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Total size of the Universal Memory Pool in bytes.
    pub pool_size: usize,
    /// Maximum size of a single memory block allocation.
    pub max_block_size: usize,
    /// Minimum alignment for memory allocations (must be power of 2).
    pub alignment: usize,
    /// Enable reference counting for shared blocks.
    pub enable_ref_counting: bool,
    /// Enable memory access logging (impacts performance).
    pub enable_access_logging: bool,
    /// Strategy for memory allocation.
    pub allocator_strategy: AllocatorStrategy,
    /// Maximum number of active memory blocks.
    pub max_active_blocks: usize,
    /// Enable memory defragmentation.
    pub enable_defrag: bool,
    /// Defragmentation threshold (percentage of fragmented space).
    pub defrag_threshold: f32,
}

/// Memory allocator strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocatorStrategy {
    /// Simple bump allocator — fast but no defrag.
    Bump,
    /// Buddy allocator — balanced speed and fragmentation.
    Buddy,
    /// Slab allocator — optimized for fixed-size allocations.
    Slab,
    /// First-fit free list — general purpose.
    FirstFit,
}

/// Bridge configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Maximum depth of nested bridge calls (A → B → C → ...).
    pub max_call_depth: usize,
    /// Timeout for individual bridge calls (milliseconds).
    pub call_timeout_ms: u64,
    /// Enable call tracing (records all bridge calls for debugging).
    pub enable_call_tracing: bool,
    /// Maximum size of arguments passed via bridge calls.
    pub max_args_size: usize,
    /// Enable async bridge calls.
    pub enable_async_calls: bool,
    /// Maximum number of concurrent async bridge calls.
    pub max_concurrent_calls: usize,
    /// Enable call result caching.
    pub enable_result_cache: bool,
    /// Maximum number of cached results.
    pub result_cache_size: usize,
}

/// Security and sandboxing configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable WASM module sandboxing (restricts host access).
    pub enable_sandbox: bool,
    /// List of allowed host functions for sandboxed modules.
    pub allowed_host_functions: Vec<String>,
    /// Maximum memory a single module can allocate (bytes).
    pub max_module_memory: usize,
    /// Enable module signature verification.
    pub enable_signature_verification: bool,
    /// Path to trusted module certificates.
    pub trusted_certs_dir: Option<PathBuf>,
    /// Enable network access for modules.
    pub allow_network: bool,
    /// Enable file system access for modules.
    pub allow_filesystem: bool,
    /// Allowed file system paths (if filesystem access is enabled).
    pub allowed_fs_paths: Vec<PathBuf>,
    /// Enable inter-module communication restrictions.
    pub enable_imc_restrictions: bool,
    /// Allowed module-to-module communication pairs.
    pub allowed_imc_pairs: Vec<(String, String)>,
}

/// Logging and diagnostics configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level: trace, debug, info, warn, error.
    pub level: String,
    /// Enable structured JSON logging.
    pub json_output: bool,
    /// Log output file (None for stdout).
    pub log_file: Option<PathBuf>,
    /// Enable performance metrics collection.
    pub enable_metrics: bool,
    /// Metrics export interval (seconds).
    pub metrics_interval_secs: u64,
    /// Enable execution tracing (OpenTelemetry-compatible).
    pub enable_tracing: bool,
}

/// Module loading configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleLoadConfig {
    /// Directories to search for WASM modules.
    pub search_paths: Vec<PathBuf>,
    /// Enable automatic module discovery in search paths.
    pub auto_discover: bool,
    /// Enable hot-reloading of modules when files change.
    pub enable_hot_reload: bool,
    /// File watch debounce interval (milliseconds).
    pub hot_reload_debounce_ms: u64,
    /// Pre-compile modules on load (improves first-call latency).
    pub precompile: bool,
    /// Cache compiled modules to disk.
    pub enable_compile_cache: bool,
    /// Path for compiled module cache.
    pub compile_cache_dir: Option<PathBuf>,
}

/// Performance tuning configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable Cranelift JIT optimizations.
    pub enable_jit_optimization: bool,
    /// JIT optimization level: none, speed, speed_and_size.
    pub jit_opt_level: JitOptLevel,
    /// Enable parallel module compilation.
    pub parallel_compilation: bool,
    /// Number of compilation threads (0 = auto-detect).
    pub compilation_threads: usize,
    /// Enable instance pooling for reduced instantiation overhead.
    pub enable_instance_pooling: bool,
    /// Maximum number of pooled instances per module.
    pub instance_pool_size: usize,
    /// Enable copy-on-write memory for WASM instances.
    pub enable_cow_memory: bool,
    /// Enable epoch-based interruption (for cooperative scheduling).
    pub enable_epoch_interruption: bool,
    /// Epoch tick interval (microseconds).
    pub epoch_tick_us: u64,
}

/// JIT optimization level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JitOptLevel {
    /// No optimization — fastest compilation.
    None,
    /// Optimize for execution speed.
    Speed,
    /// Balance between speed and code size.
    SpeedAndSize,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            runtime: RuntimeConfig::default(),
            memory: MemoryConfig::default(),
            bridge: BridgeConfig::default(),
            security: SecurityConfig::default(),
            logging: LoggingConfig::default(),
            modules: ModuleLoadConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            instance_name: "omni-kernel-primary".to_string(),
            max_modules: 256,
            enable_component_model: true,
            enable_wasi: true,
            work_dir: PathBuf::from("."),
            enable_async: true,
            max_execution_time_ms: 30_000,
            enable_fuel_metering: false,
            default_fuel_limit: 1_000_000,
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            pool_size: 64 * 1024 * 1024, // 64 MB
            max_block_size: 16 * 1024 * 1024, // 16 MB
            alignment: 8,
            enable_ref_counting: true,
            enable_access_logging: false,
            allocator_strategy: AllocatorStrategy::Bump,
            max_active_blocks: 65536,
            enable_defrag: false,
            defrag_threshold: 0.5,
        }
    }
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            max_call_depth: 64,
            call_timeout_ms: 10_000,
            enable_call_tracing: false,
            max_args_size: 4 * 1024 * 1024, // 4 MB
            enable_async_calls: false,
            max_concurrent_calls: 32,
            enable_result_cache: false,
            result_cache_size: 1024,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_sandbox: true,
            allowed_host_functions: vec![
                "bridge_alloc".to_string(),
                "bridge_free".to_string(),
                "bridge_write".to_string(),
                "bridge_read".to_string(),
                "bridge_call".to_string(),
                "bridge_log".to_string(),
            ],
            max_module_memory: 256 * 1024 * 1024, // 256 MB per module
            enable_signature_verification: false,
            trusted_certs_dir: None,
            allow_network: false,
            allow_filesystem: false,
            allowed_fs_paths: Vec::new(),
            enable_imc_restrictions: false,
            allowed_imc_pairs: Vec::new(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            json_output: false,
            log_file: None,
            enable_metrics: true,
            metrics_interval_secs: 30,
            enable_tracing: false,
        }
    }
}

impl Default for ModuleLoadConfig {
    fn default() -> Self {
        Self {
            search_paths: vec![PathBuf::from("./modules"), PathBuf::from("./guests")],
            auto_discover: true,
            enable_hot_reload: false,
            hot_reload_debounce_ms: 500,
            precompile: true,
            enable_compile_cache: true,
            compile_cache_dir: Some(PathBuf::from("./.omni-cache")),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_jit_optimization: true,
            jit_opt_level: JitOptLevel::Speed,
            parallel_compilation: true,
            compilation_threads: 0, // auto-detect
            enable_instance_pooling: true,
            instance_pool_size: 16,
            enable_cow_memory: true,
            enable_epoch_interruption: false,
            epoch_tick_us: 1000,
        }
    }
}

impl KernelConfig {
    /// Load configuration from a TOML file, falling back to defaults.
    pub fn load(path: Option<&str>) -> Self {
        if let Some(config_path) = path {
            match std::fs::read_to_string(config_path) {
                Ok(content) => {
                    match toml::from_str::<KernelConfig>(&content) {
                        Ok(config) => {
                            info!(path = config_path, "Configuration loaded from file");
                            return config;
                        }
                        Err(e) => {
                            tracing::warn!(
                                error = %e,
                                "Failed to parse config file, using defaults"
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "Failed to read config file, using defaults"
                    );
                }
            }
        }

        info!("Using default configuration");
        Self::default()
    }

    /// Apply environment variable overrides.
    /// Environment variables follow the pattern: OMNI_SECTION_KEY
    pub fn apply_env_overrides(&mut self) {
        // Memory pool size
        if let Ok(val) = std::env::var("OMNI_MEMORY_POOL_SIZE") {
            if let Ok(size) = val.parse::<usize>() {
                self.memory.pool_size = size;
                info!(size, "Memory pool size overridden by env var");
            }
        }

        // Log level
        if let Ok(val) = std::env::var("OMNI_LOG_LEVEL") {
            self.logging.level = val;
        }

        // Max modules
        if let Ok(val) = std::env::var("OMNI_MAX_MODULES") {
            if let Ok(max) = val.parse::<usize>() {
                self.runtime.max_modules = max;
            }
        }

        // Instance name
        if let Ok(val) = std::env::var("OMNI_INSTANCE_NAME") {
            self.runtime.instance_name = val;
        }

        // Bridge call timeout
        if let Ok(val) = std::env::var("OMNI_BRIDGE_TIMEOUT_MS") {
            if let Ok(timeout) = val.parse::<u64>() {
                self.bridge.call_timeout_ms = timeout;
            }
        }
    }

    /// Validate the configuration for logical consistency.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Memory validation
        if self.memory.pool_size == 0 {
            errors.push("Memory pool size must be > 0".to_string());
        }
        if self.memory.max_block_size > self.memory.pool_size {
            errors.push("Max block size cannot exceed pool size".to_string());
        }
        if !self.memory.alignment.is_power_of_two() {
            errors.push("Memory alignment must be a power of 2".to_string());
        }

        // Bridge validation
        if self.bridge.max_call_depth == 0 {
            errors.push("Max bridge call depth must be > 0".to_string());
        }

        // Runtime validation
        if self.runtime.max_modules == 0 {
            errors.push("Max modules must be > 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Export configuration as a TOML string.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = KernelConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config() {
        let mut config = KernelConfig::default();
        config.memory.pool_size = 0;
        config.memory.alignment = 3; // not power of 2

        let errors = config.validate().unwrap_err();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_toml_serialization() {
        let config = KernelConfig::default();
        let toml_str = config.to_toml().unwrap();
        assert!(toml_str.contains("instance_name"));
        assert!(toml_str.contains("pool_size"));
    }
}
