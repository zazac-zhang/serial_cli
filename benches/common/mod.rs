//! Common utilities for benchmarks

pub mod data_generator;
pub mod virtual_serial;

// Re-export commonly used functions for convenience
pub use data_generator::{generate_ascii_data, generate_pattern_data, generate_random_data};

use std::time::Duration;

/// Benchmark configuration
pub struct BenchConfig {
    pub warmup_iters: usize,
    pub measure_iters: usize,
    pub sample_size: usize,
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            warmup_iters: 3,
            measure_iters: 10,
            sample_size: 100,
        }
    }
}

/// Performance metrics collected during benchmarks
#[derive(Debug, Clone)]
pub struct Metrics {
    pub throughput_bytes_per_sec: f64,
    pub latency: Duration,
    pub operation_count: u64,
}
