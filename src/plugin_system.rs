//! # 🔌 Plugin System
//!
//! The Plugin System allows third-party extensions to be loaded into the
//! Omni-Kernel runtime at startup or dynamically at runtime. Plugins can:
//!
//! - Add new host functions available to WASM modules
//! - Register custom memory allocators
//! - Add new bridge protocols
//! - Provide language-specific optimizations
//! - Add monitoring/observability integrations
//!
//! ## Plugin Lifecycle
//!
//! ```text
//! ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
//! │ Discover │ →  │  Load    │ →  │  Init    │ →  │  Active  │
//! │ (.so/.dl)│    │ (dlopen) │    │ (setup)  │    │ (running)│
//! └──────────┘    └──────────┘    └──────────┘    └──────────┘
//!                                                      │
//!                                 ┌──────────┐    ┌────▼─────┐
//!                                 │ Unloaded │ ←  │ Shutdown │
//!                                 └──────────┘    └──────────┘
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;
use thiserror::Error;
use tracing::{debug, info, warn};

/// Plugin system errors.
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin '{0}' not found")]
    NotFound(String),

    #[error("Plugin '{0}' failed to initialize: {1}")]
    InitFailed(String, String),

    #[error("Plugin '{0}' is already loaded")]
    AlreadyLoaded(String),

    #[error("Plugin '{0}' version {found} is incompatible (requires {required})")]
    IncompatibleVersion {
        plugin: String,
        found: String,
        required: String,
    },

    #[error("Plugin '{0}' dependency '{1}' is not loaded")]
    MissingDependency(String, String),

    #[error("Plugin '{0}' failed to load: {1}")]
    LoadFailed(String, String),
}

/// Metadata describing a plugin.
#[derive(Debug, Clone)]
pub struct PluginManifest {
    /// Unique identifier for this plugin.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Semantic version.
    pub version: String,
    /// Plugin author.
    pub author: String,
    /// Description of what this plugin does.
    pub description: String,
    /// Minimum Omni-Kernel version required.
    pub min_kernel_version: String,
    /// Other plugins that must be loaded first.
    pub dependencies: Vec<String>,
    /// Plugin type.
    pub plugin_type: PluginType,
    /// Capabilities this plugin requires.
    pub required_capabilities: Vec<String>,
}

/// Type of plugin.
#[derive(Debug, Clone, PartialEq)]
pub enum PluginType {
    /// Adds new host functions.
    HostExtension,
    /// Custom memory allocator.
    Allocator,
    /// Bridge protocol extension.
    BridgeProtocol,
    /// Language-specific runtime support.
    LanguageRuntime,
    /// Monitoring/observability.
    Monitoring,
    /// Optimization pass.
    Optimizer,
    /// Transform pipeline stage.
    Transform,
}

/// Current state of a plugin.
#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    /// Plugin discovered but not loaded.
    Discovered,
    /// Plugin binary loaded into memory.
    Loaded,
    /// Plugin initialized and ready.
    Active,
    /// Plugin encountered an error.
    Error(String),
    /// Plugin is shutting down.
    ShuttingDown,
    /// Plugin has been unloaded.
    Unloaded,
}

/// A registered host function provided by a plugin.
#[derive(Debug, Clone)]
pub struct HostFunction {
    /// Function name (as visible to WASM modules).
    pub name: String,
    /// Which plugin provides this function.
    pub plugin_id: String,
    /// Module namespace for the import.
    pub namespace: String,
    /// Number of parameters.
    pub param_count: usize,
    /// Number of return values.
    pub result_count: usize,
    /// Description.
    pub description: String,
}

/// Runtime information about a loaded plugin.
#[derive(Debug)]
pub struct PluginInstance {
    /// Plugin manifest.
    pub manifest: PluginManifest,
    /// Current state.
    pub state: PluginState,
    /// When this plugin was loaded.
    pub loaded_at: Instant,
    /// Host functions registered by this plugin.
    pub host_functions: Vec<HostFunction>,
    /// Configuration specific to this plugin.
    pub config: HashMap<String, String>,
    /// Number of times this plugin's functions were called.
    pub call_count: u64,
    /// Error count.
    pub error_count: u64,
}

/// The Plugin Manager.
pub struct PluginManager {
    /// Loaded plugins.
    plugins: HashMap<String, PluginInstance>,
    /// Search paths for plugin discovery.
    search_paths: Vec<PathBuf>,
    /// All host functions registered by plugins.
    host_functions: HashMap<String, HostFunction>,
    /// Plugin load order (for dependency resolution).
    load_order: Vec<String>,
}

impl PluginManager {
    /// Create a new plugin manager.
    pub fn new(search_paths: Vec<PathBuf>) -> Self {
        info!("Plugin Manager initialized");
        Self {
            plugins: HashMap::new(),
            search_paths,
            host_functions: HashMap::new(),
            load_order: Vec::new(),
        }
    }

