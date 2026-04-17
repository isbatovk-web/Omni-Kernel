//! # 🌐 IPC Protocol — Inter-Process Communication
//!
//! While Omni-Kernel modules run in a single process, external clients
//! (IDEs, debuggers, CLI tools, dashboards) need to communicate with the
//! runtime. This module defines a JSON-RPC based IPC protocol.
//!
//! ## Protocol Overview
//!
//! ```text
//! ┌────────────┐   JSON-RPC over TCP   ┌──────────────────┐
//! │   Client   │ ◄─────────────────── │   Omni-Kernel    │
//! │  (CLI/IDE) │ ──────────────────→  │   IPC Server     │
//! └────────────┘                       └──────────────────┘
//! ```
//!
//! ## Supported Methods
//!
//! - `kernel.status` — Get runtime status
//! - `kernel.config` — Get/set configuration
//! - `module.list` — List loaded modules
//! - `module.load` — Load a new module
//! - `module.unload` — Unload a module
//! - `bridge.call` — Execute a cross-module call
//! - `bridge.stats` — Get bridge call statistics
//! - `memory.stats` — Get memory pool statistics
//! - `memory.dump` — Dump a memory region
//! - `security.status` — Get security status
//! - `profiler.start` — Start profiling
//! - `profiler.stop` — Stop and get profiling results
//! - `snapshot.create` — Create a runtime snapshot
//! - `snapshot.restore` — Restore from a snapshot

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JSON-RPC protocol version.
pub const PROTOCOL_VERSION: &str = "2.0";

/// Default IPC server port.
pub const DEFAULT_PORT: u16 = 9742; // "OMNI" on phone keypad

/// A JSON-RPC request.
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    /// Protocol version (always "2.0").
    pub jsonrpc: String,
    /// Method name.
    pub method: String,
    /// Optional parameters.
    pub params: Option<serde_json::Value>,
    /// Request ID (for correlating responses).
    pub id: u64,
}

/// A JSON-RPC response.
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    /// Protocol version (always "2.0").
    pub jsonrpc: String,
    /// Result (present on success).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error (present on failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
    /// Request ID this response corresponds to.
    pub id: u64,
}

/// A JSON-RPC error.
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    /// Error code.
    pub code: i32,
    /// Error message.
    pub message: String,
    /// Additional error data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Standard JSON-RPC error codes.
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;

    // Custom Omni-Kernel error codes
    pub const MODULE_NOT_FOUND: i32 = -40001;
    pub const MODULE_LOAD_FAILED: i32 = -40002;
    pub const BRIDGE_CALL_FAILED: i32 = -40003;
    pub const MEMORY_ERROR: i32 = -40004;
    pub const SECURITY_DENIED: i32 = -40005;
    pub const SNAPSHOT_FAILED: i32 = -40006;
    pub const TIMEOUT: i32 = -40007;
}

/// IPC event notification (server → client).
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcNotification {
    /// Protocol version.
    pub jsonrpc: String,
    /// Event method name.
    pub method: String,
    /// Event data.
    pub params: serde_json::Value,
}

/// Response payloads for specific methods.
pub mod responses {
    use super::*;

