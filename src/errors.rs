//! # ❌ Error Types
//!
//! Comprehensive error types for the Omni-Kernel runtime.
//! All subsystems report errors through this unified hierarchy.

use thiserror::Error;

/// Top-level Omni-Kernel error.
#[derive(Error, Debug)]
pub enum OmniError {
    /// Error from the WASM engine (Wasmtime).
    #[error("WASM Engine error: {0}")]
    Engine(#[from] EngineError),

    /// Error from the memory pool.
    #[error("Memory Pool error: {0}")]
    Memory(#[from] crate::memory_pool::PoolError),

    /// Error from the module registry.
    #[error("Registry error: {0}")]
    Registry(#[from] crate::module_registry::RegistryError),

    /// Error from the security layer.
    #[error("Security error: {0}")]
    Security(#[from] crate::security::SecurityError),

    /// Error from the type system.
    #[error("Type error: {0}")]
    Type(#[from] crate::type_system::TypeError),

    /// Error from the module loader.
    #[error("Module load error: {0}")]
    ModuleLoad(String),

    /// Error from the bridge.
    #[error("Bridge error: {0}")]
    Bridge(String),

    /// Error from the scheduler.
    #[error("Scheduler error: {0}")]
    Scheduler(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

/// WASM engine-specific errors.
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Failed to create engine: {0}")]
    CreationFailed(String),

    #[error("Failed to compile module '{module}': {reason}")]
    CompilationFailed { module: String, reason: String },

    #[error("Failed to instantiate module '{module}': {reason}")]
    InstantiationFailed { module: String, reason: String },

    #[error("Failed to link host function '{function}': {reason}")]
    LinkingFailed { function: String, reason: String },

    #[error("Function '{function}' not found in module '{module}'")]
    FunctionNotFound { module: String, function: String },

    #[error("Execution trapped in module '{module}': {trap}")]
    ExecutionTrapped { module: String, trap: String },

    #[error("Fuel exhausted in module '{module}' — execution limit reached")]
    FuelExhausted { module: String },

    #[error("Epoch interruption in module '{module}'")]
    EpochInterruption { module: String },
}

/// Result type alias for Omni-Kernel operations.
pub type OmniResult<T> = Result<T, OmniError>;

/// Extension trait for adding context to errors.
pub trait OmniErrorContext<T> {
    /// Add context about which module was involved.
    fn with_module(self, module: &str) -> Result<T, OmniError>;

    /// Add context about which bridge call was involved.
    fn with_bridge_context(self, caller: &str, callee: &str, function: &str)
        -> Result<T, OmniError>;
}

impl<T, E: Into<OmniError>> OmniErrorContext<T> for Result<T, E> {
    fn with_module(self, module: &str) -> Result<T, OmniError> {
        self.map_err(|e| {
            let base = e.into();
            OmniError::Internal(format!("[module:{}] {}", module, base))
        })
    }

    fn with_bridge_context(
        self,
        caller: &str,
        callee: &str,
        function: &str,
    ) -> Result<T, OmniError> {
        self.map_err(|e| {
            let base = e.into();
            OmniError::Bridge(format!(
                "Bridge call {}→{}::{} failed: {}",
                caller, callee, function, base
            ))
        })
    }
}