    /// Register a plugin from its manifest.
    pub fn register(&mut self, manifest: PluginManifest) -> Result<(), PluginError> {
        if self.plugins.contains_key(&manifest.id) {
            return Err(PluginError::AlreadyLoaded(manifest.id.clone()));
        }

        // Check dependencies
        for dep in &manifest.dependencies {
            if !self.plugins.contains_key(dep) {
                return Err(PluginError::MissingDependency(
                    manifest.id.clone(),
                    dep.clone(),
                ));
            }
        }

        let id = manifest.id.clone();
        let instance = PluginInstance {
            manifest,
            state: PluginState::Discovered,
            loaded_at: Instant::now(),
            host_functions: Vec::new(),
            config: HashMap::new(),
            call_count: 0,
            error_count: 0,
        };

        self.plugins.insert(id.clone(), instance);
        self.load_order.push(id.clone());
        debug!(plugin = &id, "Plugin registered");

        Ok(())
    }

    /// Initialize a registered plugin.
    pub fn init_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        plugin.state = PluginState::Active;
        plugin.loaded_at = Instant::now();
        info!(plugin = plugin_id, "Plugin initialized");

        Ok(())
    }

    /// Register a host function from a plugin.
    pub fn register_host_function(
        &mut self,
        plugin_id: &str,
        host_func: HostFunction,
    ) -> Result<(), PluginError> {
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        let func_name = host_func.name.clone();
        plugin.host_functions.push(host_func.clone());
        self.host_functions.insert(func_name.clone(), host_func);

        debug!(
            plugin = plugin_id,
            function = &func_name,
            "Host function registered"
        );
        Ok(())
    }

    /// Get a plugin by ID.
    pub fn get_plugin(&self, id: &str) -> Option<&PluginInstance> {
        self.plugins.get(id)
    }

    /// Get all registered host functions.
    pub fn host_functions(&self) -> &HashMap<String, HostFunction> {
        &self.host_functions
    }

    /// Shutdown a plugin.
    pub fn shutdown_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        plugin.state = PluginState::ShuttingDown;
        info!(plugin = plugin_id, "Plugin shutting down");

        // Remove host functions
        let func_names: Vec<String> = plugin.host_functions.iter().map(|f| f.name.clone()).collect();
        for name in func_names {
            self.host_functions.remove(&name);
        }

        plugin.state = PluginState::Unloaded;
        Ok(())
    }

    /// Shutdown all plugins in reverse load order.
    pub fn shutdown_all(&mut self) {
        let order: Vec<String> = self.load_order.iter().rev().cloned().collect();
        for id in order {
            if let Err(e) = self.shutdown_plugin(&id) {
                warn!(plugin = &id, error = %e, "Error shutting down plugin");
            }
        }
        info!("All plugins shut down");
    }

    /// Get the total number of loaded plugins.
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// List all active plugins.
    pub fn active_plugins(&self) -> Vec<&str> {
        self.plugins
            .iter()
            .filter(|(_, p)| p.state == PluginState::Active)
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Print plugin status.
    pub fn print_status(&self) {
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  🔌 Plugin Manager                                   │");
        println!("  ├──────────────────────────────────────────────────────┤");
        println!(
            "  │  Total Plugins: {} │ Host Functions: {}              │",
            self.plugins.len(),
            self.host_functions.len()
        );

        for (id, plugin) in &self.plugins {
            let state = match &plugin.state {
                PluginState::Active => "✅ Active",
                PluginState::Discovered => "🔍 Discovered",
                PluginState::Loaded => "📦 Loaded",
                PluginState::Error(e) => "❌ Error",
                PluginState::ShuttingDown => "⏳ Shutting Down",
                PluginState::Unloaded => "⛔ Unloaded",
            };
            println!(
                "  │  {} [{:?}] — {} v{}",
                state, plugin.manifest.plugin_type, plugin.manifest.name, plugin.manifest.version
            );
            println!(
                "  │    {} host functions, {} calls, {} errors",
                plugin.host_functions.len(),
                plugin.call_count,
                plugin.error_count,
            );
        }

        println!("  └──────────────────────────────────────────────────────┘");
    }
}

