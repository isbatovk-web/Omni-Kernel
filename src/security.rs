//! # 🔒 Security & Sandboxing Layer
//!
//! The Security module enforces isolation, access control, and resource limits
//! for WASM modules running inside Omni-Kernel. Every module operates within
//! a configurable security sandbox.
//!
//! ## Security Model
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │                 SECURITY BOUNDARY                    │
//! │  ┌───────────────┐  ┌────────────────────────────┐  │
//! │  │   Policy       │  │   Capabilities              │  │
//! │  │   Engine       │  │   Manager                   │  │
//! │  └───────┬───────┘  └──────────┬─────────────────┘  │
//! │          │                      │                    │
//! │  ┌───────▼──────────────────────▼─────────────────┐  │
//! │  │           Access Control Matrix                 │  │
//! │  │  Module → [Capabilities, Resource Limits]       │  │
//! │  └─────────────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────────┘
//! ```

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use thiserror::Error;
use tracing::{debug, info, warn};

/// Security-related errors.
#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Permission denied: module '{module}' lacks capability '{capability}'")]
    PermissionDenied {
        module: String,
        capability: String,
    },

    #[error("Resource limit exceeded: module '{module}' exceeded {resource} limit ({current}/{limit})")]
    ResourceLimitExceeded {
        module: String,
        resource: String,
        current: u64,
        limit: u64,
    },

    #[error("Cross-module call denied: '{caller}' → '{callee}' is not in allowed pairs")]
    CrossModuleCallDenied { caller: String, callee: String },

    #[error("Module '{0}' is not registered with the security manager")]
    ModuleNotRegistered(String),

    #[error("Module '{0}' has been suspended due to security violations")]
    ModuleSuspended(String),

    #[error("Invalid capability: '{0}'")]
    InvalidCapability(String),

    #[error("Rate limit exceeded: module '{module}' exceeded {limit} calls per {window:?}")]
    RateLimitExceeded {
        module: String,
        limit: u64,
        window: Duration,
    },
}

/// Capabilities that can be granted to WASM modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Can allocate memory in the shared pool.
    MemoryAlloc,
    /// Can read from the shared memory pool.
    MemoryRead,
    /// Can write to the shared memory pool.
    MemoryWrite,
    /// Can call functions in other modules via the bridge.
    BridgeCall,
    /// Can register exported functions.
    BridgeExport,
    /// Can access WASI filesystem.
    FileSystem,
    /// Can access WASI networking.
    Network,
    /// Can access WASI environment variables.
    Environment,
    /// Can spawn child tasks in the scheduler.
    Scheduling,
    /// Can access system clock.
    Clock,
    /// Can log messages.
    Logging,
    /// Custom capability.
    Custom(String),
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Capability::MemoryAlloc => write!(f, "memory:alloc"),
            Capability::MemoryRead => write!(f, "memory:read"),
            Capability::MemoryWrite => write!(f, "memory:write"),
            Capability::BridgeCall => write!(f, "bridge:call"),
            Capability::BridgeExport => write!(f, "bridge:export"),
            Capability::FileSystem => write!(f, "wasi:filesystem"),
            Capability::Network => write!(f, "wasi:network"),
            Capability::Environment => write!(f, "wasi:environment"),
            Capability::Scheduling => write!(f, "scheduler:spawn"),
            Capability::Clock => write!(f, "wasi:clock"),
            Capability::Logging => write!(f, "host:logging"),
            Capability::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Resource limits for a module.
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum total memory the module can allocate (bytes).
    pub max_memory_bytes: u64,
    /// Maximum number of active memory blocks.
    pub max_memory_blocks: u64,
    /// Maximum execution time per function call.
    pub max_execution_time: Duration,
    /// Maximum WASM fuel per call.
    pub max_fuel_per_call: u64,
    /// Maximum number of bridge calls per second (rate limiting).
    pub max_bridge_calls_per_sec: u64,
    /// Maximum size of arguments in bridge calls (bytes).
    pub max_bridge_args_size: u64,
    /// Maximum call stack depth for nested bridge calls.
    pub max_call_depth: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 64 * 1024 * 1024, // 64 MB
            max_memory_blocks: 4096,
            max_execution_time: Duration::from_secs(30),
            max_fuel_per_call: 10_000_000,
            max_bridge_calls_per_sec: 10_000,
            max_bridge_args_size: 4 * 1024 * 1024, // 4 MB
            max_call_depth: 64,
        }
    }
}

/// Current resource usage for a module.
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// Current allocated memory (bytes).
    pub memory_bytes: u64,
    /// Current number of active memory blocks.
    pub memory_blocks: u64,
    /// Bridge calls in the current rate-limit window.
    pub bridge_calls_in_window: u64,
    /// When the current rate-limit window started.
    pub window_start: Instant,
    /// Current call stack depth.
    pub current_call_depth: u32,
    /// Total security violations.
    pub violation_count: u64,
}

