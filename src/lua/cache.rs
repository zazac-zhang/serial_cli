//! Lua script cache for performance optimization
//!
//! This module provides caching of Lua scripts to avoid repeated file I/O and parsing.

use crate::error::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// Script cache entry
#[derive(Clone)]
struct CachedScript {
    /// Script content
    content: String,
    /// Script modification time for cache invalidation
    modified_time: Option<std::time::SystemTime>,
    /// Source file path (if loaded from file)
    #[allow(dead_code)]
    source_path: Option<PathBuf>,
}

/// Lua script cache
pub struct ScriptCache {
    /// Cache storage
    cache: Arc<RwLock<HashMap<String, CachedScript>>>,
    /// Maximum cache size
    max_entries: usize,
}

impl ScriptCache {
    /// Create a new script cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_entries: 100,
        }
    }

    /// Create a new script cache with custom size
    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_entries,
        }
    }

    /// Load and cache a script from a string
    pub fn load_script(&self, name: String, script: &str) -> Result<String> {
        // Check if already cached
        {
            let cache = self.cache.read().unwrap();
            if let Some(cached) = cache.get(&name) {
                tracing::debug!("Script cache hit for: {}", name);
                return Ok(cached.content.clone());
            }
        }

        // Store in cache
        {
            let mut cache = self.cache.write().unwrap();

            // Evict oldest entries if cache is full
            // Note: HashMap iteration order is non-deterministic, so eviction is effectively random
            // For production use with predictable eviction, consider using an LRU cache library
            if cache.len() >= self.max_entries {
                let key_to_remove = cache.keys().next().cloned();
                if let Some(key) = key_to_remove {
                    cache.remove(&key);
                    tracing::debug!("Evicted script from cache: {}", key);
                }
            }

            cache.insert(name.clone(), CachedScript {
                content: script.to_string(),
                modified_time: None,
                source_path: None,
            });

            tracing::debug!("Script cache miss for: {}, cached content", name);
        }

        Ok(script.to_string())
    }

    /// Load and cache a script from a file
    pub fn load_script_from_file<P: AsRef<Path>>(&self, name: String, path: P) -> Result<String> {
        let path_ref = path.as_ref();

        // Get file modification time
        let metadata = std::fs::metadata(path_ref).map_err(|e| {
            crate::error::SerialError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Script file not found: {}", e),
            ))
        })?;

        let modified_time = metadata.modified().ok();

        // Check cache with modification time validation
        {
            let cache = self.cache.read().unwrap();
            if let Some(cached) = cache.get(&name) {
                // Check if cached version is still valid
                if cached.modified_time.is_none() || cached.modified_time == modified_time {
                    tracing::debug!("Script file cache hit for: {}", name);
                    return Ok(cached.content.clone());
                } else {
                    tracing::debug!("Script file cache stale for: {}, reloading", name);
                }
            }
        }

        // Read script from file
        let script_content = std::fs::read_to_string(path_ref).map_err(|e| {
            crate::error::SerialError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to read script file: {}", e),
            ))
        })?;

        // Cache the script content
        {
            let mut cache = self.cache.write().unwrap();

            // Evict if necessary
            // Note: HashMap iteration order is non-deterministic, so eviction is effectively random
            if cache.len() >= self.max_entries {
                let key_to_remove = cache.keys().next().cloned();
                if let Some(key) = key_to_remove {
                    cache.remove(&key);
                }
            }

            cache.insert(name.clone(), CachedScript {
                content: script_content.clone(),
                modified_time,
                source_path: Some(path_ref.to_path_buf()),
            });

            tracing::debug!("Script file cached: {}", name);
        }

        Ok(script_content)
    }

    /// Get a script from cache, returns None if not cached
    pub fn get_script(&self, name: &str) -> Option<String> {
        let cache = self.cache.read().unwrap();
        cache.get(name).map(|cached| cached.content.clone())
    }

    /// Invalidate a cached script
    pub fn invalidate(&self, name: &str) {
        let mut cache = self.cache.write().unwrap();
        if cache.remove(name).is_some() {
            tracing::debug!("Invalidated script cache entry: {}", name);
        }
    }

    /// Clear all cached scripts
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        let count = cache.len();
        cache.clear();
        tracing::debug!("Cleared {} script cache entries", count);
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.read().unwrap();
        CacheStats {
            entries: cache.len(),
            max_entries: self.max_entries,
        }
    }
}

