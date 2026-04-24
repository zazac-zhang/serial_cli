//! Benchmark runner for executing performance tests
//!
//! Provides utilities for running benchmarks and collecting results.

use super::{BenchmarkCategory, BenchmarkResult};
use crate::config::ConfigManager;
use crate::lua::LuaEngine;
use crate::protocol::built_in::modbus::{ModbusMode, ModbusProtocol};
use crate::protocol::loader::ProtocolLoader;
use crate::protocol::Protocol;
use crate::Result;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

/// Benchmark runner
pub struct BenchmarkRunner {
    warmup_iterations: u64,
    measurement_iterations: u64,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner with default settings
    pub fn new() -> Self {
        Self {
            warmup_iterations: 10,
            measurement_iterations: 100,
        }
    }

    /// Set warmup iterations
    pub fn with_warmup(mut self, iterations: u64) -> Self {
        self.warmup_iterations = iterations;
        self
    }

    /// Set measurement iterations
    pub fn with_iterations(mut self, iterations: u64) -> Self {
        self.measurement_iterations = iterations;
        self
    }

    /// Run a benchmark function
    pub fn run<F>(
        &self,
        name: String,
        category: BenchmarkCategory,
        mut bench_fn: F,
    ) -> Result<BenchmarkResult>
    where
        F: FnMut() -> Result<()>,
    {
        println!("Benchmarking: {} ({})", name, category.name());

        // Warmup
        for _ in 0..self.warmup_iterations {
            bench_fn()?;
        }

        // Measurement
        let start = Instant::now();
        for _ in 0..self.measurement_iterations {
            bench_fn()?;
        }
        let elapsed = start.elapsed();

        let result = BenchmarkResult {
            name,
            category,
            iterations: self.measurement_iterations,
            elapsed_ns: elapsed.as_nanos() as u64,
            bytes_processed: None,
        };

        println!("  {:>10} iter in {:>8.2?}", result.iterations, elapsed);
        println!("  {:>10.3} ns/iter", result.avg_ns_per_iteration());

        Ok(result)
    }

    /// Run a throughput benchmark (bytes processed)
    pub fn run_throughput<F>(
        &self,
        name: String,
        category: BenchmarkCategory,
        bytes_per_iteration: u64,
        mut bench_fn: F,
    ) -> Result<BenchmarkResult>
    where
        F: FnMut() -> Result<()>,
    {
        println!("Benchmarking: {} ({})", name, category.name());

        // Warmup
        for _ in 0..self.warmup_iterations {
            bench_fn()?;
        }

        // Measurement
        let start = Instant::now();
        for _ in 0..self.measurement_iterations {
            bench_fn()?;
        }
        let elapsed = start.elapsed();

        let total_bytes = bytes_per_iteration * self.measurement_iterations;

        let result = BenchmarkResult {
            name,
            category,
            iterations: self.measurement_iterations,
            elapsed_ns: elapsed.as_nanos() as u64,
            bytes_processed: Some(total_bytes),
        };

        println!("  {:>10} iter in {:>8.2?}", result.iterations, elapsed);
        println!("  {:>10.3} MB/s", result.throughput_bytes_per_sec().unwrap() / 1_000_000.0);

        Ok(result)
    }

    /// Run all serial I/O benchmarks
    pub fn run_serial_io_benchmarks(&self) -> Result<Vec<BenchmarkResult>> {
        println!("\n=== Serial I/O Benchmarks ===\n");
        let mut results = Vec::new();

        // Buffer copy benchmark (baseline for data movement)
        for size in [64, 256, 1024, 4096, 16384] {
            let data = vec![0u8; size];
            let result = self.run_throughput(
                format!("buffer_copy_{}", size),
                BenchmarkCategory::SerialIo,
                size as u64,
                || {
                    let _copy = data.clone();
                    Ok(())
                },
            )?;
            results.push(result);
        }

        // Protocol encode + serialize pipeline (real send path)
        // Simulates: payload -> protocol encode -> Vec<u8> -> consume
        let mut rtu_proto = ModbusProtocol::new(ModbusMode::Rtu);
        let payload = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x0A];
        let encoded_frame = rtu_proto.encode(&payload).expect("encode");
        let frame_size = encoded_frame.len() as u64;

