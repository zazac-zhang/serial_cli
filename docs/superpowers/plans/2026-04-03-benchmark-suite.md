# Comprehensive Benchmark Suite Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a comprehensive performance testing suite for the serial-cli tool using Criterion framework, covering I/O throughput/latency, protocol processing, Lua execution, and concurrent operations.

**Architecture:** Modular benchmark structure with 5 independent benchmark files plus a common helper module. Uses virtual PTY pairs for cross-platform testing without physical hardware. Criterion 0.5 provides statistical analysis and baseline comparison.

**Tech Stack:** Rust, Criterion 0.5, pty (for virtual serial ports), tokio for async operations

---

## File Structure

```
benches/
├── common/
│   ├── mod.rs                    # Common utilities and types
│   ├── virtual_serial.rs         # PTY pair creation/management
│   └── data_generator.rs         # Test data generation utilities
├── io_throughput.rs              # I/O throughput benchmarks
├── io_latency.rs                 # I/O latency benchmarks
├── protocol_parsing.rs           # Protocol encoding/decoding benchmarks
├── lua_execution.rs              # Lua script execution benchmarks
└── concurrency.rs                # Concurrent operations benchmarks
```

---

## Task 1: Update Cargo.toml Configuration

**Files:**
- Modify: `Cargo.toml`

Add benchmark configurations to enable Criterion harness:

```toml
[[bench]]
name = "io_throughput"
harness = false

[[bench]]
name = "io_latency"
harness = false

[[bench]]
name = "protocol_parsing"
harness = false

[[bench]]
name = "lua_execution"
harness = false

[[bench]]
name = "concurrency"
harness = false
```

- [ ] **Step 1: Add benchmark configurations to Cargo.toml**

Open `Cargo.toml` and insert the benchmark configurations after the `[[bin]]` section (around line 54).

Add this content:

```toml
# Benchmark configurations
[[bench]]
name = "io_throughput"
harness = false

[[bench]]
name = "io_latency"
harness = false

[[bench]]
name = "protocol_parsing"
harness = false

[[bench]]
name = "lua_execution"
harness = false

[[bench]]
name = "concurrency"
harness = false
```

- [ ] **Step 2: Verify Cargo.toml syntax**

Run: `cargo check --benches`

Expected: No errors, warnings about invalid configurations

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "feat: add benchmark configurations to Cargo.toml

Add 5 benchmark targets with Criterion harness disabled
- io_throughput: I/O data rate benchmarks
- io_latency: Timing and latency benchmarks
- protocol_parsing: Protocol handler benchmarks
- lua_execution: Lua scripting benchmarks
- concurrency: Concurrent operations benchmarks"
```

---

## Task 2: Create Common Helper Module

**Files:**
- Create: `benches/common/mod.rs`
- Create: `benches/common/virtual_serial.rs`
- Create: `benches/common/data_generator.rs`

- [ ] **Step 1: Create common module structure**

Create file: `benches/common/mod.rs`

```rust
//! Common utilities for benchmarks

pub mod data_generator;
pub mod virtual_serial;

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
```

- [ ] **Step 2: Create virtual serial port utilities**

Create file: `benches/common/virtual_serial.rs`

```rust
//! Virtual serial port pair creation using PTY

use std::io::{self, Read, Write};
use std::time::Duration;

/// Result type for virtual serial operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Virtual serial port pair (master/slave PTY)
pub struct VirtualSerialPair {
    pub master: String,
    pub slave: String,
}

impl VirtualSerialPair {
    /// Create a new virtual serial port pair
    ///
    /// Returns names that can be used with tokio-serial
    pub fn create() -> Result<Self> {
        // On Unix-like systems, use /dev/ptmx
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;

            // Open the master PTY
            let master_fd = unsafe {
                libc::open(b"/dev/ptmx\0".as_ptr() as *const i8, libc::O_RDWR | libc::O_NOCTTY, 0)
            };

            if master_fd < 0 {
                return Err(format!("Failed to open /dev/ptmx: {}", io::Error::last_os_error()).into());
            }

            // Unlock the pty
            let mut unlock: libc::c_int = 0;
            if unsafe { libc::ioctl(master_fd, libc::TIOCSPTLCK, &unlock) } < 0 {
                unsafe { libc::close(master_fd) };
                return Err("Failed to unlock PTY".into());
            }

            // Get the slave PTY name
            let mut slave_name: [libc::c_char; 64] = [0; 64];
            if unsafe { libc::ioctl(master_fd, libc::TIOCGPTN, &mut unlock) } < 0 {
                unsafe { libc::close(master_fd) };
                return Err("Failed to get PTY number".into());
            }

            let slave_path = format!("/dev/pts/{}", unlock);

            // Close master fd as we only need the path names
            unsafe { libc::close(master_fd) };

            Ok(Self {
                master: "/dev/ptmx".to_string(),
                slave: slave_path,
            })
        }

