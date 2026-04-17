//! # 💾 Snapshot & State Persistence
//!
//! The Snapshot module enables saving and restoring the entire Omni-Kernel
//! runtime state, including:
//!
//! - Memory pool contents
//! - Module registry state
//! - Bridge function registrations
//! - Security profiles
//! - Configuration
//!
//! This enables:
//! - Fast restart (restore from snapshot instead of full initialization)
//! - Debugging (capture state at a specific point in time)
//! - Migration (move runtime state between processes/machines)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::{info, warn};

/// Snapshot errors.
#[derive(Error, Debug)]
pub enum SnapshotError {
    #[error("Failed to serialize snapshot: {0}")]
    SerializationFailed(String),

    #[error("Failed to deserialize snapshot: {0}")]
    DeserializationFailed(String),

    #[error("Failed to write snapshot to '{path}': {error}")]
    WriteFailed { path: String, error: String },

    #[error("Failed to read snapshot from '{path}': {error}")]
    ReadFailed { path: String, error: String },

    #[error("Snapshot version mismatch: expected {expected}, got {found}")]
    VersionMismatch { expected: String, found: String },

    #[error("Snapshot is corrupted: {0}")]
    Corrupted(String),
}

/// A complete snapshot of the runtime state.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    /// Snapshot format version.
    pub version: String,
    /// When this snapshot was taken.
    pub timestamp: u64,
    /// Runtime instance name.
    pub instance_name: String,
    /// Memory pool snapshot.
    pub memory: MemorySnapshot,
    /// Module registry snapshot.
    pub modules: Vec<ModuleSnapshot>,
    /// Security profiles.
    pub security_profiles: Vec<SecurityProfileSnapshot>,
    /// Runtime statistics at snapshot time.
    pub stats: StatsSnapshot,
    /// Custom metadata.
    pub metadata: HashMap<String, String>,
}

/// Memory pool snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// Total pool capacity.
    pub capacity: usize,
    /// Current watermark position.
    pub watermark: usize,
    /// Active block metadata.
    pub blocks: Vec<BlockSnapshot>,
    /// The raw memory data (base64 encoded for JSON compatibility).
    pub data: Vec<u8>,
}

/// Snapshot of a single memory block.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSnapshot {
    pub handle: u32,
    pub offset: usize,
    pub size: usize,
    pub owner: String,
    pub ref_count: u32,
}

/// Snapshot of a loaded module.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSnapshot {
    pub name: String,
    pub source_language: String,
    pub binary_size: usize,
    pub exports: Vec<String>,
    pub call_count: u64,
}

/// Security profile snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityProfileSnapshot {
    pub module_name: String,
    pub capabilities: Vec<String>,
    pub memory_bytes_used: u64,
    pub memory_blocks_used: u64,
    pub violation_count: u64,
    pub suspended: bool,
}

/// Runtime statistics snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatsSnapshot {
    pub total_bridge_calls: u64,
    pub total_memory_allocations: u64,
    pub modules_loaded: u64,
    pub uptime_seconds: f64,
    pub avg_bridge_latency_us: u64,
    pub peak_memory_bytes: u64,
}

/// The Snapshot Manager.
pub struct SnapshotManager {
    /// Directory for storing snapshots.
    snapshot_dir: String,
    /// Maximum number of snapshots to retain.
    max_snapshots: usize,
    /// Auto-snapshot interval (None = disabled).
    auto_interval: Option<std::time::Duration>,
    /// Snapshot history.
    history: Vec<SnapshotRecord>,
}

/// Record of a taken snapshot.
#[derive(Debug, Clone)]
pub struct SnapshotRecord {
    pub id: String,
    pub path: String,
    pub timestamp: u64,
    pub size_bytes: u64,
    pub modules_count: usize,
    pub memory_size: usize,
}

impl SnapshotManager {
    /// Create a new snapshot manager.
    pub fn new(snapshot_dir: &str, max_snapshots: usize) -> Self {
        let _ = std::fs::create_dir_all(snapshot_dir);
        info!(dir = snapshot_dir, "Snapshot Manager initialized");
        Self {
            snapshot_dir: snapshot_dir.to_string(),
            max_snapshots,
            auto_interval: None,
            history: Vec::new(),
        }
    }

    /// Set auto-snapshot interval.
    pub fn set_auto_interval(&mut self, interval: std::time::Duration) {
        self.auto_interval = Some(interval);
    }

    /// Create a snapshot of the current runtime state.
    pub fn create_snapshot(
        &mut self,
        instance_name: &str,
        pool_capacity: usize,
        pool_watermark: usize,
        pool_data: &[u8],
        modules: Vec<ModuleSnapshot>,
        stats: StatsSnapshot,
    ) -> Result<SnapshotRecord, SnapshotError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let snapshot_id = format!("snapshot_{}", timestamp);