    /// Response for `kernel.status`.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KernelStatus {
        pub version: String,
        pub instance_name: String,
        pub uptime_seconds: f64,
        pub modules_loaded: usize,
        pub total_bridge_calls: u64,
        pub memory_pool_size: usize,
        pub memory_in_use: usize,
        pub status: String,
    }

    /// Response for `module.list`.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ModuleList {
        pub modules: Vec<ModuleInfo>,
    }

    /// Module information in the list.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ModuleInfo {
        pub name: String,
        pub source_language: String,
        pub binary_size: usize,
        pub exports: Vec<String>,
        pub call_count: u64,
        pub instantiated: bool,
    }

    /// Response for `bridge.stats`.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BridgeStats {
        pub total_calls: u64,
        pub successful_calls: u64,
        pub failed_calls: u64,
        pub avg_latency_us: u64,
        pub min_latency_us: u64,
        pub max_latency_us: u64,
        pub p99_latency_us: u64,
    }

    /// Response for `memory.stats`.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MemoryStats {
        pub pool_size: usize,
        pub in_use: usize,
        pub available: usize,
        pub total_allocations: u64,
        pub active_blocks: usize,
        pub peak_usage: u64,
        pub fragmentation_ratio: f64,
    }

    /// Response for `memory.dump`.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MemoryDump {
        pub handle: u32,
        pub offset: usize,
        pub size: usize,
        pub owner: String,
        /// Base64-encoded memory contents.
        pub data_base64: String,
    }

    /// Response for `security.status`.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SecurityStatus {
        pub total_violations: u64,
        pub modules: Vec<ModuleSecurityInfo>,
    }

    /// Security info for a module.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ModuleSecurityInfo {
        pub module: String,
        pub capabilities: Vec<String>,
        pub violation_count: u64,
        pub suspended: bool,
        pub memory_used: u64,
        pub memory_limit: u64,
    }
}

/// Request handler registry — maps method names to handler functions.
pub struct IpcRouter {
    /// Registered method handlers.
    methods: HashMap<String, MethodInfo>,
}

/// Information about a registered method.
pub struct MethodInfo {
    /// Human-readable description.
    pub description: String,
    /// Parameter schema description.
    pub param_schema: String,
    /// Response schema description.
    pub result_schema: String,
    /// Whether this method requires authentication.
    pub requires_auth: bool,
}