impl ResourceUsage {
    fn new() -> Self {
        Self {
            memory_bytes: 0,
            memory_blocks: 0,
            bridge_calls_in_window: 0,
            window_start: Instant::now(),
            current_call_depth: 0,
            violation_count: 0,
        }
    }
}

/// Security profile for a module — combines capabilities and limits.
#[derive(Debug)]
pub struct SecurityProfile {
    /// Module name.
    pub module_name: String,
    /// Granted capabilities.
    pub capabilities: HashSet<Capability>,
    /// Resource limits.
    pub limits: ResourceLimits,
    /// Current resource usage.
    pub usage: ResourceUsage,
    /// Whether this module is suspended.
    pub suspended: bool,
    /// Allowed modules this module can call.
    pub allowed_targets: Option<HashSet<String>>,
    /// Maximum violations before automatic suspension.
    pub max_violations_before_suspend: u64,
}

/// The Security Manager — enforces access control for all modules.
pub struct SecurityManager {
    /// Security profiles for each registered module.
    profiles: HashMap<String, SecurityProfile>,
    /// Global allowed cross-module call pairs (if restrictions enabled).
    global_allowed_pairs: Option<HashSet<(String, String)>>,
    /// Total security violations across all modules.
    total_violations: u64,
    /// Audit log of recent security events.
    audit_log: Vec<SecurityEvent>,
    /// Maximum audit log size.
    max_audit_log: usize,
}

/// A security event for auditing.
#[derive(Debug, Clone)]
pub struct SecurityEvent {
    /// When this event occurred.
    pub timestamp: Instant,
    /// Module involved.
    pub module: String,
    /// Type of event.
    pub event_type: SecurityEventType,
    /// Additional details.
    pub details: String,
}

/// Types of security events.
#[derive(Debug, Clone)]
pub enum SecurityEventType {
    /// A capability check passed.
    CapabilityGranted,
    /// A capability check failed.
    CapabilityDenied,
    /// A resource limit was exceeded.
    ResourceLimitExceeded,
    /// A cross-module call was denied.
    CrossModuleCallDenied,
    /// A module was suspended.
    ModuleSuspended,
    /// A module was registered.
    ModuleRegistered,
    /// A rate limit was hit.
    RateLimitHit,
}

impl SecurityManager {
    /// Create a new security manager.
    pub fn new() -> Self {
        info!("Security Manager initialized");
        Self {
            profiles: HashMap::new(),
            global_allowed_pairs: None,
            total_violations: 0,
            audit_log: Vec::new(),
            max_audit_log: 10_000,
        }
    }

    /// Register a module with a security profile.
    pub fn register_module(
        &mut self,
        module_name: &str,
        capabilities: Vec<Capability>,
        limits: ResourceLimits,
    ) {
        let profile = SecurityProfile {
            module_name: module_name.to_string(),
            capabilities: capabilities.into_iter().collect(),
            limits,
            usage: ResourceUsage::new(),
            suspended: false,
            allowed_targets: None,
            max_violations_before_suspend: 10,
        };

        self.profiles.insert(module_name.to_string(), profile);
        self.log_event(module_name, SecurityEventType::ModuleRegistered, "Module registered with security profile");

        info!(module = module_name, "Module registered with security manager");
    }

    /// Register a module with the default (safe) security profile.
    /// Default profile: memory read/write, bridge calls, logging.
    pub fn register_module_default(&mut self, module_name: &str) {
        let capabilities = vec![
            Capability::MemoryAlloc,
            Capability::MemoryRead,
            Capability::MemoryWrite,
            Capability::BridgeCall,
            Capability::BridgeExport,
            Capability::Logging,
            Capability::Clock,
        ];

        self.register_module(module_name, capabilities, ResourceLimits::default());
    }

    /// Check if a module has a specific capability.
    pub fn check_capability(
        &mut self,
        module: &str,
        capability: &Capability,
    ) -> Result<(), SecurityError> {
        let has_cap = {
            let profile = self
                .profiles
                .get(module)
                .ok_or_else(|| SecurityError::ModuleNotRegistered(module.to_string()))?;

            if profile.suspended {
                return Err(SecurityError::ModuleSuspended(module.to_string()));
            }
            profile.capabilities.contains(capability)
        };

        if has_cap {
            debug!(module, capability = %capability, "Capability check passed");
            Ok(())
        } else {
            let (violations, max_violations) = {
                let profile = self.profiles.get_mut(module).unwrap();
                profile.usage.violation_count += 1;
                (profile.usage.violation_count, profile.max_violations_before_suspend)
            };
            
            self.total_violations += 1;
            self.log_event(
                module,
                SecurityEventType::CapabilityDenied,
                &format!("Denied: {}", capability),
            );

            // Auto-suspend if too many violations
            if violations >= max_violations {
                self.profiles.get_mut(module).unwrap().suspended = true;
                warn!(module, violations, "Module suspended due to repeated security violations");
                self.log_event(module, SecurityEventType::ModuleSuspended, "Auto-suspended");
            }

            Err(SecurityError::PermissionDenied {
                module: module.to_string(),
                capability: capability.to_string(),
            })
        }
    }

