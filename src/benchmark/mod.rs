//! Benchmark module for performance testing and optimization
//!
//! This module provides benchmarking utilities for measuring
//! and optimizing serial CLI performance.

pub mod runner;
pub mod reporter;

pub use runner::BenchmarkRunner;
pub use reporter::{BenchmarkReport, ComparisonResult};

/// Benchmark categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BenchmarkCategory {
    /// Serial I/O throughput benchmarks
    SerialIo,
    /// Virtual port performance benchmarks
    VirtualPort,
    /// Protocol parsing and processing benchmarks
    Protocol,
    /// Startup time benchmarks
    Startup,
    /// Memory usage benchmarks
    Memory,
    /// Concurrency benchmarks
    Concurrency,
}

impl BenchmarkCategory {
    pub fn all() -> Vec<Self> {
        vec![
            Self::SerialIo,
            Self::VirtualPort,
            Self::Protocol,
            Self::Startup,
            Self::Memory,
            Self::Concurrency,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::SerialIo => "serial-io",
            Self::VirtualPort => "virtual-port",
            Self::Protocol => "protocol",
            Self::Startup => "startup",
            Self::Memory => "memory",
            Self::Concurrency => "concurrency",
        }
    }
}

/// Benchmark result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub category: BenchmarkCategory,
    pub iterations: u64,
    pub elapsed_ns: u64,
    pub bytes_processed: Option<u64>,
}

impl BenchmarkResult {
    pub fn throughput_bytes_per_sec(&self) -> Option<f64> {
        self.bytes_processed.map(|bytes| {
            let elapsed_sec = self.elapsed_ns as f64 / 1_000_000_000.0;
            bytes as f64 / elapsed_sec
        })
    }

    pub fn avg_ns_per_iteration(&self) -> f64 {
        self.elapsed_ns as f64 / self.iterations as f64
    }
}
