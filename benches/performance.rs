//! Performance benchmarks for Serial CLI
//!
//! This module contains criterion benchmarks to measure and track performance improvements.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serial_cli::lua::{ScriptCache, ScriptEngine};
use serial_cli::protocol::{Protocol, ProtocolRegistry, registry::SimpleProtocolFactory};
use serial_cli::protocol::built_in::LineProtocol;
use serial_cli::task::{Task, TaskPriority, TaskType};
use serial_cli::task::executor::TaskExecutor;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Benchmark Lua script execution with different complexities
fn bench_lua_script_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("lua_script_execution");

    // Simple script
    let simple_script = "return 42";
    group.bench_function("simple_script", |b| {
        b.iter(|| {
            let engine = ScriptEngine::new().unwrap();
            black_box(engine.execute_string(simple_script))
        })
    });

    // Medium complexity script
    let medium_script = r#"
        local function add(a, b)
            return a + b
        end
        return add(5, 3)
    "#;

    group.bench_function("medium_script", |b| {
        b.iter(|| {
            let engine = ScriptEngine::new().unwrap();
            black_box(engine.execute_string(medium_script))
        })
    });

    // Complex script with loops
    let complex_script = r#"
        local sum = 0
        for i = 1, 100 do
            sum = sum + i
        end
        return sum
    "#;

    group.bench_function("complex_script", |b| {
        b.iter(|| {
            let engine = ScriptEngine::new().unwrap();
            black_box(engine.execute_string(complex_script))
        })
    });

    group.finish();
}

/// Benchmark Lua script cache performance
fn bench_lua_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("lua_cache");

    let cache = ScriptCache::new();
    let script = "return 42";

    // Cache miss (first load)
    group.bench_function("cache_miss", |b| {
        b.iter(|| {
            let cache = ScriptCache::new();
            black_box(cache.load_script("test".to_string(), script).unwrap())
        })
    });

    // Cache hit (subsequent loads)
    group.bench_function("cache_hit", |b| {
        let _ = cache.load_script("warmup".to_string(), script);
        b.iter(|| {
            black_box(cache.get_script("warmup").unwrap())
        })
    });

    // Cache operations
    group.bench_function("cache_operations", |b| {
        b.iter(|| {
            // Insert
            for i in 0..10 {
                let script = format!("return {}", i);
                let _ = cache.load_script(format!("key_{}", i), &script).unwrap();
            }

            // Lookup
            for i in 0..10 {
                let _ = cache.get_script(&format!("key_{}", i));
            }

            // Invalidate
            cache.invalidate("key_5");
        })
    });

    group.finish();
}

/// Benchmark protocol operations
fn bench_protocol_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("protocol_operations");

    // Line protocol encoding
    group.bench_function("line_protocol_encode", |b| {
        let mut protocol = LineProtocol::new();
        let data = b"Hello, World!";

        b.iter(|| {
            black_box(protocol.encode(data).unwrap())
        });
    });

    // Line protocol parsing
    group.bench_function("line_protocol_parse", |b| {
        let mut protocol = LineProtocol::new();
        let data = b"Hello, World!\n";

        b.iter(|| {
            black_box(protocol.parse(data).unwrap())
        });
    });

    // Protocol registration
    group.bench_function("protocol_registration", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let mut registry = ProtocolRegistry::new();

                for i in 0..5 {
                    let factory = Arc::new(SimpleProtocolFactory::new(
                        format!("bench_proto_{}", i),
                        "Benchmark protocol".to_string(),
                        LineProtocol::new,
                    ));

                    black_box(registry.register(factory).await);
                }
            });
        });
    });

    // Protocol lookup
    group.bench_function("protocol_lookup", |b| {
        let rt = Runtime::new().unwrap();
        let mut registry = ProtocolRegistry::new();

        rt.block_on(async {
            // Register some protocols
            for i in 0..5 {
                let factory = Arc::new(SimpleProtocolFactory::new(
                    format!("lookup_proto_{}", i),
                    "Lookup protocol".to_string(),
                    LineProtocol::new,
                ));

                registry.register(factory).await;
            }

            b.iter(|| {
                black_box(registry.is_registered("lookup_proto_2"))
            });
        });
    });

    group.finish();
}

/// Benchmark task executor performance
fn bench_task_executor(c: &mut Criterion) {
    let mut group = c.benchmark_group("task_executor");

    // Task creation overhead
    group.bench_function("task_creation", |b| {
        b.iter(|| {
            let task = Task::new(TaskType::Custom {
                name: "bench_task".to_string(),
                data: "bench_data".to_string(),
            });
            black_box(task)
        });
    });

    // Task submission
    group.bench_function("task_submission", |b| {
        let rt = Runtime::new().unwrap();
        let executor = Arc::new(TaskExecutor::new(10));

        b.iter(|| {
            rt.block_on(async {
                let task = Task::new(TaskType::Custom {
                    name: "submit_task".to_string(),
                    data: "submit_data".to_string(),
                });

                let executor_clone = executor.clone();
                let handle = async move {
                    executor_clone.submit(task, TaskPriority::Normal).await
                };
                black_box(handle.await.unwrap())
            });
        });
    });

    // Task throughput
    group.bench_function("task_throughput", |b| {
        let rt = Runtime::new().unwrap();
        let executor = Arc::new(TaskExecutor::new(50));

        b.iter(|| {
            rt.block_on(async {
                for i in 0..5 {
                    let task = Task::new(TaskType::Custom {
                        name: format!("task_{}", i),
                        data: "data".to_string(),
                    });

                    let _ = executor.submit(task, TaskPriority::Normal).await;
                }
            });
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_lua_script_execution,
    bench_lua_cache,
    bench_protocol_operations,
    bench_task_executor
);

criterion_main!(benches);