impl IpcRouter {
    /// Create a new router with all Omni-Kernel methods registered.
    pub fn new() -> Self {
        let mut methods = HashMap::new();

        // Register all supported methods
        methods.insert(
            "kernel.status".to_string(),
            MethodInfo {
                description: "Get the current runtime status".to_string(),
                param_schema: "none".to_string(),
                result_schema: "KernelStatus".to_string(),
                requires_auth: false,
            },
        );

        methods.insert(
            "kernel.config".to_string(),
            MethodInfo {
                description: "Get or update runtime configuration".to_string(),
                param_schema: "{ section?: string, key?: string, value?: any }".to_string(),
                result_schema: "KernelConfig".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "module.list".to_string(),
            MethodInfo {
                description: "List all loaded WASM modules".to_string(),
                param_schema: "none".to_string(),
                result_schema: "ModuleList".to_string(),
                requires_auth: false,
            },
        );

        methods.insert(
            "module.load".to_string(),
            MethodInfo {
                description: "Load a WASM module from a file path".to_string(),
                param_schema: "{ path: string, name?: string }".to_string(),
                result_schema: "ModuleInfo".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "module.unload".to_string(),
            MethodInfo {
                description: "Unload a WASM module".to_string(),
                param_schema: "{ name: string }".to_string(),
                result_schema: "{ success: bool }".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "bridge.call".to_string(),
            MethodInfo {
                description: "Execute a cross-module function call".to_string(),
                param_schema: "{ module: string, function: string, args: any[] }".to_string(),
                result_schema: "{ result: any, latency_us: u64 }".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "bridge.stats".to_string(),
            MethodInfo {
                description: "Get bridge call statistics".to_string(),
                param_schema: "none".to_string(),
                result_schema: "BridgeStats".to_string(),
                requires_auth: false,
            },
        );

        methods.insert(
            "memory.stats".to_string(),
            MethodInfo {
                description: "Get memory pool statistics".to_string(),
                param_schema: "none".to_string(),
                result_schema: "MemoryStats".to_string(),
                requires_auth: false,
            },
        );

        methods.insert(
            "memory.dump".to_string(),
            MethodInfo {
                description: "Dump contents of a memory handle".to_string(),
                param_schema: "{ handle: u32, offset?: u32, length?: u32 }".to_string(),
                result_schema: "MemoryDump".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "security.status".to_string(),
            MethodInfo {
                description: "Get security sandbox status for all modules".to_string(),
                param_schema: "none".to_string(),
                result_schema: "SecurityStatus".to_string(),
                requires_auth: false,
            },
        );

        methods.insert(
            "profiler.start".to_string(),
            MethodInfo {
                description: "Start a new profiling session".to_string(),
                param_schema: "{ session_id?: string }".to_string(),
                result_schema: "{ session_id: string }".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "profiler.stop".to_string(),
            MethodInfo {
                description: "Stop current profiling session and get results".to_string(),
                param_schema: "{ format?: 'json' | 'chrome_tracing' | 'flamegraph' }".to_string(),
                result_schema: "TraceSession | string".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "snapshot.create".to_string(),
            MethodInfo {
                description: "Create a snapshot of the runtime state".to_string(),
                param_schema: "{ label?: string }".to_string(),
                result_schema: "SnapshotRecord".to_string(),
                requires_auth: true,
            },
        );

        methods.insert(
            "snapshot.restore".to_string(),
            MethodInfo {
                description: "Restore from a snapshot".to_string(),
                param_schema: "{ snapshot_id: string }".to_string(),
                result_schema: "{ success: bool }".to_string(),
                requires_auth: true,
            },
        );

        Self { methods }
    }

    /// Check if a method is registered.
    pub fn has_method(&self, name: &str) -> bool {
        self.methods.contains_key(name)
    }

    /// Get all registered method names.
    pub fn method_names(&self) -> Vec<&str> {
        self.methods.keys().map(|s| s.as_str()).collect()
    }

    /// Create an error response.
    pub fn error_response(id: u64, code: i32, message: &str) -> RpcResponse {
        RpcResponse {
            jsonrpc: PROTOCOL_VERSION.to_string(),
            result: None,
            error: Some(RpcError {
                code,
                message: message.to_string(),
                data: None,
            }),
            id,
        }
    }

    /// Create a success response.
    pub fn success_response(id: u64, result: serde_json::Value) -> RpcResponse {
        RpcResponse {
            jsonrpc: PROTOCOL_VERSION.to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Print all registered methods.
    pub fn print_methods(&self) {
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  🌐 IPC Methods (JSON-RPC)                           │");
        println!("  ├──────────────────────────────────────────────────────┤");

        for (name, info) in &self.methods {
            let auth = if info.requires_auth { "🔒" } else { "🌍" };
            println!("  │  {} {} — {}", auth, name, info.description);
        }

        println!(
            "  │                                                      │"
        );
        println!(
            "  │  Total: {} methods │ Port: {}                │",
            self.methods.len(),
            DEFAULT_PORT
        );
        println!("  └──────────────────────────────────────────────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let req = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "kernel.status".to_string(),
            params: None,
            id: 1,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("kernel.status"));

        let parsed: RpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.method, "kernel.status");
    }

    #[test]
    fn test_response_success() {
        let resp = IpcRouter::success_response(
            1,
            serde_json::json!({ "status": "running" }),
        );

        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_response_error() {
        let resp = IpcRouter::error_response(
            1,
            error_codes::MODULE_NOT_FOUND,
            "Module 'calc' not found",
        );

        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, error_codes::MODULE_NOT_FOUND);
    }

    #[test]
    fn test_router_methods() {
        let router = IpcRouter::new();

        assert!(router.has_method("kernel.status"));
        assert!(router.has_method("module.list"));
        assert!(router.has_method("bridge.call"));
        assert!(router.has_method("memory.stats"));
        assert!(router.has_method("profiler.start"));
        assert!(router.has_method("snapshot.create"));
        assert!(!router.has_method("nonexistent.method"));

        assert!(router.method_names().len() >= 13);
    }

    #[test]
    fn test_kernel_status_response() {
        let status = responses::KernelStatus {
            version: "0.1.0".to_string(),
            instance_name: "test".to_string(),
            uptime_seconds: 123.45,
            modules_loaded: 3,
            total_bridge_calls: 42,
            memory_pool_size: 16 * 1024 * 1024,
            memory_in_use: 4096,
            status: "running".to_string(),
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("0.1.0"));
        assert!(json.contains("running"));
    }
}
