//! # 🧠 Universal Memory Pool
//!
//! The Universal Memory Pool is a host-managed contiguous memory region that
//! enables "zero-copy" interoperability between WASM modules. Instead of each
//! module serializing data to pass to another, they read/write to this shared
//! pool using opaque handles.
//!
//! ## Design
//!
//! ```text
//! ┌───────────────────────────────────────────────────────────┐
//! │                UNIVERSAL MEMORY POOL                      │
//! ├──────┬──────┬──────────┬──────┬───────────────────────────┤
//! │ Blk0 │ Blk1 │  Blk2    │ Blk3 │       Free Space         │
//! │ 4KB  │ 16KB │  64KB    │ 8KB  │                           │
//! └──────┴──────┴──────────┴──────┴───────────────────────────┘
//! ```
//!
//! Each allocation returns a `MemHandle` — an opaque integer that maps to
//! a `(offset, size, owner)` tuple in the metadata table.

use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, warn};

/// Errors that can occur in the memory pool.
#[derive(Error, Debug)]
pub enum PoolError {
    #[error("Out of memory: requested {requested} bytes, only {available} available")]
    OutOfMemory { requested: usize, available: usize },

    #[error("Invalid handle: {0}")]
    InvalidHandle(u32),

    #[error("Access out of bounds: offset {offset} + length {length} exceeds block size {block_size}")]
    OutOfBounds {
        offset: usize,
        length: usize,
        block_size: usize,
    },

    #[error("Double free detected for handle {0}")]
    DoubleFree(u32),
}

/// Metadata for an allocated memory block.
#[derive(Debug, Clone)]
pub struct BlockMeta {
    /// Offset within the pool's backing buffer.
    pub offset: usize,
    /// Size of this allocation in bytes.
    pub size: usize,
    /// Which module owns this block (module ID).
    pub owner: String,
    /// Reference count for shared blocks.
    pub ref_count: u32,
    /// Whether this block has been freed.
    pub freed: bool,
}

/// An opaque handle to a memory block in the pool.
pub type MemHandle = u32;

/// The Universal Memory Pool.
///
/// Manages a contiguous byte buffer with a simple bump-allocator strategy.
/// In future phases, this will be upgraded to a buddy allocator for better
/// fragmentation handling.
pub struct UniversalMemoryPool {
    /// The raw backing memory.
    buffer: Vec<u8>,
    /// Total capacity of the pool.
    capacity: usize,
    /// Current allocation watermark (bump pointer).
    watermark: usize,
    /// Metadata for each allocated block, keyed by handle.
    blocks: HashMap<MemHandle, BlockMeta>,
    /// Next handle ID to assign.
    next_handle: MemHandle,
    /// Total number of allocations made (lifetime).
    total_allocations: u64,
    /// Total bytes currently allocated.
    bytes_allocated: usize,
}

/// Alignment for all allocations (8 bytes for WASM compatibility).
const ALIGNMENT: usize = 8;

impl UniversalMemoryPool {
    /// Create a new memory pool with the given capacity in bytes.
    ///
    /// # Arguments
    /// * `capacity` - Size of the pool in bytes. Typical: 1MB (1_048_576) to 256MB.
    pub fn new(capacity: usize) -> Self {
        debug!(capacity, "Creating Universal Memory Pool");
        Self {
            buffer: vec![0u8; capacity],
            capacity,
            watermark: 0,
            blocks: HashMap::new(),
            next_handle: 1, // Handle 0 is reserved as "null"
            total_allocations: 0,
            bytes_allocated: 0,
        }
    }

    /// Allocate a block of `size` bytes in the pool.
    ///
    /// Returns a `MemHandle` that can be used to read/write/free the block.
    /// All allocations are 8-byte aligned.
    pub fn alloc(&mut self, size: usize, owner: &str) -> Result<MemHandle, PoolError> {
        // Align the size up to ALIGNMENT boundary
        let aligned_size = (size + ALIGNMENT - 1) & !(ALIGNMENT - 1);

        // Check if we have enough space
        let available = self.capacity - self.watermark;
        if aligned_size > available {
            warn!(
                requested = aligned_size,
                available, "Memory pool allocation failed: out of memory"
            );
            return Err(PoolError::OutOfMemory {
                requested: aligned_size,
                available,
            });
        }

        // Allocate at the current watermark
        let offset = self.watermark;
        self.watermark += aligned_size;

        // Create handle and metadata
        let handle = self.next_handle;
        self.next_handle += 1;
        self.total_allocations += 1;
        self.bytes_allocated += aligned_size;

        let meta = BlockMeta {
            offset,
            size: aligned_size,
            owner: owner.to_string(),
            ref_count: 1,
            freed: false,
        };

        self.blocks.insert(handle, meta);

        debug!(
            handle,
            offset,
            size = aligned_size,
            owner,
            "Allocated memory block"
        );

        Ok(handle)
    }

    /// Free a previously allocated block.
    ///
    /// Note: With the current bump allocator, freed memory is not reclaimed
    /// unless it's the most recently allocated block. A buddy allocator
    /// upgrade (Phase 3) will fix this.
    pub fn free(&mut self, handle: MemHandle) -> Result<(), PoolError> {
        let meta = self
            .blocks
            .get_mut(&handle)
            .ok_or(PoolError::InvalidHandle(handle))?;

        if meta.freed {
            return Err(PoolError::DoubleFree(handle));
        }

        meta.ref_count = meta.ref_count.saturating_sub(1);

        if meta.ref_count == 0 {
            meta.freed = true;
            self.bytes_allocated -= meta.size;

            // If this is the top of the watermark, reclaim space
            if meta.offset + meta.size == self.watermark {
                self.watermark = meta.offset;
            }

            debug!(handle, "Freed memory block");
        } else {
            debug!(
                handle,
                remaining_refs = meta.ref_count,
                "Decremented reference count"
            );
        }

        Ok(())
    }

