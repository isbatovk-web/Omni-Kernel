//! # 🌉 The Bridge — Cross-Module Communication Engine
//!
//! The Bridge is the core innovation of Omni-Kernel. It enables WASM modules
//! written in different languages to call each other's functions through the
//! host runtime, passing data via the Universal Memory Pool.
//!
//! ## Architecture
//!
//! ```text
//!   ┌──────────┐    bridge_call()     ┌──────────────┐    invoke()     ┌──────────┐
//!   │  JS Mod  │ ──────────────────→  │    Bridge     │ ────────────→  │ Rust Mod │
//!   │ (caller) │                      │  (host-side)  │                │ (callee) │
//!   └──────────┘                      └──────────────┘                └──────────┘
//!        │                                   │                              │
//!        │      bridge_write(handle)         │                              │
//!        └──────────────→ ┌──────────────────┤                              │
//!                         │  Memory Pool     │  ← bridge_read(handle) ──────┘
//!                         └──────────────────┘
//! ```
//!
//! ## How Bridge Calls Work
//!
//! 1. Caller writes arguments to the Memory Pool via `bridge_write()`
//! 2. Caller invokes `bridge_call(target_module, func_name, args_handle)`
//! 3. Bridge resolves target module in the Registry
//! 4. Bridge invokes the target function, passing the handle
//! 5. Callee reads arguments from the Memory Pool (same physical memory!)
//! 6. Callee writes result to the Memory Pool
//! 7. Bridge returns the result handle to the caller

use crate::memory_pool::{MemHandle, UniversalMemoryPool};
use crate::module_registry::{ModuleRegistry, SourceLanguage, ValueType};
use anyhow::{Context, Result};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// A registered bridge function — a native Rust closure that simulates
/// what a real WASM module export would do.
///
/// In production (Phase 2+), this will be replaced by actual
/// `wasmtime::TypedFunc` references from loaded WASM instances.
type BridgeFunction = Box<dyn Fn(&mut UniversalMemoryPool, MemHandle) -> Result<MemHandle>>;

/// The Bridge — orchestrates cross-module function calls through shared memory.
pub struct Bridge {
    /// Registered callable functions: (module_name, func_name) → handler.
    functions: HashMap<(String, String), BridgeFunction>,
    /// Total number of bridge calls made.
    call_count: u64,
}

impl Bridge {
    /// Create a new Bridge instance.
    pub fn new() -> Self {
        info!("Bridge initialized — ready for cross-module dispatch");
        Self {
            functions: HashMap::new(),
            call_count: 0,
        }
    }

    /// Register a function that can be called through the bridge.
    ///
    /// In Phase 2+, this will be called automatically when a WASM module
    /// is instantiated, registering its exports as bridge targets.
    pub fn register_function<F>(&mut self, module: &str, func_name: &str, handler: F)
    where
        F: Fn(&mut UniversalMemoryPool, MemHandle) -> Result<MemHandle> + 'static,
    {
        let key = (module.to_string(), func_name.to_string());
        self.functions.insert(key, Box::new(handler));
        debug!(module, func_name, "Registered bridge function");
    }

    /// Execute a cross-module bridge call.
    ///
    /// This is the core dispatch mechanism. When WASM Module A wants to call
    /// a function in WASM Module B:
    ///
    /// 1. Module A calls the imported host function `bridge_call`
    /// 2. The host intercepts this and calls `Bridge::call()`
    /// 3. Bridge looks up the target function and invokes it
    /// 4. The result handle is returned to Module A
    pub fn call(
        &mut self,
        pool: &mut UniversalMemoryPool,
        registry: &mut ModuleRegistry,
        target_module: &str,
        func_name: &str,
        args_handle: MemHandle,
    ) -> Result<MemHandle> {
        info!(
            target = target_module,
            function = func_name,
            args_handle,
            "Bridge call dispatching"
        );

        // Look up the target function
        let key = (target_module.to_string(), func_name.to_string());
        let handler = self
            .functions
            .get(&key)
            .context(format!(
                "Bridge target not found: {}::{}",
                target_module, func_name
            ))?;

        // Record the call in the registry
        let _ = registry.record_call(target_module);

        // Execute the function
        let result_handle = handler(pool, args_handle)?;

        self.call_count += 1;

        info!(
            target = target_module,
            function = func_name,
            result_handle,
            "Bridge call completed"
        );

        Ok(result_handle)
    }

    /// Get the total number of bridge calls made.
    pub fn call_count(&self) -> u64 {
        self.call_count
    }
}