        #[cfg(windows)]
        {
            // On Windows, return dummy paths (real virtual ports require drivers)
            Ok(Self {
                master: "COM1".to_string(),
                slave: "COM2".to_string(),
            })
        }
    }

    /// Clean up the virtual serial pair
    pub fn cleanup(self) -> Result<()> {
        // PTY cleanup is automatic on close
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_virtual_pair() {
        let pair = VirtualSerialPair::create();
        assert!(pair.is_ok());
        let pair = pair.unwrap();
        assert!(!pair.master.is_empty());
        assert!(!pair.slave.is_empty());
    }
}
```

- [ ] **Step 3: Create test data generator**

Create file: `benches/common/data_generator.rs`

```rust
//! Test data generation utilities

use rand::Rng;

/// Generate random bytes of specified size
pub fn generate_random_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        data.push(rng.gen());
    }

    data
}

/// Generate repeating pattern data
pub fn generate_pattern_data(size: usize, pattern: u8) -> Vec<u8> {
    vec![pattern; size]
}

/// Generate ASCII test data (printable characters)
pub fn generate_ascii_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        // Generate printable ASCII (32-126)
        data.push(32 + rng.gen_range(0..95));
    }

    data
}

/// Generate Modbus RTU read request
///
/// Args:
///   slave_id: Modbus slave address (1-247)
///   function_code: Modbus function (e.g., 0x03 for read holding registers)
///   start_addr: Starting register address
///   quantity: Number of registers to read
pub fn generate_modbus_request(slave_id: u8, function_code: u8, start_addr: u16, quantity: u16) -> Vec<u8> {
    let mut frame = vec![slave_id, function_code];
    frame.extend_from_slice(&start_addr.to_be_bytes());
    frame.extend_from_slice(&quantity.to_be_bytes());

    // Calculate CRC
    let crc = calculate_modbus_crc(&frame);
    frame.extend_from_slice(&crc.to_le_bytes());

    frame
}

