//! # 📋 Module Registry
//!
//! The Module Registry tracks all loaded WASM modules, their exported functions,
//! and metadata. When a bridge call targets a specific module, the registry
//! resolves the module name → instance mapping.
//!
//! ## Design
//!
//! Each module is identified by a unique string name (e.g., "calc-engine",
//! "orchestrator"). When a WASM module calls `bridge_call("calc-engine", "add", ...)`,
//! the Bridge consults the registry to find the target module's instance and
//! function reference.

use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, info};

/// Errors from the module registry.
#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Module '{0}' not found in registry")]
    ModuleNotFound(String),

    #[error("Function '{func}' not found in module '{module}'")]
    FunctionNotFound { module: String, func: String },

    #[error("Module '{0}' is already registered")]
    DuplicateModule(String),
}

/// Describes the type signature of a function exported by a WASM module.
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Name of the function.
    pub name: String,
    /// Parameter types (simplified for now — uses WASM value types).
    pub params: Vec<ValueType>,
    /// Return types.
    pub returns: Vec<ValueType>,
}

/// Simplified WASM value types for the registry.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

/// Information about a registered module.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    /// Unique name of the module.
    pub name: String,
    /// Source language this module was compiled from.
    pub source_language: SourceLanguage,
    /// List of exported functions and their signatures.
    pub exports: Vec<FunctionSignature>,
    /// Size of the module's WASM binary in bytes.
    pub binary_size: usize,
    /// Whether this module has been instantiated.
    pub instantiated: bool,
    /// Number of times functions in this module have been called via bridge.
    pub call_count: u64,
}

/// The original source language of a WASM module.
#[derive(Debug, Clone, PartialEq)]
pub enum SourceLanguage {
    Rust,
    Cpp,
    Go,
    JavaScript,
    Python,
    Unknown,
}

impl std::fmt::Display for SourceLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceLanguage::Rust => write!(f, "🦀 Rust"),
            SourceLanguage::Cpp => write!(f, "⚙️  C++"),
            SourceLanguage::Go => write!(f, "🔵 Go"),
            SourceLanguage::JavaScript => write!(f, "📜 JavaScript"),
            SourceLanguage::Python => write!(f, "🐍 Python"),
            SourceLanguage::Unknown => write!(f, "❓ Unknown"),
        }
    }
}

/// The Module Registry — central index of all loaded WASM modules.
pub struct ModuleRegistry {
    /// Map of module name → module info.
    modules: HashMap<String, ModuleInfo>,
}

impl ModuleRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        info!("Module Registry initialized");
        Self {
            modules: HashMap::new(),
        }
    }

    /// Register a new module in the registry.
    pub fn register(
        &mut self,
        name: &str,
        source_language: SourceLanguage,
        binary_size: usize,
    ) -> Result<(), RegistryError> {
        if self.modules.contains_key(name) {
            return Err(RegistryError::DuplicateModule(name.to_string()));
        }

        let info = ModuleInfo {
            name: name.to_string(),
            source_language,
            exports: Vec::new(),
            binary_size,
            instantiated: false,
            call_count: 0,
        };

        self.modules.insert(name.to_string(), info);
        debug!(module = name, "Module registered");
        Ok(())
    }

    /// Register an exported function for a module.
    pub fn register_export(
        &mut self,
        module: &str,
        func_name: &str,
        params: Vec<ValueType>,
        returns: Vec<ValueType>,
    ) -> Result<(), RegistryError> {
        let info = self
            .modules
            .get_mut(module)
            .ok_or_else(|| RegistryError::ModuleNotFound(module.to_string()))?;

        info.exports.push(FunctionSignature {
            name: func_name.to_string(),
            params,
            returns,
        });

        debug!(
            module,
            function = func_name,
            "Function export registered"
        );
        Ok(())
    }

    /// Mark a module as instantiated.
    pub fn mark_instantiated(&mut self, module: &str) -> Result<(), RegistryError> {
        let info = self
            .modules
            .get_mut(module)
            .ok_or_else(|| RegistryError::ModuleNotFound(module.to_string()))?;
        info.instantiated = true;
        Ok(())
    }

    /// Record a bridge call to a module's function.
    pub fn record_call(&mut self, module: &str) -> Result<(), RegistryError> {
        let info = self
            .modules
            .get_mut(module)
            .ok_or_else(|| RegistryError::ModuleNotFound(module.to_string()))?;
        info.call_count += 1;
        Ok(())
    }

    /// Look up a module by name.
    pub fn get(&self, name: &str) -> Result<&ModuleInfo, RegistryError> {
        self.modules
            .get(name)
            .ok_or_else(|| RegistryError::ModuleNotFound(name.to_string()))
    }

    /// Check if a module has a specific export.
    pub fn has_export(&self, module: &str, func_name: &str) -> bool {
        self.modules
            .get(module)
            .map(|info| info.exports.iter().any(|e| e.name == func_name))
            .unwrap_or(false)
    }

    /// Get the total number of registered modules.
    pub fn count(&self) -> usize {
        self.modules.len()
    }

    /// List all registered module names.
    pub fn list_modules(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }

    /// Print a formatted summary of all registered modules.
    pub fn print_summary(&self) {
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  📋 Module Registry                                  │");
        println!("  ├──────────────────────────────────────────────────────┤");

        for (name, info) in &self.modules {
            let status = if info.instantiated { "✅" } else { "⏳" };
            println!(
                "  │  {} {} [{}] — {} exports, {} calls",
                status,
                name,
                info.source_language,
                info.exports.len(),
                info.call_count,
            );
        }

        println!("  └──────────────────────────────────────────────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_lookup() {
        let mut registry = ModuleRegistry::new();
        registry
            .register("calc", SourceLanguage::Rust, 1024)
            .unwrap();

        let info = registry.get("calc").unwrap();
        assert_eq!(info.name, "calc");
        assert_eq!(info.source_language, SourceLanguage::Rust);
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = ModuleRegistry::new();
        registry
            .register("calc", SourceLanguage::Rust, 1024)
            .unwrap();
        assert!(registry
            .register("calc", SourceLanguage::Rust, 1024)
            .is_err());
    }

    #[test]
    fn test_export_registration() {
        let mut registry = ModuleRegistry::new();
        registry
            .register("calc", SourceLanguage::Rust, 1024)
            .unwrap();
        registry
            .register_export(
                "calc",
                "add",
                vec![ValueType::I32, ValueType::I32],
                vec![ValueType::I32],
            )
            .unwrap();

        assert!(registry.has_export("calc", "add"));
        assert!(!registry.has_export("calc", "subtract"));
    }

    #[test]
    fn test_call_counting() {
        let mut registry = ModuleRegistry::new();
        registry
            .register("calc", SourceLanguage::Rust, 1024)
            .unwrap();

        registry.record_call("calc").unwrap();
        registry.record_call("calc").unwrap();
        registry.record_call("calc").unwrap();

        assert_eq!(registry.get("calc").unwrap().call_count, 3);
    }
}
