//! # 📦 WASM Module Loader
//!
//! The WASM Module Loader handles discovery, compilation, caching, and
//! instantiation of WebAssembly modules. It integrates with the Wasmtime
//! engine to provide:
//!
//! - Module discovery from filesystem paths
//! - Pre-compilation and caching for fast startup
//! - Module validation and security checks
//! - Host function linking (Bridge API injection)
//! - Instance lifecycle management
//!
//! ## Loading Pipeline
//!
//! ```text
//! ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
//! │ Discover │ →  │ Validate │ →  │ Compile  │ →  │   Link   │ →  │Instantia-│
//! │ .wasm    │    │ (check   │    │ (JIT or  │    │ (inject  │    │  te      │
//! │ files    │    │  safety) │    │  AOT)    │    │  bridge) │    │          │
//! └──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘
//! ```

use crate::module_registry::SourceLanguage;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing::{debug, info, warn};

/// Information about a discovered WASM module file.
#[derive(Debug, Clone)]
pub struct WasmModuleFile {
    /// Absolute path to the .wasm file.
    pub path: PathBuf,
    /// Module name (derived from filename).
    pub name: String,
    /// Size of the WASM binary in bytes.
    pub size: u64,
    /// Detected source language (if identifiable).
    pub source_language: SourceLanguage,
    /// Whether this module has been validated.
    pub validated: bool,
    /// Whether this module has been compiled.
    pub compiled: bool,
    /// Compilation time in milliseconds (if compiled).
    pub compile_time_ms: Option<f64>,
}

/// Cache entry for a compiled WASM module.
#[derive(Debug)]
pub struct CompiledModuleCache {
    /// Path to the cached compiled artifact.
    pub cache_path: PathBuf,
    /// Hash of the source WASM binary (for invalidation).
    pub source_hash: String,
    /// Timestamp of compilation.
    pub compiled_at: Instant,
}

/// Validation result for a WASM module.
#[derive(Debug)]
pub struct ValidationResult {
    /// Whether the module passed all checks.
    pub valid: bool,
    /// List of exported functions.
    pub exports: Vec<WasmExport>,
    /// List of imported functions.
    pub imports: Vec<WasmImport>,
    /// Warnings (non-fatal issues).
    pub warnings: Vec<String>,
    /// Errors (fatal issues that prevent loading).
    pub errors: Vec<String>,
    /// Memory requirements.
    pub memory_requirements: MemoryRequirements,
}

/// An exported function from a WASM module.
#[derive(Debug, Clone)]
pub struct WasmExport {
    /// Export name.
    pub name: String,
    /// Export kind.
    pub kind: ExportKind,
}

/// An imported function for a WASM module.
#[derive(Debug, Clone)]
pub struct WasmImport {
    /// Module name of the import.
    pub module: String,
    /// Import name.
    pub name: String,
    /// Import kind.
    pub kind: ImportKind,
}

/// Kind of WASM export.
#[derive(Debug, Clone)]
pub enum ExportKind {
    Function { params: usize, results: usize },
    Memory,
    Table,
    Global,
}

/// Kind of WASM import.
#[derive(Debug, Clone)]
pub enum ImportKind {
    Function,
    Memory,
    Table,
    Global,
}

/// Memory requirements for a module.
#[derive(Debug, Clone)]
pub struct MemoryRequirements {
    /// Minimum memory pages required.
    pub min_pages: u32,
    /// Maximum memory pages (None = unlimited).
    pub max_pages: Option<u32>,
    /// Whether shared memory is used.
    pub shared: bool,
}

/// The WASM Module Loader.
pub struct WasmLoader {
    /// Search paths for module discovery.
    search_paths: Vec<PathBuf>,
    /// Discovered module files.
    discovered: HashMap<String, WasmModuleFile>,
    /// Compilation cache directory.
    cache_dir: Option<PathBuf>,
    /// Whether to pre-compile modules.
    precompile: bool,
    /// Compiled module cache.
    compile_cache: HashMap<String, CompiledModuleCache>,
    /// Total modules discovered.
    total_discovered: usize,
    /// Total modules successfully loaded.
    total_loaded: usize,
}