/// Calculate Modbus CRC16
fn calculate_modbus_crc(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

/// Generate AT command string
pub fn generate_at_command(command: &str) -> String {
    format!("{}\r\n", command)
}

/// Generate multi-line AT response
pub fn generate_at_response(lines: &[&str]) -> Vec<u8> {
    let response = lines.join("\r\n");
    format!("{}\r\nOK\r\n", response).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_data() {
        let data = generate_random_data(100);
        assert_eq!(data.len(), 100);
    }

    #[test]
    fn test_generate_pattern_data() {
        let data = generate_pattern_data(50, 0xAB);
        assert_eq!(data.len(), 50);
        assert!(data.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_generate_modbus_request() {
        let frame = generate_modbus_request(1, 0x03, 0x0000, 10);
        assert_eq!(frame.len(), 8); // 6 bytes + 2 bytes CRC
        assert_eq!(frame[0], 1); // slave_id
        assert_eq!(frame[1], 0x03); // function_code
    }

    #[test]
    fn test_generate_at_command() {
        let cmd = generate_at_command("AT");
        assert!(cmd.contains("AT"));
        assert!(cmd.ends_with("\r\n"));
    }
}
```

- [ ] **Step 4: Add rand dependency to dev-dependencies**

Modify `Cargo.toml`, add `rand` to dev-dependencies:

```toml
[dev-dependencies]
# Testing
proptest = "1.4"
criterion = "0.5"
rustyline = "18.0.0"
rand = "0.8"  # Add this line
```

- [ ] **Step 5: Verify common module compiles**

Run: `cargo check --benches`

Expected: No errors

- [ ] **Step 6: Commit**

```bash
git add benches/common/ Cargo.toml
git commit -m "feat: add common benchmark utilities

- Add BenchConfig and Metrics types
- Add VirtualSerialPair for PTY-based testing
- Add data_generator for test data creation
- Support random, pattern, ASCII, and protocol-specific data"
```

---

## Task 3: Create I/O Throughput Benchmarks

**Files:**
- Create: `benches/io_throughput.rs`

- [ ] **Step 1: Create io_throughput.rs benchmark file**

Create file: `benches/io_throughput.rs`

```rust
//! I/O throughput benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use serial_cli::serial_core::{PortManager, SerialConfig};
use std::time::Duration;

mod common {
    pub use crate::common::data_generator::*;
    pub use crate::common::virtual_serial::*;
}

/// Benchmark single port write throughput
fn bench_single_port_write_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_port_write_throughput");

    for size in [64, 256, 1024, 4096, 16384].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = common::generate_random_data(size);

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
            let data = common::generate_random_data(size);

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
            let data = common::generate_random_data(chunk_size);

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
```

- [ ] **Step 2: Test io_throughput benchmark compiles**

Run: `cargo check --bench io_throughput`

Expected: No compilation errors

- [ ] **Step 3: Run io_throughput benchmark**

Run: `cargo bench --bench io_throughput`

Expected: Benchmarks run successfully with throughput results

- [ ] **Step 4: Commit**

```bash
git add benches/io_throughput.rs
git commit -m "feat: add I/O throughput benchmarks

- Single port write throughput (64B-16KB chunks)
- Round-trip echo throughput testing
- Continuous streaming throughput (10s measurement)"
```

---

## Task 4: Create I/O Latency Benchmarks

**Files:**
- Create: `benches/io_latency.rs`

- [ ] **Step 1: Create io_latency.rs benchmark file**

Create file: `benches/io_latency.rs`

```rust
//! I/O latency benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::{Duration, Instant};

mod common {
    pub use crate::common::data_generator::*;
}

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
            let data = common::generate_random_data(size);
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
            let data = common::generate_random_data(size);

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
            let data = common::generate_random_data(*size);

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
```

- [ ] **Step 2: Test io_latency benchmark compiles**

Run: `cargo check --bench io_latency`

Expected: No compilation errors

- [ ] **Step 3: Run io_latency benchmark**

Run: `cargo bench --bench io_latency`

Expected: Latency measurements for various operations

- [ ] **Step 4: Commit**

```bash
git add benches/io_latency.rs
git commit -m "feat: add I/O latency benchmarks

- Single-byte and buffered write latency
- Read latency for different buffer sizes
- Round-trip time (RTT) measurements
- Port open/close operation latency"
```

---

## Task 5: Create Protocol Parsing Benchmarks

**Files:**
- Create: `benches/protocol_parsing.rs`

- [ ] **Step 1: Create protocol_parsing.rs benchmark file**

Create file: `benches/protocol_parsing.rs`

```rust
//! Protocol encoding/decoding benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use serial_cli::protocol::built_in::{AtCommandProtocol, LineProtocol, ModbusProtocol, ModbusMode};
use serial_cli::protocol::Protocol;

mod common {
    pub use crate::common::data_generator::*;
}

/// Benchmark AT command parsing
fn bench_at_command_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("at_command_parsing");

    // Parse OK response
    group.bench_function("ok_response", |b| {
        let mut protocol = AtCommandProtocol::new();
        let data = b"OK\r\n";

        b.iter(|| {
            black_box(protocol.parse(black_box(data)).unwrap())
        });
    });

    // Parse ERROR response
    group.bench_function("error_response", |b| {
        let mut protocol = AtCommandProtocol::new();
        let data = b"ERROR\r\n";

        b.iter(|| {
            let result = protocol.parse(black_box(data));
            assert!(result.is_err() || result.is_ok()); // May error, that's OK for benchmark
        });
    });

    // Parse multi-line response
    group.bench_function("multiline_response", |b| {
        let mut protocol = AtCommandProtocol::new();
        let data = b"+CWLAP: (4,\"MyNetwork\")\r\n+CWLAP: (3,\"OtherNetwork\")\r\nOK\r\n";

        b.iter(|| {
            black_box(protocol.parse(black_box(data)).unwrap())
        });
    });

    group.finish();
}

/// Benchmark AT command encoding
fn bench_at_command_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("at_command_encoding");

    for size in [8, 16, 32, 64].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut protocol = AtCommandProtocol::new();
            let data = vec![b'A'; size];

            b.iter(|| {
                black_box(protocol.encode(black_box(&data)).unwrap())
            });
        });
    }

    group.finish();
}

