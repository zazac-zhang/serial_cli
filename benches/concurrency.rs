//! Concurrent operations benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use tokio::runtime::Runtime;

// Import the common module (sibling directory)
mod common;

// Use the data_generator function from the common module
use common::data_generator::generate_random_data;

/// Benchmark multi-port concurrent reads
fn bench_multi_port_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_port_read");
    let rt = Arc::new(Runtime::new().unwrap());

    for port_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(port_count),
            port_count,
            |b, &port_count| {
                b.iter(|| {
                    let rt = rt.clone();
                    let data = generate_random_data(1024);

                    rt.block_on(async {
                        let mut handles = vec![];
                        for _ in 0..port_count {
                            let data = data.clone();
                            handles
                                .push(tokio::spawn(async move { black_box(&data).len() as usize }));
                        }

                        let mut total = 0usize;
                        for handle in handles {
                            total += handle.await.unwrap();
                        }
                        total
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark multi-port concurrent writes
fn bench_multi_port_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_port_write");
    let rt = Arc::new(Runtime::new().unwrap());

    for port_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(port_count),
            port_count,
            |b, &port_count| {
                b.iter(|| {
                    let rt = rt.clone();
                    let data = generate_random_data(1024);

                    rt.block_on(async {
                        let mut handles = vec![];
                        for _ in 0..port_count {
                            let data = data.clone();
                            handles
                                .push(tokio::spawn(async move { black_box(&data).len() as usize }));
                        }

                        let mut total = 0usize;
                        for handle in handles {
                            total += handle.await.unwrap();
                        }
                        total
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark event dispatch performance
fn bench_event_dispatch(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_dispatch");
    let rt = Arc::new(Runtime::new().unwrap());

    for handler_count in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("handlers", handler_count),
            handler_count,
            |b, &handler_count| {
                b.iter(|| {
                    let rt = rt.clone();

                    rt.block_on(async {
                        let mut handles = vec![];
                        for i in 0..handler_count {
                            handles.push(tokio::spawn(async move { black_box(i) }));
                        }

                        for handle in handles {
                            handle.await.unwrap();
                        }
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark port switching performance
fn bench_port_switching(c: &mut Criterion) {
    let mut group = c.benchmark_group("port_switching");
    let rt = Arc::new(Runtime::new().unwrap());

    group.bench_function("rapid_switch", |b| {
        b.iter(|| {
            let rt = rt.clone();

            rt.block_on(async {
                for i in 0..100 {
                    black_box(i);
                    tokio::task::yield_now().await;
                }
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_multi_port_read,
    bench_multi_port_write,
    bench_event_dispatch,
    bench_port_switching
);
criterion_main!(benches);
