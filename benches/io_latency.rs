//! I/O latency benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Instant;

// Import the common module (sibling directory)
mod common;

// Use the data_generator function from the common module
use common::data_generator::generate_random_data;

/// Benchmark write operation latency
fn bench_write_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_latency");

    // Test single-byte write latency
    group.bench_function("single_byte", |b| {
        b.iter(|| {
            let start = Instant::now();
            let data = black_box(&[0u8]);
            let _ = black_box(data.len());
            start.elapsed()
        });
    });

    // Test buffered write latency
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

/// Benchmark read operation latency
fn bench_read_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_latency");

    for size in [64, 256, 1024].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_random_data(size);

            b.iter(|| {
                let start = Instant::now();
                // Simulate reading available data
                let _ = black_box(&data).len();
                start.elapsed()
            });
        });
    }

    group.finish();
}

/// Benchmark round-trip time (RTT)
fn bench_round_trip_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip_time");

    for size in [32, 64, 128, 256, 512].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_random_data(size);

            b.iter(|| {
                let start = Instant::now();
                // Simulate: send -> wait for echo -> receive
                let _ = black_box(&data);
                let elapsed = start.elapsed();
                elapsed.as_micros()
            });
        });
    }

    group.finish();
}

/// Benchmark port open/close latency
fn bench_port_operations_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("port_operations_latency");

    group.bench_function("open_close", |b| {
        b.iter(|| {
            let start = Instant::now();
            // Simulate port open
            // Simulate port close
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