/// Builder for creating built-in plugins.
pub fn create_builtin_plugins() -> Vec<PluginManifest> {
    vec![
        PluginManifest {
            id: "omni.builtin.wasi-shim".to_string(),
            name: "WASI Compatibility Shim".to_string(),
            version: "0.1.0".to_string(),
            author: "Omni-Kernel Team".to_string(),
            description: "Provides WASI preview1 compatibility layer for guest modules".to_string(),
            min_kernel_version: "0.1.0".to_string(),
            dependencies: Vec::new(),
            plugin_type: PluginType::HostExtension,
            required_capabilities: vec!["wasi:filesystem".to_string(), "wasi:clock".to_string()],
        },
        PluginManifest {
            id: "omni.builtin.quickjs".to_string(),
            name: "QuickJS Runtime".to_string(),
            version: "0.1.0".to_string(),
            author: "Omni-Kernel Team".to_string(),
            description: "Embeds QuickJS as a WASM module for JavaScript execution".to_string(),
            min_kernel_version: "0.1.0".to_string(),
            dependencies: vec!["omni.builtin.wasi-shim".to_string()],
            plugin_type: PluginType::LanguageRuntime,
            required_capabilities: vec!["memory:alloc".to_string(), "bridge:call".to_string()],
        },
        PluginManifest {
            id: "omni.builtin.metrics-exporter".to_string(),
            name: "Prometheus Metrics Exporter".to_string(),
            version: "0.1.0".to_string(),
            author: "Omni-Kernel Team".to_string(),
            description: "Exports runtime metrics in Prometheus format".to_string(),
            min_kernel_version: "0.1.0".to_string(),
            dependencies: Vec::new(),
            plugin_type: PluginType::Monitoring,
            required_capabilities: vec!["wasi:network".to_string()],
        },
        PluginManifest {
            id: "omni.builtin.buddy-allocator".to_string(),
            name: "Buddy Memory Allocator".to_string(),
            version: "0.1.0".to_string(),
            author: "Omni-Kernel Team".to_string(),
            description: "High-performance buddy allocator for the memory pool".to_string(),
            min_kernel_version: "0.1.0".to_string(),
            dependencies: Vec::new(),
            plugin_type: PluginType::Allocator,
            required_capabilities: vec!["memory:alloc".to_string()],
        },
        PluginManifest {
            id: "omni.builtin.ir-optimizer".to_string(),
            name: "AI-Driven IR Optimizer".to_string(),
            version: "0.1.0".to_string(),
            author: "Omni-Kernel Team".to_string(),
            description: "Uses AI analysis to optimize cross-module LLVM IR boundaries".to_string(),
            min_kernel_version: "0.1.0".to_string(),
            dependencies: Vec::new(),
            plugin_type: PluginType::Optimizer,
            required_capabilities: Vec::new(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_manifest(id: &str, deps: Vec<String>) -> PluginManifest {
        PluginManifest {
            id: id.to_string(),
            name: id.to_string(),
            version: "1.0.0".to_string(),
            author: "test".to_string(),
            description: "test plugin".to_string(),
            min_kernel_version: "0.1.0".to_string(),
            dependencies: deps,
            plugin_type: PluginType::HostExtension,
            required_capabilities: Vec::new(),
        }
    }

    #[test]
    fn test_plugin_registration() {
        let mut mgr = PluginManager::new(vec![]);
        let manifest = make_test_manifest("test-plugin", vec![]);

        mgr.register(manifest).unwrap();
        assert_eq!(mgr.count(), 1);
        assert!(mgr.get_plugin("test-plugin").is_some());
    }

    #[test]
    fn test_duplicate_registration() {
        let mut mgr = PluginManager::new(vec![]);
        let manifest = make_test_manifest("test-plugin", vec![]);

        mgr.register(manifest.clone()).unwrap();
        assert!(mgr.register(manifest).is_err());
    }

    #[test]
    fn test_dependency_check() {
        let mut mgr = PluginManager::new(vec![]);

        // Try to register plugin with unmet dependency
        let manifest = make_test_manifest("child", vec!["parent".to_string()]);
        assert!(mgr.register(manifest).is_err());

        // Register parent first, then child
        let parent = make_test_manifest("parent", vec![]);
        mgr.register(parent).unwrap();

        let child = make_test_manifest("child", vec!["parent".to_string()]);
        assert!(mgr.register(child).is_ok());
    }

    #[test]
    fn test_host_function_registration() {
        let mut mgr = PluginManager::new(vec![]);
        let manifest = make_test_manifest("test-plugin", vec![]);
        mgr.register(manifest).unwrap();
        mgr.init_plugin("test-plugin").unwrap();

        let host_func = HostFunction {
            name: "custom_log".to_string(),
            plugin_id: "test-plugin".to_string(),
            namespace: "env".to_string(),
            param_count: 2,
            result_count: 0,
            description: "Custom logging function".to_string(),
        };

        mgr.register_host_function("test-plugin", host_func).unwrap();
        assert_eq!(mgr.host_functions().len(), 1);
    }

    #[test]
    fn test_shutdown_all() {
        let mut mgr = PluginManager::new(vec![]);
        mgr.register(make_test_manifest("a", vec![])).unwrap();
        mgr.init_plugin("a").unwrap();
        mgr.register(make_test_manifest("b", vec!["a".to_string()])).unwrap();
        mgr.init_plugin("b").unwrap();

        mgr.shutdown_all();

        assert_eq!(
            mgr.get_plugin("a").unwrap().state,
            PluginState::Unloaded
        );
        assert_eq!(
            mgr.get_plugin("b").unwrap().state,
            PluginState::Unloaded
        );
    }

    #[test]
    fn test_builtin_plugins() {
        let builtins = create_builtin_plugins();
        assert!(builtins.len() >= 5);

        // Verify the critical ones exist
        assert!(builtins.iter().any(|p| p.id == "omni.builtin.quickjs"));
        assert!(builtins.iter().any(|p| p.id == "omni.builtin.ir-optimizer"));
    }
}