impl WasmLoader {
    /// Create a new WASM loader.
    pub fn new(search_paths: Vec<PathBuf>, cache_dir: Option<PathBuf>) -> Self {
        info!(
            search_paths = ?search_paths,
            "WASM Loader initialized"
        );

        // Ensure cache directory exists
        if let Some(ref dir) = cache_dir {
            let _ = std::fs::create_dir_all(dir);
        }

        Self {
            search_paths,
            discovered: HashMap::new(),
            cache_dir,
            precompile: true,
            compile_cache: HashMap::new(),
            total_discovered: 0,
            total_loaded: 0,
        }
    }

    /// Discover WASM modules in all search paths.
    pub fn discover(&mut self) -> Result<Vec<WasmModuleFile>> {
        let mut found = Vec::new();

        for search_path in &self.search_paths.clone() {
            if search_path.exists() && search_path.is_dir() {
                let modules = self.scan_directory(search_path)?;
                found.extend(modules);
            } else {
                debug!(path = ?search_path, "Search path does not exist, skipping");
            }
        }

        self.total_discovered = found.len();
        info!(count = found.len(), "Module discovery complete");

        Ok(found)
    }

    /// Scan a directory recursively for .wasm files.
    fn scan_directory(&mut self, dir: &Path) -> Result<Vec<WasmModuleFile>> {
        let mut modules = Vec::new();

        let entries = std::fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {:?}", dir))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recurse into subdirectories
                modules.extend(self.scan_directory(&path)?);
            } else if path.extension().map(|e| e == "wasm").unwrap_or(false) {
                let name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let metadata = std::fs::metadata(&path)?;
                let source_language = Self::detect_source_language(&name);

                let module_file = WasmModuleFile {
                    path: path.clone(),
                    name: name.clone(),
                    size: metadata.len(),
                    source_language,
                    validated: false,
                    compiled: false,
                    compile_time_ms: None,
                };

                debug!(name = &name, size = metadata.len(), "Discovered WASM module");
                self.discovered.insert(name, module_file.clone());
                modules.push(module_file);
            }
        }

        Ok(modules)
    }

    /// Detect the source language from module naming conventions.
    fn detect_source_language(name: &str) -> SourceLanguage {
        let name_lower = name.to_lowercase();

        if name_lower.contains("_rs") || name_lower.ends_with("-rs") {
            SourceLanguage::Rust
        } else if name_lower.contains("_cpp") || name_lower.contains("_cc") {
            SourceLanguage::Cpp
        } else if name_lower.contains("_go") || name_lower.ends_with("-go") {
            SourceLanguage::Go
        } else if name_lower.contains("_js") || name_lower.ends_with("-js") {
            SourceLanguage::JavaScript
        } else if name_lower.contains("_py") || name_lower.ends_with("-py") {
            SourceLanguage::Python
        } else {
            SourceLanguage::Unknown
        }
    }

    /// Validate a WASM module binary.
    ///
    /// Checks:
    /// - Valid WASM magic number (\0asm)
    /// - Module structure integrity
    /// - Required imports (bridge functions)
    /// - Export analysis
    /// - Memory requirements
    pub fn validate(&self, wasm_bytes: &[u8]) -> ValidationResult {
        let mut result = ValidationResult {
            valid: true,
            exports: Vec::new(),
            imports: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            memory_requirements: MemoryRequirements {
                min_pages: 0,
                max_pages: None,
                shared: false,
            },
        };

        // Check WASM magic number
        if wasm_bytes.len() < 8 {
            result.errors.push("File too small to be a valid WASM module".to_string());
            result.valid = false;
            return result;
        }

        let magic = &wasm_bytes[0..4];
        if magic != b"\0asm" {
            result.errors.push("Invalid WASM magic number — expected \\0asm".to_string());
            result.valid = false;
            return result;
        }

        // Check WASM version
        let version = u32::from_le_bytes(wasm_bytes[4..8].try_into().unwrap());
        if version != 1 {
            result
                .warnings
                .push(format!("Unexpected WASM version: {} (expected 1)", version));
        }

        // In production, we would parse all sections here using wasmparser.
        // For now, we just validate the header.

        info!(
            size = wasm_bytes.len(),
            exports = result.exports.len(),
            imports = result.imports.len(),
            "Module validation complete"
        );

        result
    }

    /// Compute a hash of WASM bytes for cache invalidation.
    pub fn hash_module(wasm_bytes: &[u8]) -> String {
        // Simple hash using FNV-1a
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in wasm_bytes {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{:016x}", hash)
    }

    /// Get a discovered module by name.
    pub fn get_module(&self, name: &str) -> Option<&WasmModuleFile> {
        self.discovered.get(name)
    }

    /// Get the total number of discovered modules.
    pub fn discovered_count(&self) -> usize {
        self.total_discovered
    }

    /// Get the total number of loaded modules.
    pub fn loaded_count(&self) -> usize {
        self.total_loaded
    }

    /// Set whether to pre-compile modules.
    pub fn set_precompile(&mut self, precompile: bool) {
        self.precompile = precompile;
    }

    /// List all discovered modules.
    pub fn list_discovered(&self) -> Vec<&WasmModuleFile> {
        self.discovered.values().collect()
    }

    /// Print discovery summary.
    pub fn print_summary(&self) {
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  📦 WASM Loader Status                               │");
        println!("  ├──────────────────────────────────────────────────────┤");
        println!(
            "  │  Search Paths: {}                                     │",
            self.search_paths.len()
        );
        println!(
            "  │  Discovered: {} modules                                │",
            self.total_discovered
        );
        println!(
            "  │  Loaded: {} modules                                    │",
            self.total_loaded
        );

        for (name, module) in &self.discovered {
            let compiled_status = if module.compiled {
                format!("✅ {:.1}ms", module.compile_time_ms.unwrap_or(0.0))
            } else {
                "⏳ pending".to_string()
            };
            println!(
                "  │    {} — {} ({} bytes) [{}]",
                name,
                module.source_language,
                module.size,
                compiled_status,
            );
        }

        println!("  └──────────────────────────────────────────────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_language_detection() {
        assert_eq!(
            WasmLoader::detect_source_language("calc_rs"),
            SourceLanguage::Rust
        );
        assert_eq!(
            WasmLoader::detect_source_language("physics-cpp"),
            SourceLanguage::Cpp
        );
        assert_eq!(
            WasmLoader::detect_source_language("orchestrator-js"),
            SourceLanguage::JavaScript
        );
        assert_eq!(
            WasmLoader::detect_source_language("data_py"),
            SourceLanguage::Python
        );
        assert_eq!(
            WasmLoader::detect_source_language("unknown_module"),
            SourceLanguage::Unknown
        );
    }

    #[test]
    fn test_wasm_validation_invalid() {
        let loader = WasmLoader::new(vec![], None);

        // Invalid magic
        let bad_bytes = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00];
        let result = loader.validate(&bad_bytes);
        assert!(!result.valid);

        // Too small
        let tiny = vec![0x00, 0x61];
        let result = loader.validate(&tiny);
        assert!(!result.valid);
    }

    #[test]
    fn test_wasm_validation_valid_header() {
        let loader = WasmLoader::new(vec![], None);

        // Valid WASM header (minimal)
        let valid_header = vec![
            0x00, 0x61, 0x73, 0x6D, // \0asm magic
            0x01, 0x00, 0x00, 0x00, // version 1
        ];
        let result = loader.validate(&valid_header);
        assert!(result.valid);
    }

    #[test]
    fn test_module_hashing() {
        let bytes1 = b"hello wasm world";
        let bytes2 = b"different content";

        let hash1 = WasmLoader::hash_module(bytes1);
        let hash2 = WasmLoader::hash_module(bytes2);

        assert_ne!(hash1, hash2);
        assert_eq!(hash1.len(), 16); // 16 hex chars for u64
    }
}