impl Clone for ScriptCache {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            max_entries: self.max_entries,
        }
    }
}

impl Default for ScriptCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_creation() {
        let cache = ScriptCache::new();
        let stats = cache.stats();
        assert_eq!(stats.entries, 0);
        assert_eq!(stats.max_entries, 100);
    }

    #[test]
    fn test_cache_script_string() {
        let cache = ScriptCache::new();

        // Load a simple script
        let script = r#"
            function add(a, b)
                return a + b
            end
            return add(5, 3)
        "#;

        let result1 = cache.load_script("test_script".to_string(), script).unwrap();
        assert!(result1.contains("add"));

        // Second load should hit cache
        let result2 = cache.load_script("test_script".to_string(), script).unwrap();
        assert_eq!(result1, result2);

        let stats = cache.stats();
        assert_eq!(stats.entries, 1); // Only one entry
    }

    #[test]
    fn test_cache_get_script() {
        let cache = ScriptCache::new();

        // Load a script
        let script = "return 42";
        cache.load_script("get_test".to_string(), script).unwrap();

        // Get cached script
        let cached = cache.get_script("get_test");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap(), script);

        // Try to get non-existent script
        let not_cached = cache.get_script("nonexistent");
        assert!(not_cached.is_none());
    }

    #[test]
    fn test_cache_invalidation() {
        let cache = ScriptCache::new();

        // Load a script
        let script = "return 42";
        let _ = cache.load_script("temp_script".to_string(), script).unwrap();

        assert_eq!(cache.stats().entries, 1);

        // Invalidate
        cache.invalidate("temp_script");

        assert_eq!(cache.stats().entries, 0);
    }

    #[test]
    fn test_cache_clear() {
        let cache = ScriptCache::new();

        // Load multiple scripts
        for i in 0..5 {
            let script = format!("return {}", i);
            let _ = cache.load_script(format!("script_{}", i), &script).unwrap();
        }

        assert_eq!(cache.stats().entries, 5);

        // Clear all
        cache.clear();

        assert_eq!(cache.stats().entries, 0);
    }

    #[test]
    fn test_cache_max_entries() {
        let cache = ScriptCache::with_max_entries(3);

        // Load more scripts than max entries
        for i in 0..5 {
            let script = format!("return {}", i);
            let _ = cache.load_script(format!("script_{}", i), &script).unwrap();
        }

        let stats = cache.stats();
        assert_eq!(stats.entries, 3); // Should not exceed max
    }

    #[test]
    fn test_cache_script_from_file() {
        use tempfile::NamedTempFile;
        use std::io::Write;

        let cache = ScriptCache::new();

        // Create a temporary script file
        let script_content = r#"
            function multiply(a, b)
                return a * b
            end
            return multiply(3, 4)
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(script_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        // Load from file
        let result = cache.load_script_from_file("file_script".to_string(), temp_file.path()).unwrap();
        assert!(result.contains("multiply"));

        // Should be cached
        assert_eq!(cache.stats().entries, 1);

        // Second load should hit cache
        let result2 = cache.load_script_from_file("file_script".to_string(), temp_file.path()).unwrap();
        assert_eq!(result, result2);

        // Still only one entry
        assert_eq!(cache.stats().entries, 1);
    }

    #[test]
    fn test_cache_clone() {
        let cache1 = ScriptCache::new();

        // Load a script
        let script = "return 42";
        let _ = cache1.load_script("clone_test".to_string(), script).unwrap();

        // Clone cache
        let cache2 = cache1.clone();

        // Both should see the cached entry
        assert_eq!(cache2.stats().entries, 1);

        // Load from cloned cache should hit cache
        let result = cache2.load_script("clone_test".to_string(), script).unwrap();
        assert_eq!(result, script);
    }
}