        let result = self.run_throughput(
            "modbus_rtu_encode_serialize".to_string(),
            BenchmarkCategory::SerialIo,
            frame_size,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Rtu);
                let frame = proto.encode(&payload)?;
                let _consumed = frame;
                Ok(())
            },
        )?;
        results.push(result);

        // Protocol parse + deserialize pipeline (real receive path)
        // Simulates: raw bytes -> protocol parse -> payload
        let result = self.run_throughput(
            "modbus_rtu_decode_deserialize".to_string(),
            BenchmarkCategory::SerialIo,
            encoded_frame.len() as u64,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Rtu);
                let _payload = proto.parse(&encoded_frame)?;
                Ok(())
            },
        )?;
        results.push(result);

        // Full round-trip: encode -> parse (complete I/O cycle)
        let result = self.run_throughput(
            "modbus_rtu_roundtrip".to_string(),
            BenchmarkCategory::SerialIo,
            payload.len() as u64,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Rtu);
                let encoded = proto.encode(&payload)?;
                let _decoded = proto.parse(&encoded)?;
                Ok(())
            },
        )?;
        results.push(result);

        // Modbus ASCII full round-trip (more expensive due to hex encoding)
        let result = self.run_throughput(
            "modbus_ascii_roundtrip".to_string(),
            BenchmarkCategory::SerialIo,
            payload.len() as u64,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Ascii);
                let encoded = proto.encode(&payload)?;
                let _decoded = proto.parse(&encoded)?;
                Ok(())
            },
        )?;
        results.push(result);

        // Async channel throughput simulation (tokio::io::duplex)
        let result = self.run_throughput(
            "async_duplex_channel_4096".to_string(),
            BenchmarkCategory::SerialIo,
            4096,
            || {
                let rt = tokio::runtime::Runtime::new()?;
                rt.block_on(async {
                    let (mut tx, mut rx) = tokio::io::duplex(4096);
                    let data = vec![0u8; 4096];
                    tokio::io::copy(&mut std::io::Cursor::new(data.clone()), &mut tx).await?;
                    let mut buf = Vec::new();
                    tokio::io::copy(&mut rx, &mut buf).await?;
                    Ok::<(), crate::error::SerialError>(())
                })?;
                Ok(())
            },
        )?;
        results.push(result);

        Ok(results)
    }

    /// Run all virtual port benchmarks
    pub fn run_virtual_port_benchmarks(&self) -> Result<Vec<BenchmarkResult>> {
        println!("\n=== Virtual Port Benchmarks ===\n");
        let mut results = Vec::new();

        // Simulated virtual port creation timing (real creation requires OS-level PTY)
        let result = self.run(
            "virtual_port_create_simulation".to_string(),
            BenchmarkCategory::VirtualPort,
            || {
                std::thread::sleep(Duration::from_micros(100));
                Ok(())
            },
        )?;
        results.push(result);

        Ok(results)
    }

    /// Run all protocol benchmarks
    pub fn run_protocol_benchmarks(&self) -> Result<Vec<BenchmarkResult>> {
        println!("\n=== Protocol Benchmarks ===\n");
        let mut results = Vec::new();

        // Modbus RTU parse (real parsing with CRC validation)
        let mut rtu_proto = ModbusProtocol::new(ModbusMode::Rtu);
        let rtu_payload = vec![0x01, 0x03, 0x02, 0x00, 0x0A];
        let rtu_frame = rtu_proto.encode(&rtu_payload).expect("encode RTU frame");
        let rtu_bytes = rtu_frame.len() as u64;

        let result = self.run_throughput(
            "modbus_rtu_parse".to_string(),
            BenchmarkCategory::Protocol,
            rtu_bytes,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Rtu);
                let _decoded = proto.parse(&rtu_frame)?;
                Ok(())
            },
        )?;
        results.push(result);

        // Modbus RTU encode (real encoding with CRC)
        let result = self.run_throughput(
            "modbus_rtu_encode".to_string(),
            BenchmarkCategory::Protocol,
            rtu_payload.len() as u64,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Rtu);
                let _encoded = proto.encode(&rtu_payload)?;
                Ok(())
            },
        )?;
        results.push(result);

        // Modbus ASCII parse (real parsing with hex decode + LRC validation)
        let mut ascii_proto = ModbusProtocol::new(ModbusMode::Ascii);
        let ascii_frame = ascii_proto.encode(&rtu_payload).expect("encode ASCII frame");
        let ascii_bytes = ascii_frame.len() as u64;

        let result = self.run_throughput(
            "modbus_ascii_parse".to_string(),
            BenchmarkCategory::Protocol,
            ascii_bytes,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Ascii);
                let _decoded = proto.parse(&ascii_frame)?;
                Ok(())
            },
        )?;
        results.push(result);

        // Modbus ASCII encode
        let result = self.run_throughput(
            "modbus_ascii_encode".to_string(),
            BenchmarkCategory::Protocol,
            rtu_payload.len() as u64,
            || {
                let mut proto = ModbusProtocol::new(ModbusMode::Ascii);
                let _encoded = proto.encode(&rtu_payload)?;
                Ok(())
            },
        )?;
        results.push(result);

        // Modbus CRC16 standalone (raw computation benchmark)
        let crc_data = rtu_frame.clone();
        let crc_bytes = crc_data.len() as u64;
        let result = self.run_throughput(
            "modbus_crc16_compute".to_string(),
            BenchmarkCategory::Protocol,
            crc_bytes,
            || {
                let _crc = ModbusProtocol::calculate_crc(&crc_data);
                Ok(())
            },
        )?;
        results.push(result);

        // Modbus LRC standalone
        let lrc_data = rtu_payload.clone();
        let lrc_bytes = lrc_data.len() as u64;
        let result = self.run_throughput(
            "modbus_lrc_compute".to_string(),
            BenchmarkCategory::Protocol,
            lrc_bytes,
            || {
                let _lrc = ModbusProtocol::calculate_lrc(&lrc_data);
                Ok(())
            },
        )?;
        results.push(result);

        Ok(results)
    }

    /// Run all startup time benchmarks
    pub fn run_startup_benchmarks(&self) -> Result<Vec<BenchmarkResult>> {
        println!("\n=== Startup Benchmarks ===\n");
        let mut results = Vec::new();

        // Cold start benchmark (no config cache)
        let result = self.run(
            "cold_start_no_config".to_string(),
            BenchmarkCategory::Startup,
            || {
                let _manager = ConfigManager::new();
                Ok(())
            },
        )?;
        results.push(result);

        // Warm start benchmark (with config)
        let result = self.run(
            "warm_start_with_config".to_string(),
            BenchmarkCategory::Startup,
            || {
                let _manager = ConfigManager::load_with_fallback();
                Ok(())
            },
        )?;
        results.push(result);

        // Protocol loading benchmark
        let script_content = r#"
            -- Protocol: benchmark_proto
            function on_frame(data)
                return data
            end

            function on_encode(data)
                return data
            end
        "#;

        let result = self.run(
            "protocol_load_lua_script".to_string(),
            BenchmarkCategory::Startup,
            || {
                // Create a temporary protocol file
                let temp_dir = env::temp_dir();
                let temp_file_path = temp_dir.join(format!("benchmark_proto_{}.lua", std::process::id()));

                let mut file = File::create(&temp_file_path).map_err(|e| {
                    crate::error::SerialError::Protocol(
                        crate::error::ProtocolError::InvalidFrame(format!("Failed to create temp file: {}", e))
                    )
                })?;
                file.write_all(script_content.as_bytes()).map_err(|e| {
                    crate::error::SerialError::Protocol(
                        crate::error::ProtocolError::InvalidFrame(format!("Failed to write temp file: {}", e))
                    )
                })?;

                let _loaded = ProtocolLoader::load_from_file(&temp_file_path)?;

                // Clean up
                let _ = fs::remove_file(&temp_file_path);
                Ok(())
            },
        )?;
        results.push(result);

        // Lua engine initialization benchmark
        let result = self.run(
            "lua_engine_init".to_string(),
            BenchmarkCategory::Startup,
            || {
                let _engine = LuaEngine::new()?;
                Ok(())
            },
        )?;
        results.push(result);

        Ok(results)
    }

    /// Run all memory usage benchmarks
    pub fn run_memory_benchmarks(&self) -> Result<Vec<BenchmarkResult>> {
        println!("\n=== Memory Benchmarks ===\n");
        let mut results = Vec::new();

        // Buffer allocation rate benchmark
        for size in [64, 256, 1024, 4096, 16384] {
            let result = self.run(
                format!("buffer_alloc_{}", size),
                BenchmarkCategory::Memory,
                move || {
                    let _buffer = vec![0u8; size];
                    Ok(())
                },
            )?;
            results.push(result);
        }

        // Data structure size benchmark (ConfigManager)
        let result = self.run(
            "config_manager_size".to_string(),
            BenchmarkCategory::Memory,
            || {
                let _manager = ConfigManager::new();
                Ok(())
            },
        )?;
        results.push(result);

        // Lua engine memory footprint
        let result = self.run(
            "lua_engine_footprint".to_string(),
            BenchmarkCategory::Memory,
            || {
                let _engine = LuaEngine::new()?;
                Ok(())
            },
        )?;
        results.push(result);

        // Protocol loader memory footprint
        let script_content = r#"
            -- Protocol: memory_test
            function on_frame(data) return data end
            function on_encode(data) return data end
        "#;

        let result = self.run(
            "protocol_load_memory".to_string(),
            BenchmarkCategory::Memory,
            || {
                let temp_dir = env::temp_dir();
                let temp_file_path = temp_dir.join(format!("memory_test_{}.lua", std::process::id()));

                let mut file = File::create(&temp_file_path).map_err(|e| {
                    crate::error::SerialError::Protocol(
                        crate::error::ProtocolError::InvalidFrame(format!("Failed to create temp file: {}", e))
                    )
                })?;
                file.write_all(script_content.as_bytes()).map_err(|e| {
                    crate::error::SerialError::Protocol(
                        crate::error::ProtocolError::InvalidFrame(format!("Failed to write temp file: {}", e))
                    )
                })?;

                let _loaded = ProtocolLoader::load_from_file(&temp_file_path)?;

                let _ = fs::remove_file(&temp_file_path);
                Ok(())
            },
        )?;
        results.push(result);

        println!("\nNote: Memory benchmarks measure allocation overhead.");
        println!("For actual memory usage profiling, use tools like heaptrack or valgrind.");

        Ok(results)
    }

    /// Run all concurrency benchmarks
    pub fn run_concurrency_benchmarks(&self) -> Result<Vec<BenchmarkResult>> {
        println!("\n=== Concurrency Benchmarks ===\n");
        let mut results = Vec::new();

        // Pre-build a shared Runtime to avoid measuring Runtime creation overhead
        let rt = Arc::new(tokio::runtime::Runtime::new()?);

        // Concurrent buffer operations benchmark
        for num_tasks in [2, 4, 8] {
            let rt = Arc::clone(&rt);
            let result = self.run(
                format!("concurrent_buffer_ops_{}", num_tasks),
                BenchmarkCategory::Concurrency,
                move || {
                    rt.block_on(async {
                        let mut join_set = JoinSet::new();
                        for _ in 0..num_tasks {
                            join_set.spawn(async {
                                let _buffer = vec![0u8; 4096];
                                // Simulate some work
                                tokio::task::yield_now().await;
                                Ok::<(), crate::error::SerialError>(())
                            });
                        }
                        while let Some(res) = join_set.join_next().await {
                            res.map_err(|e| crate::error::SerialError::Task(
                                crate::error::TaskError::InvalidState(e.to_string())
                            ))??;
                        }
                        Ok::<(), crate::error::SerialError>(())
                    })?;
                    Ok(())
                },
            )?;
            results.push(result);
        }

        // Concurrent Lua engine initialization
        for num_tasks in [2, 4] {
            let rt = Arc::clone(&rt);
            let result = self.run(
                format!("concurrent_lua_init_{}", num_tasks),
                BenchmarkCategory::Concurrency,
                move || {
                    rt.block_on(async {
                        let mut join_set = JoinSet::new();
                        for _ in 0..num_tasks {
                            join_set.spawn(async {
                                let _engine = LuaEngine::new()?;
                                Ok::<(), crate::error::SerialError>(())
                            });
                        }
                        while let Some(res) = join_set.join_next().await {
                            res.map_err(|e| crate::error::SerialError::Task(
                                crate::error::TaskError::InvalidState(e.to_string())
                            ))??;
                        }
                        Ok::<(), crate::error::SerialError>(())
                    })?;
                    Ok(())
                },
            )?;
            results.push(result);
        }

        // Concurrent config loading
        for num_tasks in [2, 4, 8] {
            let rt = Arc::clone(&rt);
            let result = self.run(
                format!("concurrent_config_load_{}", num_tasks),
                BenchmarkCategory::Concurrency,
                move || {
                    rt.block_on(async {
                        let mut join_set = JoinSet::new();
                        for _ in 0..num_tasks {
                            join_set.spawn(async {
                                let _manager = ConfigManager::new();
                                Ok::<(), crate::error::SerialError>(())
                            });
                        }
                        while let Some(res) = join_set.join_next().await {
                            res.map_err(|e| crate::error::SerialError::Task(
                                crate::error::TaskError::InvalidState(e.to_string())
                            ))??;
                        }
                        Ok::<(), crate::error::SerialError>(())
                    })?;
                    Ok(())
                },
            )?;
            results.push(result);
        }

        println!("\nNote: Concurrency benchmarks measure parallel operation overhead.");
        println!("Higher task counts may show diminishing returns due to thread contention.");

        Ok(results)
    }

    /// Run all benchmarks
    pub fn run_all(&self) -> Result<Vec<BenchmarkResult>> {
        let mut all_results = Vec::new();

        all_results.extend(self.run_serial_io_benchmarks()?);
        all_results.extend(self.run_virtual_port_benchmarks()?);
        all_results.extend(self.run_protocol_benchmarks()?);
        all_results.extend(self.run_startup_benchmarks()?);
        all_results.extend(self.run_memory_benchmarks()?);
        all_results.extend(self.run_concurrency_benchmarks()?);

        Ok(all_results)
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}
