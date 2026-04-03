//! Lua script execution benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use serial_cli::lua::LuaEngine;

mod common;

/// Benchmark script execution overhead
fn bench_script_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("script_overhead");

    // Empty script baseline
    group.bench_function("empty", |b| {
        let engine = LuaEngine::new().unwrap();

        b.iter(|| {
            let lua = engine.lua();
            black_box(lua.load("").exec()).unwrap()
        });
    });

    // Simple script
    group.bench_function("simple", |b| {
        let engine = LuaEngine::new().unwrap();

        b.iter(|| {
            let lua = engine.lua();
            black_box(lua.load("return 1 + 1").exec()).unwrap()
        });
    });

    // Complex calculation
    group.bench_function("complex", |b| {
        let engine = LuaEngine::new().unwrap();
        let script = r#"
            local function fibonacci(n)
                if n <= 1 then return n end
                return fibonacci(n - 1) + fibonacci(n - 2)
            end
            return fibonacci(10)
        "#;

        b.iter(|| {
            let lua = engine.lua();
            black_box(lua.load(script).exec()).unwrap()
        });
    });

    group.finish();
}

/// Benchmark data transformation in Lua
fn bench_data_transformation(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_transformation");

    for size in [64, 256, 1024].iter() {
        group.bench_with_input(BenchmarkId::new("hex_encode", size), size, |b, &size| {
            let engine = LuaEngine::new().unwrap();
            let _data = common::generate_random_data(size);
            let script = r#"
                local data = "test data for benchmarking"
                local hex = ""
                for i = 1, #data do
                    local byte = string.byte(data, i)
                    hex = hex .. string.format("%02X", byte)
                end
                return hex
            "#;

            b.iter(|| {
                let lua = engine.lua();
                black_box(lua.load(script).exec()).unwrap()
            });
        });
    }

    group.finish();
}

/// Benchmark callback overhead
fn bench_callback_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("callback_overhead");

    // Data callback
    group.bench_function("on_data", |b| {
        let engine = LuaEngine::new().unwrap();
        let script = r#"
            function on_data(data)
                -- Process incoming data
                return #data
            end
        "#;

        b.iter(|| {
            let lua = engine.lua();
            black_box(lua.load(script).exec()).unwrap()
        });
    });

    // Error callback
    group.bench_function("on_error", |b| {
        let engine = LuaEngine::new().unwrap();
        let script = r#"
            function on_error(err)
                -- Handle error
                return err
            end
        "#;

        b.iter(|| {
            let lua = engine.lua();
            black_box(lua.load(script).exec()).unwrap()
        });
    });

    group.finish();
}

/// Benchmark script loading time
fn bench_script_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("script_loading");

    // Small script
    group.bench_function("small", |b| {
        let script = "return 1 + 1";

        b.iter(|| {
            let engine = LuaEngine::new().unwrap();
            let lua = engine.lua();
            black_box(lua.load(black_box(script)).exec()).unwrap()
        });
    });

    // Medium script
    group.bench_function("medium", |b| {
        let script = r#"
            local function helper1(x) return x * 2 end
            local function helper2(x) return x + 10 end
            return helper2(helper1(5))
        "#;

        b.iter(|| {
            let engine = LuaEngine::new().unwrap();
            let lua = engine.lua();
            black_box(lua.load(black_box(script)).exec()).unwrap()
        });
    });

    // Large script
    group.bench_function("large", |b| {
        let script: String = (0..100).map(|i| format!("local func{} = function() return {} end\n", i, i)).collect();

        b.iter(|| {
            let engine = LuaEngine::new().unwrap();
            let lua = engine.lua();
            black_box(lua.load(black_box(&script)).exec()).unwrap()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_script_overhead,
    bench_data_transformation,
    bench_callback_overhead,
    bench_script_loading
);
criterion_main!(benches);
