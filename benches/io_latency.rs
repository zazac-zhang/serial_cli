//! I/O latency benchmarks (SIMULATION)
//!
//! **NOTE:** These are simulated benchmarks that measure timing framework
//! overhead rather than actual serial port I/O. They validate the benchmark
//! structure and provide baseline measurements for future real I/O integration.
//!
//! ## What is being measured
//!
//! These benchmarks measure the overhead of the Criterion benchmarking framework
//! and basic timing operations, **NOT** actual serial port I/O performance. The
//! operations are simulated using `black_box` to prevent compiler optimization.
//!
//! ## For real I/O benchmarks
//!
//! To measure actual serial port latency, these benchmarks would need to:
//! - Open real or virtual serial port devices (e.g., `/dev/ttyUSB0`, `COM1`)
//! - Perform actual hardware write/read operations
//! - Measure timing at the hardware/driver level
//! - Account for baud rate, flow control, and signal latency
//!
//! ## Current value
//!
//! Despite being simulated, these benchmarks are valuable for:
//! - Validating the benchmark framework structure
//! - Establishing baseline timing methodology
//! - Providing template for future real I/O integration
//! - Testing CI/CD benchmark pipeline infrastructure

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Instant;

// Import the common module (sibling directory)
mod common;

// Use the data_generator function from the common module
use common::data_generator::generate_random_data;

/// Benchmark write operation latency (SIMULATED)
///
/// **SIMULATION**: Measures the timing overhead of capturing write latency,
/// not actual serial port write operations. Real writes would involve
/// hardware driver calls and baud-rate-dependent transmission.
fn bench_write_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_latency");

    // Test single-byte write latency (simulated timing measurement)
    group.bench_function("single_byte", |b| {
        b.iter(|| {
            let start = Instant::now();
            let data = black_box(&[0u8]);
            let _ = black_box(data.len());
            start.elapsed()
        });
    });

    // Test buffered write latency (simulated timing for different buffer sizes)
    for size in [64, 256, 1024].iter() {
        group.bench_with_input(BenchmarkId::new("buffered", size), size, |b, &size| {
            let data = generate_random_data(size);
            b.iter(|| {
                let start = Instant::now();
                let _ = black_box(&data);
                start.elapsed()
            });
        });
    }

    group.finish();
}

/// Benchmark read operation latency (SIMULATED)
///
/// **SIMULATION**: Measures the timing overhead of capturing read latency,
/// not actual serial port read operations. Real reads would involve
/// polling hardware buffers and driver-level data retrieval.
fn bench_read_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_latency");

    for size in [64, 256, 1024].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_random_data(size);

            b.iter(|| {
                let start = Instant::now();
                // Simulate reading available data (timing overhead only)
                let _ = black_box(&data).len();
                start.elapsed()
            });
        });
    }

    group.finish();
}

/// Benchmark round-trip time (RTT) (SIMULATED)
///
/// **SIMULATION**: Measures basic timing overhead, not actual RTT.
/// Real RTT would include:
/// - Transmission time at configured baud rate
/// - Hardware signal propagation
/// - Remote device processing time
/// - Reception time
fn bench_round_trip_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip_time");

    for size in [32, 64, 128, 256, 512].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_random_data(size);

            b.iter(|| {
                let start = Instant::now();
                // Simulate: send -> wait for echo -> receive (timing overhead only)
                let _ = black_box(&data);
                let elapsed = start.elapsed();
                elapsed.as_micros()
            });
        });
    }

    group.finish();
}

/// Benchmark port open/close latency (SIMULATED)
///
/// **SIMULATION**: Measures timing framework overhead only.
/// Real port operations would involve:
/// - Opening device file or system handle
/// - Configuring baud rate, parity, stop bits
/// - Hardware driver initialization
/// - Resource cleanup and handle release
fn bench_port_operations_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("port_operations_latency");

    group.bench_function("open_close", |b| {
        b.iter(|| {
            let start = Instant::now();
            // Simulate port open (timing overhead only)
            // Simulate port close (timing overhead only)
            let elapsed = start.elapsed();
            elapsed.as_millis()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_write_latency,
    bench_read_latency,
    bench_round_trip_time,
    bench_port_operations_latency
);
criterion_main!(benches);