    /// Check a memory allocation against resource limits.
    pub fn check_memory_alloc(
        &mut self,
        module: &str,
        size: u64,
    ) -> Result<(), SecurityError> {
        self.check_capability(module, &Capability::MemoryAlloc)?;

        let profile = self.profiles.get(module).unwrap();

        let new_bytes = profile.usage.memory_bytes + size;
        if new_bytes > profile.limits.max_memory_bytes {
            return Err(SecurityError::ResourceLimitExceeded {
                module: module.to_string(),
                resource: "memory_bytes".to_string(),
                current: new_bytes,
                limit: profile.limits.max_memory_bytes,
            });
        }

        let new_blocks = profile.usage.memory_blocks + 1;
        if new_blocks > profile.limits.max_memory_blocks {
            return Err(SecurityError::ResourceLimitExceeded {
                module: module.to_string(),
                resource: "memory_blocks".to_string(),
                current: new_blocks,
                limit: profile.limits.max_memory_blocks,
            });
        }

        Ok(())
    }

    /// Record a memory allocation.
    pub fn record_alloc(&mut self, module: &str, size: u64) {
        if let Some(profile) = self.profiles.get_mut(module) {
            profile.usage.memory_bytes += size;
            profile.usage.memory_blocks += 1;
        }
    }

    /// Record a memory free.
    pub fn record_free(&mut self, module: &str, size: u64) {
        if let Some(profile) = self.profiles.get_mut(module) {
            profile.usage.memory_bytes = profile.usage.memory_bytes.saturating_sub(size);
            profile.usage.memory_blocks = profile.usage.memory_blocks.saturating_sub(1);
        }
    }