/// Demo: Register example bridge functions that simulate real WASM module exports.
///
/// This demonstrates the bridge mechanism without requiring compiled WASM modules.
/// In Phase 2, these will be replaced by actual WASM function references.
pub fn register_demo_functions(
    bridge: &mut Bridge,
    registry: &mut ModuleRegistry,
) -> Result<()> {
    // ── Register "calc-engine" module (simulates a Rust WASM module) ─────
    registry.register("calc-engine", SourceLanguage::Rust, 4096)?;
    registry.register_export(
        "calc-engine",
        "add",
        vec![ValueType::I32, ValueType::I32],
        vec![ValueType::I32],
    )?;
    registry.register_export(
        "calc-engine",
        "multiply",
        vec![ValueType::I32, ValueType::I32],
        vec![ValueType::I32],
    )?;
    registry.register_export(
        "calc-engine",
        "fibonacci",
        vec![ValueType::I32],
        vec![ValueType::I64],
    )?;
    registry.mark_instantiated("calc-engine")?;

    // Bridge function: add(a, b) → a + b
    // Reads two i32 values from the args handle, writes result to a new handle
    bridge.register_function("calc-engine", "add", |pool, args_handle| {
        // Read two i32 values (8 bytes total) from args
        let args_data = pool.read(args_handle, 0, 8)?;
        let a = i32::from_le_bytes(args_data[0..4].try_into().unwrap());
        let b = i32::from_le_bytes(args_data[4..8].try_into().unwrap());

        let result = a + b;
        debug!(a, b, result, "calc-engine::add executed");

        // Write result to a new block
        let result_handle = pool.alloc(4, "calc-engine")?;
        pool.write(result_handle, 0, &result.to_le_bytes())?;

        Ok(result_handle)
    });

    // Bridge function: multiply(a, b) → a * b
    bridge.register_function("calc-engine", "multiply", |pool, args_handle| {
        let args_data = pool.read(args_handle, 0, 8)?;
        let a = i32::from_le_bytes(args_data[0..4].try_into().unwrap());
        let b = i32::from_le_bytes(args_data[4..8].try_into().unwrap());

        let result = a * b;
        debug!(a, b, result, "calc-engine::multiply executed");

        let result_handle = pool.alloc(4, "calc-engine")?;
        pool.write(result_handle, 0, &result.to_le_bytes())?;

        Ok(result_handle)
    });

    // Bridge function: fibonacci(n) → fib(n)
    bridge.register_function("calc-engine", "fibonacci", |pool, args_handle| {
        let args_data = pool.read(args_handle, 0, 4)?;
        let n = i32::from_le_bytes(args_data[0..4].try_into().unwrap()) as u64;

        // Compute fibonacci iteratively (high-performance path)
        let result = if n <= 1 {
            n
        } else {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        };

        debug!(n, result, "calc-engine::fibonacci executed");

        let result_handle = pool.alloc(8, "calc-engine")?;
        pool.write(result_handle, 0, &result.to_le_bytes())?;

        Ok(result_handle)
    });

    // ── Register "orchestrator" module (simulates a JS WASM module) ──────
    registry.register("orchestrator", SourceLanguage::JavaScript, 8192)?;
    registry.register_export(
        "orchestrator",
        "run_pipeline",
        vec![ValueType::I32],
        vec![ValueType::I32],
    )?;
    registry.mark_instantiated("orchestrator")?;

    // Bridge function: run_pipeline — demonstrates cross-module call chaining
    // The orchestrator calls calc-engine functions and assembles results
    bridge.register_function("orchestrator", "run_pipeline", |pool, args_handle| {
        let args_data = pool.read(args_handle, 0, 4)?;
        let input = i32::from_le_bytes(args_data[0..4].try_into().unwrap());

        debug!(input, "orchestrator::run_pipeline started");

        // In a real implementation, this would trigger bridge calls back to
        // calc-engine. For the demo, we simulate the pipeline result.
        let pipeline_result = input * 2 + 42; // Simulated pipeline

        let result_handle = pool.alloc(4, "orchestrator")?;
        pool.write(result_handle, 0, &pipeline_result.to_le_bytes())?;

        Ok(result_handle)
    });

    info!("Demo bridge functions registered successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_call_add() {
        let mut pool = UniversalMemoryPool::new(4096);
        let mut registry = ModuleRegistry::new();
        let mut bridge = Bridge::new();

        register_demo_functions(&mut bridge, &mut registry).unwrap();

        // Write args: a=10, b=32
        let args = pool.alloc(8, "test").unwrap();
        pool.write(args, 0, &10_i32.to_le_bytes()).unwrap();
        pool.write(args, 4, &32_i32.to_le_bytes()).unwrap();

        // Call calc-engine::add through the bridge
        let result_handle = bridge
            .call(&mut pool, &mut registry, "calc-engine", "add", args)
            .unwrap();

        // Read result
        let result_data = pool.read(result_handle, 0, 4).unwrap();
        let result = i32::from_le_bytes(result_data.try_into().unwrap());
        assert_eq!(result, 42);
    }

    #[test]
    fn test_bridge_call_fibonacci() {
        let mut pool = UniversalMemoryPool::new(4096);
        let mut registry = ModuleRegistry::new();
        let mut bridge = Bridge::new();

        register_demo_functions(&mut bridge, &mut registry).unwrap();

        // Compute fib(10) = 55
        let args = pool.alloc(4, "test").unwrap();
        pool.write(args, 0, &10_i32.to_le_bytes()).unwrap();

        let result_handle = bridge
            .call(
                &mut pool,
                &mut registry,
                "calc-engine",
                "fibonacci",
                args,
            )
            .unwrap();

        let result_data = pool.read(result_handle, 0, 8).unwrap();
        let result = u64::from_le_bytes(result_data.try_into().unwrap());
        assert_eq!(result, 55);
    }

    #[test]
    fn test_call_counting() {
        let mut pool = UniversalMemoryPool::new(4096);
        let mut registry = ModuleRegistry::new();
        let mut bridge = Bridge::new();

        register_demo_functions(&mut bridge, &mut registry).unwrap();

        let args = pool.alloc(8, "test").unwrap();
        pool.write(args, 0, &1_i32.to_le_bytes()).unwrap();
        pool.write(args, 4, &2_i32.to_le_bytes()).unwrap();

        bridge
            .call(&mut pool, &mut registry, "calc-engine", "add", args)
            .unwrap();
        bridge
            .call(&mut pool, &mut registry, "calc-engine", "add", args)
            .unwrap();

        assert_eq!(bridge.call_count(), 2);
        assert_eq!(registry.get("calc-engine").unwrap().call_count, 2);
    }
}
