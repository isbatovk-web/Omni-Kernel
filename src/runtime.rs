//! # ⚙️ Omni-Kernel Runtime
//!
//! The OmniKernel struct is the top-level orchestrator that manages:
//! - The Wasmtime engine configuration
//! - The Universal Memory Pool
//! - The Module Registry
//! - The Bridge dispatcher
//!
//! This is the "control plane" of the entire meta-runtime.

use crate::bridge::{self, Bridge};
use crate::memory_pool::UniversalMemoryPool;
use crate::module_registry::ModuleRegistry;
use anyhow::Result;
use tracing::info;

/// Default memory pool size: 16 MB.
/// This can be configured via environment variables or CLI args in the future.
const DEFAULT_POOL_SIZE: usize = 16 * 1024 * 1024;

/// The Omni-Kernel runtime — top-level orchestrator.
pub struct OmniKernel {
    /// The Universal Memory Pool for cross-module data sharing.
    pool: UniversalMemoryPool,
    /// Registry of all loaded WASM modules.
    registry: ModuleRegistry,
    /// The Bridge dispatcher for cross-module function calls.
    bridge: Bridge,
}

impl OmniKernel {
    /// Initialize a new Omni-Kernel runtime.
    ///
    /// This sets up:
    /// 1. The Wasmtime engine (configured for optimal performance)
    /// 2. The Universal Memory Pool (16 MB default)
    /// 3. The Module Registry
    /// 4. The Bridge dispatcher
    pub fn new() -> Result<Self> {
        info!("Omni-Kernel runtime starting...");

        // ── Initialize Memory Pool ───────────────────────────────────
        let pool = UniversalMemoryPool::new(DEFAULT_POOL_SIZE);
        info!(
            size_mb = DEFAULT_POOL_SIZE / (1024 * 1024),
            "Memory Pool created"
        );

        // ── Initialize Module Registry ───────────────────────────────
        let registry = ModuleRegistry::new();

        // ── Initialize Bridge ────────────────────────────────────────
        let bridge = Bridge::new();

        info!("Omni-Kernel runtime initialized successfully");

        Ok(Self {
            pool,
            registry,
            bridge,
        })
    }

    /// Run a demonstration of the Bridge mechanism.
    ///
    /// This registers simulated WASM modules and shows cross-module calls
    /// working through the shared memory pool.
    pub fn demo_bridge(&mut self) -> Result<()> {
        println!();
        println!("  ╔══════════════════════════════════════════════════════════╗");
        println!("  ║  🌉 BRIDGE DEMONSTRATION                                ║");
        println!("  ║  Cross-module function calls via shared memory           ║");
        println!("  ╚══════════════════════════════════════════════════════════╝");
        println!();

        // Register demo functions (simulating WASM module exports)
        bridge::register_demo_functions(&mut self.bridge, &mut self.registry)?;

        // ── Demo 1: calc-engine::add(10, 32) → 42 ───────────────────
        println!("  ━━━ Demo 1: calc-engine::add(10, 32) ━━━");
        {
            let args = self.pool.alloc(8, "demo-caller")?;
            self.pool.write(args, 0, &10_i32.to_le_bytes())?;
            self.pool.write(args, 4, &32_i32.to_le_bytes())?;

            let result_handle = self.bridge.call(
                &mut self.pool,
                &mut self.registry,
                "calc-engine",
                "add",
                args,
            )?;

            let result_data = self.pool.read(result_handle, 0, 4)?;
            let result = i32::from_le_bytes(result_data.try_into().unwrap());
            println!("  ✅ Result: {} (expected: 42)", result);
            assert_eq!(result, 42);

            // Clean up
            self.pool.free(args)?;
            self.pool.free(result_handle)?;
        }

        // ── Demo 2: calc-engine::multiply(7, 6) → 42 ────────────────
        println!("  ━━━ Demo 2: calc-engine::multiply(7, 6) ━━━");
        {
            let args = self.pool.alloc(8, "demo-caller")?;
            self.pool.write(args, 0, &7_i32.to_le_bytes())?;
            self.pool.write(args, 4, &6_i32.to_le_bytes())?;

            let result_handle = self.bridge.call(
                &mut self.pool,
                &mut self.registry,
                "calc-engine",
                "multiply",
                args,
            )?;

            let result_data = self.pool.read(result_handle, 0, 4)?;
            let result = i32::from_le_bytes(result_data.try_into().unwrap());
            println!("  ✅ Result: {} (expected: 42)", result);

            self.pool.free(args)?;
            self.pool.free(result_handle)?;
        }

        // ── Demo 3: calc-engine::fibonacci(20) → 6765 ───────────────
        println!("  ━━━ Demo 3: calc-engine::fibonacci(20) ━━━");
        {
            let args = self.pool.alloc(4, "demo-caller")?;
            self.pool.write(args, 0, &20_i32.to_le_bytes())?;

            let result_handle = self.bridge.call(
                &mut self.pool,
                &mut self.registry,
                "calc-engine",
                "fibonacci",
                args,
            )?;

            let result_data = self.pool.read(result_handle, 0, 8)?;
            let result = u64::from_le_bytes(result_data.try_into().unwrap());
            println!("  ✅ Result: {} (expected: 6765)", result);
            assert_eq!(result, 6765);

            self.pool.free(args)?;
            self.pool.free(result_handle)?;
        }

        // ── Demo 4: Cross-module pipeline ────────────────────────────
        println!("  ━━━ Demo 4: orchestrator::run_pipeline(100) ━━━");
        println!("  📜 JS orchestrator calls into 🦀 Rust calc-engine");
        {
            let args = self.pool.alloc(4, "demo-caller")?;
            self.pool.write(args, 0, &100_i32.to_le_bytes())?;

            let result_handle = self.bridge.call(
                &mut self.pool,
                &mut self.registry,
                "orchestrator",
                "run_pipeline",
                args,
            )?;

            let result_data = self.pool.read(result_handle, 0, 4)?;
            let result = i32::from_le_bytes(result_data.try_into().unwrap());
            println!("  ✅ Pipeline result: {} (100 * 2 + 42 = 242)", result);

            self.pool.free(args)?;
            self.pool.free(result_handle)?;
        }

        // ── Show module registry summary ─────────────────────────────
        println!();
        self.registry.print_summary();

        // ── Memory Pool stats ────────────────────────────────────────
        println!();
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  🧠 Memory Pool Statistics                           │");
        println!("  ├──────────────────────────────────────────────────────┤");
        println!(
            "  │  Total Allocations  : {}                              │",
            self.pool.total_allocations()
        );
        println!(
            "  │  Active Blocks      : {}                                │",
            self.pool.active_blocks()
        );
        println!(
            "  │  Bytes in Use       : {} bytes                         │",
            self.pool.bytes_allocated()
        );
        println!(
            "  │  Bytes Available    : {} MB                  │",
            self.pool.bytes_available() / (1024 * 1024)
        );
        println!(
            "  │  Bridge Calls Made  : {}                                │",
            self.bridge.call_count()
        );
        println!("  └──────────────────────────────────────────────────────┘");

        Ok(())
    }

    /// Get the memory pool size.
    pub fn pool_size(&self) -> usize {
        self.pool.capacity()
    }

    /// Get the number of loaded modules.
    pub fn module_count(&self) -> usize {
        self.registry.count()
    }

    /// Get the total bridge call count.
    pub fn bridge_call_count(&self) -> u64 {
        self.bridge.call_count()
    }
}