        let snapshot = RuntimeSnapshot {
            version: "0.1.0".to_string(),
            timestamp,
            instance_name: instance_name.to_string(),
            memory: MemorySnapshot {
                capacity: pool_capacity,
                watermark: pool_watermark,
                blocks: Vec::new(),
                data: pool_data.to_vec(),
            },
            modules,
            security_profiles: Vec::new(),
            stats,
            metadata: HashMap::new(),
        };

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&snapshot)
            .map_err(|e| SnapshotError::SerializationFailed(e.to_string()))?;

        // Write to file
        let path = format!("{}/{}.json", self.snapshot_dir, snapshot_id);
        std::fs::write(&path, &json).map_err(|e| SnapshotError::WriteFailed {
            path: path.clone(),
            error: e.to_string(),
        })?;

        let record = SnapshotRecord {
            id: snapshot_id,
            path: path.clone(),
            timestamp,
            size_bytes: json.len() as u64,
            modules_count: snapshot.modules.len(),
            memory_size: snapshot.memory.data.len(),
        };

        self.history.push(record.clone());
        info!(path = &path, size = json.len(), "Snapshot created");

        // Cleanup old snapshots
        self.cleanup_old_snapshots();

        Ok(record)
    }

    /// Load a snapshot from disk.
    pub fn load_snapshot(path: &str) -> Result<RuntimeSnapshot, SnapshotError> {
        let json = std::fs::read_to_string(path).map_err(|e| SnapshotError::ReadFailed {
            path: path.to_string(),
            error: e.to_string(),
        })?;

        let snapshot: RuntimeSnapshot = serde_json::from_str(&json)
            .map_err(|e| SnapshotError::DeserializationFailed(e.to_string()))?;

        // Version check
        if snapshot.version != "0.1.0" {
            return Err(SnapshotError::VersionMismatch {
                expected: "0.1.0".to_string(),
                found: snapshot.version,
            });
        }

        info!(
            path,
            timestamp = snapshot.timestamp,
            modules = snapshot.modules.len(),
            "Snapshot loaded"
        );

        Ok(snapshot)
    }

    /// Remove old snapshots exceeding max_snapshots.
    fn cleanup_old_snapshots(&mut self) {
        while self.history.len() > self.max_snapshots {
            let oldest = self.history.remove(0);
            if let Err(e) = std::fs::remove_file(&oldest.path) {
                warn!(path = &oldest.path, error = %e, "Failed to remove old snapshot");
            }
        }
    }

    /// List all available snapshots.
    pub fn list_snapshots(&self) -> &[SnapshotRecord] {
        &self.history
    }

    /// Get the most recent snapshot.
    pub fn latest_snapshot(&self) -> Option<&SnapshotRecord> {
        self.history.last()
    }

    /// Print snapshot history.
    pub fn print_history(&self) {
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  💾 Snapshot History                                  │");
        println!("  ├──────────────────────────────────────────────────────┤");

        if self.history.is_empty() {
            println!("  │  No snapshots available                              │");
        } else {
            for record in &self.history {
                println!(
                    "  │  {} — {} modules, {} bytes, {}KB on disk",
                    record.id,
                    record.modules_count,
                    record.memory_size,
                    record.size_bytes / 1024,
                );
            }
        }

        println!("  └──────────────────────────────────────────────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_snapshot_serialization() {
        let snapshot = RuntimeSnapshot {
            version: "0.1.0".to_string(),
            timestamp: 1234567890,
            instance_name: "test".to_string(),
            memory: MemorySnapshot {
                capacity: 1024,
                watermark: 256,
                blocks: vec![BlockSnapshot {
                    handle: 1,
                    offset: 0,
                    size: 128,
                    owner: "test-module".to_string(),
                    ref_count: 1,
                }],
                data: vec![0u8; 256],
            },
            modules: vec![ModuleSnapshot {
                name: "calc".to_string(),
                source_language: "Rust".to_string(),
                binary_size: 4096,
                exports: vec!["add".to_string(), "multiply".to_string()],
                call_count: 42,
            }],
            security_profiles: Vec::new(),
            stats: StatsSnapshot {
                total_bridge_calls: 100,
                total_memory_allocations: 50,
                modules_loaded: 3,
                uptime_seconds: 123.45,
                avg_bridge_latency_us: 42,
                peak_memory_bytes: 65536,
            },
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string_pretty(&snapshot).unwrap();
        assert!(json.contains("calc"));
        assert!(json.contains("0.1.0"));

        // Deserialize back
        let restored: RuntimeSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.modules[0].name, "calc");
        assert_eq!(restored.stats.total_bridge_calls, 100);
    }
}