    /// Check if a cross-module bridge call is allowed.
    pub fn check_bridge_call(
        &mut self,
        caller: &str,
        callee: &str,
    ) -> Result<(), SecurityError> {
        // Check caller has bridge call capability
        self.check_capability(caller, &Capability::BridgeCall)?;

        // Check rate limits
        let (limit_exceeded, limit_val) = {
            let profile = self.profiles.get_mut(caller).unwrap();
            let now = Instant::now();

            // Reset window if needed
            if now.duration_since(profile.usage.window_start) >= Duration::from_secs(1) {
                profile.usage.bridge_calls_in_window = 0;
                profile.usage.window_start = now;
            }

            profile.usage.bridge_calls_in_window += 1;
            (
                profile.usage.bridge_calls_in_window > profile.limits.max_bridge_calls_per_sec,
                profile.limits.max_bridge_calls_per_sec
            )
        };

        if limit_exceeded {
            self.log_event(caller, SecurityEventType::RateLimitHit, "Bridge call rate limit");
            return Err(SecurityError::RateLimitExceeded {
                module: caller.to_string(),
                limit: limit_val,
                window: Duration::from_secs(1),
            });
        }

        // Check allowed targets
        let allowed = {
            let profile = self.profiles.get(caller).unwrap();
            profile.allowed_targets.clone()
        };

        if let Some(ref allowed_set) = allowed {
            if !allowed_set.contains(callee) {
                self.log_event(
                    caller,
                    SecurityEventType::CrossModuleCallDenied,
                    &format!("Call to '{}' denied", callee),
                );
                return Err(SecurityError::CrossModuleCallDenied {
                    caller: caller.to_string(),
                    callee: callee.to_string(),
                });
            }
        }

        // Check global allowed pairs
        if let Some(ref pairs) = self.global_allowed_pairs {
            let pair = (caller.to_string(), callee.to_string());
            if !pairs.contains(&pair) {
                return Err(SecurityError::CrossModuleCallDenied {
                    caller: caller.to_string(),
                    callee: callee.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Set allowed call targets for a module.
    pub fn set_allowed_targets(&mut self, module: &str, targets: Vec<String>) {
        if let Some(profile) = self.profiles.get_mut(module) {
            profile.allowed_targets = Some(targets.into_iter().collect());
        }
    }

    /// Enable global cross-module call restrictions.
    pub fn set_global_allowed_pairs(&mut self, pairs: Vec<(String, String)>) {
        self.global_allowed_pairs = Some(pairs.into_iter().collect());
    }

    /// Add an event to the audit log.
    fn log_event(&mut self, module: &str, event_type: SecurityEventType, details: &str) {
        let event = SecurityEvent {
            timestamp: Instant::now(),
            module: module.to_string(),
            event_type,
            details: details.to_string(),
        };

        self.audit_log.push(event);

        // Trim log if needed
        if self.audit_log.len() > self.max_audit_log {
            self.audit_log.drain(0..self.max_audit_log / 2);
        }
    }

    /// Get the audit log.
    pub fn audit_log(&self) -> &[SecurityEvent] {
        &self.audit_log
    }

    /// Get total violations count.
    pub fn total_violations(&self) -> u64 {
        self.total_violations
    }

    /// Unsuspend a module.
    pub fn unsuspend(&mut self, module: &str) -> Result<(), SecurityError> {
        let profile = self
            .profiles
            .get_mut(module)
            .ok_or_else(|| SecurityError::ModuleNotRegistered(module.to_string()))?;
        profile.suspended = false;
        profile.usage.violation_count = 0;
        info!(module, "Module unsuspended");
        Ok(())
    }

    /// Print security status for all modules.
    pub fn print_status(&self) {
        println!("  ┌──────────────────────────────────────────────────────┐");
        println!("  │  🔒 Security Status                                  │");
        println!("  ├──────────────────────────────────────────────────────┤");

        for (name, profile) in &self.profiles {
            let status = if profile.suspended { "🚫 SUSPENDED" } else { "✅ Active" };
            println!(
                "  │  {} — {} ({} capabilities, {} violations)",
                name,
                status,
                profile.capabilities.len(),
                profile.usage.violation_count,
            );
            println!(
                "  │    Memory: {}/{} bytes, {} blocks",
                profile.usage.memory_bytes,
                profile.limits.max_memory_bytes,
                profile.usage.memory_blocks,
            );
        }

        println!(
            "  │  Total Violations: {}, Audit Log: {} events",
            self.total_violations,
            self.audit_log.len()
        );
        println!("  └──────────────────────────────────────────────────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capabilities() {
        let mut mgr = SecurityManager::new();
        mgr.register_module_default("calc");

        assert!(mgr.check_capability("calc", &Capability::MemoryAlloc).is_ok());
        assert!(mgr.check_capability("calc", &Capability::BridgeCall).is_ok());
        assert!(mgr.check_capability("calc", &Capability::FileSystem).is_err());
        assert!(mgr.check_capability("calc", &Capability::Network).is_err());
    }

    #[test]
    fn test_resource_limits() {
        let mut mgr = SecurityManager::new();
        let mut limits = ResourceLimits::default();
        limits.max_memory_bytes = 1024; // Very small limit

        mgr.register_module("test", vec![Capability::MemoryAlloc], limits);

        assert!(mgr.check_memory_alloc("test", 512).is_ok());
        mgr.record_alloc("test", 512);

        // This should exceed the limit
        assert!(mgr.check_memory_alloc("test", 1024).is_err());
    }

    #[test]
    fn test_cross_module_restrictions() {
        let mut mgr = SecurityManager::new();
        mgr.register_module_default("module-a");
        mgr.register_module_default("module-b");
        mgr.register_module_default("module-c");

        // Restrict module-a to only call module-b
        mgr.set_allowed_targets("module-a", vec!["module-b".to_string()]);

        assert!(mgr.check_bridge_call("module-a", "module-b").is_ok());
        assert!(mgr.check_bridge_call("module-a", "module-c").is_err());
    }

    #[test]
    fn test_auto_suspend() {
        let mut mgr = SecurityManager::new();
        let caps = vec![Capability::MemoryRead]; // No MemoryAlloc
        let mut limits = ResourceLimits::default();
        mgr.register_module("bad-module", caps, limits);

        // Set low violation threshold
        mgr.profiles.get_mut("bad-module").unwrap().max_violations_before_suspend = 3;

        // Keep trying disallowed operations
        for _ in 0..3 {
            let _ = mgr.check_capability("bad-module", &Capability::MemoryAlloc);
        }

        // Module should now be suspended
        assert!(mgr.profiles.get("bad-module").unwrap().suspended);

        // All operations should fail
        assert!(matches!(
            mgr.check_capability("bad-module", &Capability::MemoryRead),
            Err(SecurityError::ModuleSuspended(_))
        ));

        // Unsuspend
        mgr.unsuspend("bad-module").unwrap();
        assert!(mgr.check_capability("bad-module", &Capability::MemoryRead).is_ok());
    }

    #[test]
    fn test_audit_log() {
        let mut mgr = SecurityManager::new();
        mgr.register_module_default("test");

        // This should fail and be logged
        let _ = mgr.check_capability("test", &Capability::Network);

        assert!(!mgr.audit_log().is_empty());
    }
}