    /// Write data into an allocated block at the given offset.
    ///
    /// This is the "write side" of the bridge — a WASM module copies data
    /// from its linear memory into the shared pool.
    pub fn write(
        &mut self,
        handle: MemHandle,
        offset: usize,
        data: &[u8],
    ) -> Result<(), PoolError> {
        let meta = self
            .blocks
            .get(&handle)
            .ok_or(PoolError::InvalidHandle(handle))?
            .clone();

        if meta.freed {
            return Err(PoolError::InvalidHandle(handle));
        }

        // Bounds check
        if offset + data.len() > meta.size {
            return Err(PoolError::OutOfBounds {
                offset,
                length: data.len(),
                block_size: meta.size,
            });
        }

        // Write to the buffer
        let start = meta.offset + offset;
        self.buffer[start..start + data.len()].copy_from_slice(data);

        debug!(
            handle,
            offset,
            len = data.len(),
            "Wrote data to memory block"
        );

        Ok(())
    }

    /// Read data from an allocated block at the given offset.
    ///
    /// This is the "read side" of the bridge — a WASM module copies data
    /// from the shared pool into its linear memory.
    pub fn read(
        &self,
        handle: MemHandle,
        offset: usize,
        length: usize,
    ) -> Result<&[u8], PoolError> {
        let meta = self
            .blocks
            .get(&handle)
            .ok_or(PoolError::InvalidHandle(handle))?;

        if meta.freed {
            return Err(PoolError::InvalidHandle(handle));
        }

        // Bounds check
        if offset + length > meta.size {
            return Err(PoolError::OutOfBounds {
                offset,
                length,
                block_size: meta.size,
            });
        }

        let start = meta.offset + offset;
        Ok(&self.buffer[start..start + length])

    }

    /// Increment the reference count for a block (for shared access).
    pub fn add_ref(&mut self, handle: MemHandle) -> Result<(), PoolError> {
        let meta = self
            .blocks
            .get_mut(&handle)
            .ok_or(PoolError::InvalidHandle(handle))?;

        if meta.freed {
            return Err(PoolError::InvalidHandle(handle));
        }

        meta.ref_count += 1;
        debug!(
            handle,
            ref_count = meta.ref_count,
            "Added reference to memory block"
        );
        Ok(())
    }

    /// Get metadata for a block.
    pub fn get_meta(&self, handle: MemHandle) -> Result<&BlockMeta, PoolError> {
        self.blocks
            .get(&handle)
            .ok_or(PoolError::InvalidHandle(handle))
    }

    /// Get the total capacity of the pool.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the number of bytes currently allocated.
    pub fn bytes_allocated(&self) -> usize {
        self.bytes_allocated
    }

    /// Get the number of bytes available for allocation.
    pub fn bytes_available(&self) -> usize {
        self.capacity - self.watermark
    }

    /// Get the total number of allocations made (lifetime).
    pub fn total_allocations(&self) -> u64 {
        self.total_allocations
    }

    /// Get the number of active (non-freed) blocks.
    pub fn active_blocks(&self) -> usize {
        self.blocks.values().filter(|b| !b.freed).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_and_read_write() {
        let mut pool = UniversalMemoryPool::new(1024);

        // Allocate a block
        let handle = pool.alloc(64, "test-module").unwrap();
        assert!(handle > 0);

        // Write data
        let data = b"Hello from Omni-Kernel!";
        pool.write(handle, 0, data).unwrap();

        // Read data back
        let read_back = pool.read(handle, 0, data.len()).unwrap();
        assert_eq!(read_back, data);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut pool = UniversalMemoryPool::new(256);
        let handle = pool.alloc(16, "test").unwrap();

        // Try to write beyond block bounds
        let result = pool.write(handle, 0, &[0u8; 32]);
        assert!(result.is_err());
    }

    #[test]
    fn test_free_and_double_free() {
        let mut pool = UniversalMemoryPool::new(256);
        let handle = pool.alloc(16, "test").unwrap();

        pool.free(handle).unwrap();
        assert!(pool.free(handle).is_err()); // Double free should error
    }

    #[test]
    fn test_reference_counting() {
        let mut pool = UniversalMemoryPool::new(256);
        let handle = pool.alloc(16, "module-a").unwrap();

        // Add a reference (module-b also uses this block)
        pool.add_ref(handle).unwrap();
        assert_eq!(pool.get_meta(handle).unwrap().ref_count, 2);

        // First free just decrements
        pool.free(handle).unwrap();
        assert!(!pool.get_meta(handle).unwrap().freed);

        // Second free actually frees
        pool.free(handle).unwrap();
        assert!(pool.get_meta(handle).unwrap().freed);
    }

    #[test]
    fn test_alignment() {
        let mut pool = UniversalMemoryPool::new(1024);

        // Allocate 3 bytes — should be aligned to 8
        let h1 = pool.alloc(3, "test").unwrap();
        let h2 = pool.alloc(5, "test").unwrap();

        let m1 = pool.get_meta(h1).unwrap();
        let m2 = pool.get_meta(h2).unwrap();

        assert_eq!(m1.size, 8); // 3 → 8 (aligned)
        assert_eq!(m2.offset, 8); // Second block starts at 8
    }
}