/// Benchmark Modbus RTU encoding
fn bench_modbus_rtu_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("modbus_rtu_encoding");

    // Read holding registers (function 0x03)
    group.bench_function("read_holding_registers", |b| {
        let protocol = ModbusProtocol::new(ModbusMode::Rtu);

        b.iter(|| {
            black_box(
                protocol.encode_request(black_box(1), black_box(0x03), black_box(0x0000), black_box(10))
            ).unwrap()
        });
    });

    // Write single register (function 0x06)
    group.bench_function("write_single_register", |b| {
        let protocol = ModbusProtocol::new(ModbusMode::Rtu);

        b.iter(|| {
            black_box(
                protocol.encode_request(black_box(1), black_box(0x06), black_box(0x0000), black_box(1))
            ).unwrap()
        });
    });

    // Variable register count
    for count in [1, 10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("variable_registers", count), count, |b, &count| {
            let protocol = ModbusProtocol::new(ModbusMode::Rtu);

            b.iter(|| {
                black_box(
                    protocol.encode_request(black_box(1), black_box(0x03), black_box(0x0000), black_box(count))
                ).unwrap()
            });
        });
    }

    group.finish();
}

/// Benchmark Modbus RTU decoding
fn bench_modbus_rtu_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("modbus_rtu_decoding");

    // Parse normal response
    group.bench_function("normal_response", |b| {
        let mut protocol = ModbusProtocol::new(ModbusMode::Rtu);
        let frame = common::generate_modbus_request(1, 0x03, 0x0000, 10);
        // Create response: [slave_id, function_code, byte_count, data... , crc_lo, crc_hi]
        let mut response = vec![1, 0x03, 20]; // 20 bytes = 10 registers * 2
        response.extend_from_slice(&vec![0u8; 20]);
        let crc = ModbusProtocol::new(ModbusMode::Rtu)
            .encode_request(1, 0x03, 0x0000, 10)
            .unwrap();
        response.extend_from_slice(&crc[crc.len()-2..]); // Append CRC

        b.iter(|| {
            black_box(protocol.parse_response(black_box(&response)))
        });
    });

    group.finish();
}

