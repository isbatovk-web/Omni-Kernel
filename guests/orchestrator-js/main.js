/**
 * ═══════════════════════════════════════════════════════════
 * 📜 Omni-Kernel JS Orchestrator Module
 * ═══════════════════════════════════════════════════════════
 *
 * This JavaScript module runs inside a WASM-based JS engine
 * (QuickJS compiled to WASM) within the Omni-Kernel runtime.
 *
 * It demonstrates how a high-level language (JavaScript) can
 * orchestrate computations performed by low-level modules
 * (Rust, C++) through the Omni-Kernel Bridge.
 *
 * ┌─────────────────────────────────────────────────────────┐
 * │  This file will be embedded into a QuickJS WASM module  │
 * │  in Phase 2. For now, it serves as a design reference.  │
 * └─────────────────────────────────────────────────────────┘
 */

// ──────────────────────────────────────────────────────────
// Bridge API (provided by Omni-Kernel host via imports)
// ──────────────────────────────────────────────────────────

/**
 * @typedef {Object} Bridge
 * @property {function(number): number} alloc - Allocate shared memory
 * @property {function(number): void} free - Free shared memory
 * @property {function(number, number, Uint8Array): void} write - Write to shared memory
 * @property {function(number, number, number): Uint8Array} read - Read from shared memory
 * @property {function(string, string, number): number} call - Call another module's function
 * @property {function(string): void} log - Log a message
 */

/** @type {Bridge} */
const bridge = globalThis.__omni_bridge__;

// ──────────────────────────────────────────────────────────
// Helper: Typed data serialization for the bridge
// ──────────────────────────────────────────────────────────

/**
 * Write two i32 values to a shared memory block.
 * @param {number} a - First integer
 * @param {number} b - Second integer
 * @returns {number} Handle to the args block
 */
function writeI32Pair(a, b) {
  const handle = bridge.alloc(8);
  const buffer = new ArrayBuffer(8);
  const view = new DataView(buffer);
  view.setInt32(0, a, true); // little-endian
  view.setInt32(4, b, true);
  bridge.write(handle, 0, new Uint8Array(buffer));
  return handle;
}

/**
 * Read an i32 from a shared memory block.
 * @param {number} handle - Memory handle
 * @returns {number} The integer value
 */
function readI32(handle) {
  const data = bridge.read(handle, 0, 4);
  const view = new DataView(data.buffer);
  return view.getInt32(0, true);
}

/**
 * Read an i64 from a shared memory block.
 * @param {number} handle - Memory handle
 * @returns {BigInt} The integer value
 */
function readI64(handle) {
  const data = bridge.read(handle, 0, 8);
  const view = new DataView(data.buffer);
  return view.getBigInt64(0, true);
}

// ──────────────────────────────────────────────────────────
// Orchestration Pipeline
// ──────────────────────────────────────────────────────────

/**
 * Run the main computation pipeline.
 *
 * This demonstrates how JS orchestrates multiple calls to the
 * Rust calc-engine through the Omni-Kernel Bridge:
 *
 * 1. Call calc-engine::add(10, 32) → 42
 * 2. Call calc-engine::multiply(result, 3) → 126
 * 3. Call calc-engine::fibonacci(10) → 55
 * 4. Combine all results
 *
 * All data travels through the Universal Memory Pool — no JSON,
 * no serialization, just raw bytes in shared memory.
 */
function runPipeline() {
  bridge.log("Pipeline started — JS orchestrating Rust computations");

  // Step 1: Add 10 + 32
  const addArgs = writeI32Pair(10, 32);
  const addResult = bridge.call("calc-engine", "add", addArgs);
  const sum = readI32(addResult);
  bridge.log(`Step 1: add(10, 32) = ${sum}`);
  bridge.free(addArgs);

  // Step 2: Multiply the sum by 3
  const mulArgs = writeI32Pair(sum, 3);
  const mulResult = bridge.call("calc-engine", "multiply", mulArgs);
  const product = readI32(mulResult);
  bridge.log(`Step 2: multiply(${sum}, 3) = ${product}`);
  bridge.free(mulArgs);

  // Step 3: Fibonacci(10)
  const fibArgs = bridge.alloc(4);
  const fibBuffer = new ArrayBuffer(4);
  new DataView(fibBuffer).setInt32(0, 10, true);
  bridge.write(fibArgs, 0, new Uint8Array(fibBuffer));
  const fibResult = bridge.call("calc-engine", "fibonacci", fibArgs);
  const fib = readI64(fibResult);
  bridge.log(`Step 3: fibonacci(10) = ${fib}`);
  bridge.free(fibArgs);

  // Step 4: Combine results
  const finalResult = Number(fib) + product;
  bridge.log(`Pipeline complete: fib(10) + multiply(add(10,32), 3) = ${fib} + ${product} = ${finalResult}`);

  // Clean up result handles
  bridge.free(addResult);
  bridge.free(mulResult);
  bridge.free(fibResult);

  return finalResult;
}

/**
 * Data transformation pipeline — demonstrates processing arrays.
 *
 * Shows how JS can prepare data in shared memory for processing
 * by a high-performance Rust module, then collect results.
 */
function transformData(inputArray) {
  bridge.log(`Transforming array of ${inputArray.length} elements`);

  // Write input array to shared memory
  const dataSize = inputArray.length * 8; // f64 = 8 bytes
  const handle = bridge.alloc(dataSize);
  const buffer = new ArrayBuffer(dataSize);
  const view = new DataView(buffer);

  for (let i = 0; i < inputArray.length; i++) {
    view.setFloat64(i * 8, inputArray[i], true);
  }
  bridge.write(handle, 0, new Uint8Array(buffer));

  // Call Rust module for heavy computation
  const resultHandle = bridge.call("calc-engine", "dot_product", handle);

  // Read result
  const resultData = bridge.read(resultHandle, 0, 8);
  const result = new DataView(resultData.buffer).getFloat64(0, true);

  // Clean up
  bridge.free(handle);
  bridge.free(resultHandle);

  return result;
}

// ──────────────────────────────────────────────────────────
// Module Lifecycle (called by Omni-Kernel host)
// ──────────────────────────────────────────────────────────

/**
 * Initialize the orchestrator module.
 * Called by the host when this module is loaded.
 */
function guestInit() {
  bridge.log("JS Orchestrator module initialized");
  return true;
}

/**
 * Handle incoming invocations from the bridge.
 * Called when another module (or the host) calls this module.
 */
function guestInvoke(funcName, argsHandle) {
  switch (funcName) {
    case "run_pipeline":
      return runPipeline();
    case "transform_data":
      return transformData(argsHandle);
    default:
      bridge.log(`Unknown function: ${funcName}`);
      return -1;
  }
}

/**
 * Shut down the module.
 */
function guestShutdown() {
  bridge.log("JS Orchestrator module shutting down");
}

// Export lifecycle functions for the host
globalThis.__omni_exports__ = {
  init: guestInit,
  invoke: guestInvoke,
  shutdown: guestShutdown,
};
