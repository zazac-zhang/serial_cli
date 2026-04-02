/// I/O throughput benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

// Re-export the specific function we need from the common module
fn generate_random_data(size: usize) -> Vec<u8> {
    use rand::Rng;
    let mut data = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        data.push(rng.gen());
    }

    data
}

/// Benchmark single port write throughput
fn bench_single_port_write_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_port_write_throughput");

    for size in [64, 256, 1024, 4096, 16384].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_random_data(size);

            b.iter(|| {
                // Simulate write operation timing
                black_box(&data);
                data.len()
            });
        });
    }

    group.finish();
}

/// Benchmark round-trip throughput (echo simulation)
fn bench_round_trip_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip_throughput");

    for size in [64, 256, 1024, 4096].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_random_data(size);

            b.iter(|| {
                // Simulate: write -> read (echo)
                let written = black_box(&data).len();
                let read = black_box(written); // Simulate echo
                written + read
            });
        });
    }

    group.finish();
}

/// Benchmark continuous streaming throughput
fn bench_continuous_stream(c: &mut Criterion) {
    let mut group = c.benchmark_group("continuous_stream");

    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    for chunk_size in [1024, 4096].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(chunk_size), chunk_size, |b, &chunk_size| {
            let data = generate_random_data(chunk_size);

            b.iter(|| {
                // Simulate streaming multiple chunks
                let mut total = 0;
                for _ in 0..100 {
                    total += black_box(&data).len();
                }
                total
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_single_port_write_throughput,
    bench_round_trip_throughput,
    bench_continuous_stream
);
criterion_main!(benches);