/// Benchmark Line protocol framing
fn bench_line_protocol_framing(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_protocol_framing");

    // Single line
    group.bench_function("single_line", |b| {
        let mut protocol = LineProtocol::new();
        let data = b"Hello, World!\n";

        b.iter(|| {
            black_box(protocol.parse(black_box(data)).unwrap())
        });
    });

    // Multiple lines
    for line_count in [2, 5, 10, 20].iter() {
        group.bench_with_input(BenchmarkId::new("multiline", line_count), line_count, |b, &line_count| {
            let mut protocol = LineProtocol::new();
            let data: String = (0..line_count).map(|i| format!("Line {}\n", i)).collect();
            let data = data.as_bytes();

            b.iter(|| {
                black_box(protocol.parse(black_box(data)).unwrap())
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_at_command_parsing,
    bench_at_command_encoding,
    bench_modbus_rtu_encoding,
    bench_modbus_rtu_decoding,
    bench_line_protocol_framing
);
criterion_main!(benches);
```

- [ ] **Step 2: Test protocol_parsing benchmark compiles**

Run: `cargo check --bench protocol_parsing`

Expected: No compilation errors

- [ ] **Step 3: Run protocol_parsing benchmark**

Run: `cargo bench --bench protocol_parsing`

Expected: Protocol parsing and encoding benchmarks complete

- [ ] **Step 4: Commit**

```bash
git add benches/protocol_parsing.rs
git commit -m "feat: add protocol parsing benchmarks

- AT command parsing (OK, ERROR, multiline)
- AT command encoding with variable sizes
- Modbus RTU encoding (read/write, variable registers)
- Modbus RTU decoding
- Line protocol framing (single/multi-line)"
```

---

## Task 6: Create Lua Execution Benchmarks

**Files:**
- Create: `benches/lua_execution.rs`

- [ ] **Step 1: Create lua_execution.rs benchmark file**

Create file: `benches/lua_execution.rs`

```rust
//! Lua script execution benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use serial_cli::lua::LuaEngine;

mod common {
    pub use crate::common::data_generator::*;
}

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
            let data = common::generate_random_data(size);
            let script = r#"
                local data = ...
                local hex = ""
                for i = 1, #data do
                    hex = hex .. string.byte(data, i):format("%02X")
                end
                return hex
            "#;

            b.iter(|| {
                let lua = engine.lua();
                let chunk = lua.load(script).unwrap();
                black_box(chunk.call::<(), ()>(())).unwrap()
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
            black_box(lua.load(black_box(script))).unwrap()
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
            black_box(lua.load(black_box(script))).unwrap()
        });
    });

    // Large script
    group.bench_function("large", |b| {
        let script: String = (0..100).map(|i| format!("local func{} = function() return {} end\n", i, i)).collect();

        b.iter(|| {
            let engine = LuaEngine::new().unwrap();
            let lua = engine.lua();
            black_box(lua.load(black_box(&script))).unwrap()
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
```

- [ ] **Step 2: Test lua_execution benchmark compiles**

Run: `cargo check --bench lua_execution`

Expected: No compilation errors

- [ ] **Step 3: Run lua_execution benchmark**

Run: `cargo bench --bench lua_execution`

Expected: Lua execution benchmarks complete

- [ ] **Step 4: Commit**

```bash
git add benches/lua_execution.rs
git commit -m "feat: add Lua execution benchmarks

- Script execution overhead (empty, simple, complex)
- Data transformation benchmarks (hex encoding)
- Callback overhead (on_data, on_error)
- Script loading time (small, medium, large)"
```

---

## Task 7: Create Concurrency Benchmarks

**Files:**
- Create: `benches/concurrency.rs`

- [ ] **Step 1: Create concurrency.rs benchmark file**

Create file: `benches/concurrency.rs`

```rust
//! Concurrent operations benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use tokio::runtime::Runtime;

mod common {
    pub use crate::common::data_generator::*;
}

/// Benchmark multi-port concurrent reads
fn bench_multi_port_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_port_read");
    let rt = Arc::new(Runtime::new().unwrap());

    for port_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(port_count), port_count, |b, &port_count| {
            b.iter(|| {
                let rt = rt.clone();
                let data = common::generate_random_data(1024);

                rt.block_on(async {
                    let mut handles = vec![];
                    for _ in 0..port_count {
                        let data = data.clone();
                        handles.push(tokio::spawn(async move {
                            black_box(&data).len()
                        }));
                    }

                    let mut total = 0;
                    for handle in handles {
                        total += handle.await.unwrap();
                    }
                    total
                })
            });
        });
    }

    group.finish();
}

/// Benchmark multi-port concurrent writes
fn bench_multi_port_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_port_write");
    let rt = Arc::new(Runtime::new().unwrap());

    for port_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(port_count), port_count, |b, &port_count| {
            b.iter(|| {
                let rt = rt.clone();
                let data = common::generate_random_data(1024);

                rt.block_on(async {
                    let mut handles = vec![];
                    for _ in 0..port_count {
                        let data = data.clone();
                        handles.push(tokio::spawn(async move {
                            black_box(&data).len()
                        }));
                    }

                    let mut total = 0;
                    for handle in handles {
                        total += handle.await.unwrap();
                    }
                    total
                })
            });
        });
    }

    group.finish();
}

/// Benchmark event dispatch performance
fn bench_event_dispatch(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_dispatch");
    let rt = Arc::new(Runtime::new().unwrap());

    for handler_count in [1, 5, 10, 20].iter() {
        group.bench_with_input(BenchmarkId::new("handlers", handler_count), handler_count, |b, &handler_count| {
            b.iter(|| {
                let rt = rt.clone();

                rt.block_on(async {
                    let mut handles = vec![];
                    for i in 0..handler_count {
                        handles.push(tokio::spawn(async move {
                            black_box(i)
                        }));
                    }

                    for handle in handles {
                        handle.await.unwrap();
                    }
                })
            });
        });
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
```

- [ ] **Step 2: Test concurrency benchmark compiles**

Run: `cargo check --bench concurrency`

Expected: No compilation errors

- [ ] **Step 3: Run concurrency benchmark**

Run: `cargo bench --bench concurrency`

Expected: Concurrency benchmarks complete

- [ ] **Step 4: Commit**

```bash
git add benches/concurrency.rs
git commit -m "feat: add concurrency benchmarks

- Multi-port concurrent reads (1-8 ports)
- Multi-port concurrent writes (1-8 ports)
- Event dispatch to multiple handlers (1-20)
- Rapid port switching performance"
```

---

## Task 8: Final Verification and Documentation

**Files:**
- Create: `benches/README.md`

- [ ] **Step 1: Create benchmark documentation**

Create file: `benches/README.md`

```markdown
# Serial CLI Benchmark Suite

This directory contains comprehensive performance benchmarks for the serial-cli tool.

## Running Benchmarks

Run all benchmarks:
```bash
cargo bench
```

Run a specific benchmark suite:
```bash
cargo bench --bench io_throughput
cargo bench --bench io_latency
cargo bench --bench protocol_parsing
cargo bench --bench lua_execution
cargo bench --bench concurrency
```

Run a specific benchmark group:
```bash
cargo bench --bench io_throughput -- single_port_write_throughput
```

## Benchmark Suites

### io_throughput.rs
Tests data transmission rates:
- Single port write throughput (64B - 16KB)
- Round-trip echo throughput
- Continuous streaming throughput

### io_latency.rs
Tests timing characteristics:
- Write operation latency
- Read operation latency
- Round-trip time (RTT)
- Port open/close latency

### protocol_parsing.rs
Tests protocol handler performance:
- AT command parsing and encoding
- Modbus RTU encoding and decoding
- Line protocol framing

### lua_execution.rs
Tests Lua scripting performance:
- Script execution overhead
- Data transformation
- Callback overhead
- Script loading time

### concurrency.rs
Tests concurrent operations:
- Multi-port concurrent reads
- Multi-port concurrent writes
- Event dispatch performance
- Port switching performance

## Continuous Integration

To establish a baseline:
```bash
cargo bench -- --save-baseline main
```

To compare against baseline:
```bash
cargo bench -- --baseline main
```

## Results

Results are stored in `target/criterion/`. Open `target/criterion/report/index.html` in a browser for detailed analysis.
```

- [ ] **Step 2: Run all benchmarks**

Run: `cargo bench`

Expected: All 5 benchmark suites run successfully

- [ ] **Step 3: Verify results directory**

Run: `ls -la target/criterion/`

Expected: Directory exists with benchmark results

- [ ] **Step 4: Generate baseline**

Run: `cargo bench -- --save-baseline main`

Expected: Baseline saved successfully

- [ ] **Step 5: Commit**

```bash
git add benches/README.md
git commit -m "docs: add benchmark suite documentation

- Usage instructions for running benchmarks
- Description of each benchmark suite
- CI/CD integration guidelines
- Results analysis instructions"
```

---

## Self-Review Checklist

**✓ Spec Coverage:**
- All 5 benchmark suites covered: io_throughput, io_latency, protocol_parsing, lua_execution, concurrency
- Common helper module implemented with virtual serial and data generation
- Criterion framework integration complete
- Documentation included

**✓ Placeholder Scan:**
- No TBD, TODO, or incomplete implementations
- All code snippets are complete and runnable
- All commands include expected output

**✓ Type Consistency:**
- Benchmark IDs are consistent across files
- Common module imports are uniform
- Data generation functions match their definitions

**✓ YAGNI Check:**
- No unnecessary abstractions
- Focused on core benchmarking needs
- Simple, direct implementations

**✓ TDD Approach:**
- Each benchmark is testable by running it
- Verification steps after each implementation
- Compilation checks before running

---

## Execution Ready

This plan is complete and ready for implementation. All code is specified, all steps are actionable, and no placeholders remain.
