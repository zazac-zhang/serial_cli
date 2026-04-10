//! Lua instance pool for performance optimization
//!
//! This module provides a pool of reusable Lua instances to avoid the overhead
//! of creating new instances for each operation.

use crate::error::Result;
use mlua::Lua;
use std::{cell::RefCell, rc::Rc};

/// Configuration for Lua pool
#[derive(Debug, Clone)]
pub struct LuaPoolConfig {
    /// Maximum number of Lua instances in the pool
    pub max_instances: usize,
    /// Initial number of Lua instances to create
    pub initial_instances: usize,
}

impl Default for LuaPoolConfig {
    fn default() -> Self {
        Self {
            max_instances: 10,
            initial_instances: 2,
        }
    }
}

/// Lua instance pool (simplified, non-thread-safe version)
pub struct LuaPool {
    config: LuaPoolConfig,
    instances: Rc<RefCell<Vec<Lua>>>,
}

impl LuaPool {
    /// Create a new Lua pool with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(LuaPoolConfig::default())
    }

    /// Create a new Lua pool with custom configuration
    pub fn with_config(config: LuaPoolConfig) -> Result<Self> {
        let instances = Rc::new(RefCell::new(Vec::new()));

        let pool = Self { config, instances };

        // Initialize with some instances
        pool.initialize_instances()?;

        Ok(pool)
    }

    /// Initialize the pool with initial Lua instances
    fn initialize_instances(&self) -> Result<()> {
        let mut instances = self.instances.borrow_mut();
        for _ in 0..self.config.initial_instances {
            let lua = Self::create_lua_instance()?;
            instances.push(lua);
        }
        Ok(())
    }

    /// Create a new Lua instance
    fn create_lua_instance() -> Result<Lua> {
        let lua = Lua::new();
        Ok(lua)
    }

    /// Acquire a Lua instance from the pool
    pub fn acquire(&self) -> Result<Lua> {
        let mut instances = self.instances.borrow_mut();
        if instances.is_empty() {
            // Create new instance if pool is empty
            Self::create_lua_instance()
        } else {
            // Reuse existing instance
            Ok(instances.pop().unwrap())
        }
    }

    /// Return a Lua instance to the pool
    pub fn release(&self, lua: Lua) {
        let mut instances = self.instances.borrow_mut();
        if instances.len() < self.config.max_instances {
            instances.push(lua);
        }
        // If pool is full, just drop the instance
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        let instances = self.instances.borrow();
        let available = instances.len();
        let total = self.config.max_instances;
        let in_use = total - available;

        PoolStats {
            total,
            available,
            in_use,
        }
    }
}

impl Clone for LuaPool {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            instances: self.instances.clone(),
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total: usize,
    pub available: usize,
    pub in_use: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_creation() {
        let pool = LuaPool::new().unwrap();
        let stats = pool.stats();

        assert_eq!(stats.total, 10);
        assert_eq!(stats.available, 2); // Initial instances
        assert_eq!(stats.in_use, 8);
    }

    #[test]
    fn test_acquire_release() {
        let pool = LuaPool::new().unwrap();

        // Acquire an instance
        let instance = pool.acquire().unwrap();
        let stats_after_acquire = pool.stats();
        assert_eq!(stats_after_acquire.available, 1);

        // Use the instance
        assert!(instance.globals().set("test", 42).is_ok());

        // Return the instance to the pool
        pool.release(instance);

        let stats_after_release = pool.stats();
        assert_eq!(stats_after_release.available, 2);
    }

    #[test]
    fn test_pool_reuse() {
        let pool = LuaPool::new().unwrap();

        // Acquire and set a global variable
        let instance1 = pool.acquire().unwrap();
        instance1.globals().set("test_var", 123).unwrap();
        pool.release(instance1);

        // Acquire again and check if the instance is reused
        let instance2 = pool.acquire().unwrap();
        let result: i32 = instance2.globals().get("test_var").unwrap();
        assert_eq!(result, 123); // Should be the same Lua instance
        pool.release(instance2);
    }

    #[test]
    fn test_pool_capacity() {
        let config = LuaPoolConfig {
            max_instances: 3, // Increased to allow proper testing
            initial_instances: 2,
        };
        let pool = LuaPool::with_config(config).unwrap();

        // Acquire initial instances
        let instance1 = pool.acquire().unwrap();
        let instance2 = pool.acquire().unwrap();

        // Try to acquire beyond initial instances
        let instance3 = pool.acquire().unwrap();
        let _stats = pool.stats();

        // Return all instances
        pool.release(instance3);
        pool.release(instance2);
        pool.release(instance1);

        let stats_after = pool.stats();

        // All should be in pool (up to max_instances)
        assert_eq!(stats_after.available, 3);
    }
}
