# Benchmark Performance Testing Design

**Date:** 2026-04-03
**Status:** Approved
**Author:** Serial CLI Team

## Overview

This document outlines the comprehensive benchmark testing strategy for the serial-cli tool. The benchmark suite will measure performance across all major components: I/O throughput/latency, protocol processing, Lua script execution, and concurrent operations.

## Goals

1. **Performance Regression Detection** - Establish performance baselines to detect degradations
2. **Bottleneck Identification** - Find performance limits and system constraints
3. **Comparative Analysis** - Enable before/after performance comparisons
4. **Continuous Monitoring** - Support CI/CD integration for automated performance tracking

## Testing Approach

### Virtual Serial Port Strategy

We will use virtual pseudo-terminal (PTY) pairs to simulate serial ports without requiring physical hardware. This approach:

- Enables testing on any system (CI/CD friendly)
- Provides reproducible results across different machines
- Allows automation without hardware dependencies
- Supports cross-platform testing (Linux/macOS via pty, Windows via virtual drivers or skip)

### Test Data Strategy

**Data Size Categories:**
- **Micro benchmarks:** 64B - 1KB (function-level performance)
- **Standard tests:** 1KB - 100KB (typical use cases)
- **Stress tests:** 1MB - 10MB (extreme conditions)

**Configuration Coverage:**
- Baud rates: 9600, 115200, 921600
- Data bits: 7, 8
- Stop bits: 1, 2
- Parity: None, Odd, Even
- Flow control: None, Software, Hardware

## Architecture

### Directory Structure

```
benches/
├── io_throughput.rs       # I/O throughput benchmarks
├── io_latency.rs          # I/O latency benchmarks
├── protocol_parsing.rs    # Protocol encoding/decoding benchmarks
├── lua_execution.rs       # Lua script execution benchmarks
├── concurrency.rs         # Concurrent operations benchmarks
└── common/
    ├── mod.rs             # Common utilities module
    ├── virtual_serial.rs  # Virtual PTY pair creation
    └── data_generator.rs  # Test data generation
```

### Benchmark Modules

#### 1. I/O Throughput (`io_throughput.rs`)

Tests data transmission rates under various configurations.

**Benchmark Groups:**

- **single_port_throughput** - Single port throughput measurement
  - Variables: Data chunk size (64B, 256B, 1KB, 4KB, 16KB)
  - Variables: Baud rate simulation (9600, 115200, 921600)
  - Metrics: MB/s, total bytes transferred

- **round_trip_throughput** - Echo-based bidirectional throughput
  - Sends data and receives echo
  - Tests full-duplex communication performance

- **continuous_stream** - Sustained data stream testing
  - Simulates long-running data transfers
  - Tests for performance degradation over time

#### 2. I/O Latency (`io_latency.rs`)

Tests timing characteristics of serial operations.

**Benchmark Groups:**

- **write_latency** - Write operation timing
  - Single-byte write latency
  - Buffered write vs multiple small writes

- **read_latency** - Read operation timing
  - Time to read available data
  - Blocking vs non-blocking mode comparison

- **round_trip_time** - Round-trip time (RTT) measurement
  - Send and wait for echo
  - Measure at different data sizes

- **port_open_latency** - Port management overhead
  - Time to open and close ports
  - Port configuration changes

#### 3. Protocol Parsing (`protocol_parsing.rs`)

Tests built-in protocol handler performance.

**Benchmark Groups:**

- **at_command_parsing** - AT command protocol
  - Parse standard responses ("OK", "ERROR")
  - Parse multi-line responses
  - Parse complex responses (e.g., +CWLAP)

- **modbus_rtu_encoding** - Modbus RTU encoding
  - Read holding registers request
  - Write single/multiple registers
  - CRC calculation performance

- **modbus_rtu_decoding** - Modbus RTU decoding
  - Parse normal responses
  - Parse exception responses
  - Variable register count impact

- **line_protocol_framing** - Line-based framing
  - Split by newline delimiter
  - Handle partial buffers
  - Multi-line data processing

#### 4. Lua Execution (`lua_execution.rs`)

Tests Lua scripting integration performance.

**Benchmark Groups:**

- **script_overhead** - Baseline execution cost
  - Empty script baseline
  - Simple data processing
  - Complex transformation logic

- **data_transformation** - Data conversion performance
  - Hex encoding/decoding in Lua vs Rust
  - Custom protocol parsing in Lua
  - Performance comparison with native implementation

- **event_handling** - Callback performance
  - `on_data` callback overhead
  - `on_error` callback overhead
  - Multiple concurrent event handlers

- **script_loading** - Script initialization
  - Load time vs script size
  - Script caching effectiveness
  - Repeated load performance

#### 5. Concurrency (`concurrency.rs`)

Tests multi-port concurrent operations.

**Benchmark Groups:**

- **multi_port_read** - Concurrent read operations
  - 2, 4, 8, 16 simultaneous ports
  - Throughput scalability measurement

- **multi_port_write** - Concurrent write operations
  - Concurrent write performance
  - Write synchronization overhead

- **event_dispatch** - Event system performance
  - Dispatch to multiple handlers
  - Channel capacity impact

- **port_switching** - Rapid port switching
  - Fast context switching between ports
  - Port manager concurrent access

#### 6. Common Utilities (`common/`)

Shared testing infrastructure.

**Modules:**

- **virtual_serial.rs** - PTY pair management
  ```rust
  pub fn create_virtual_pair() -> Result<(String, String)>
  pub fn cleanup_virtual_ports()
  ```

- **data_generator.rs** - Test data generation
  ```rust
  pub fn generate_random_data(size: usize) -> Vec<u8>
  pub fn generate_pattern_data(size: usize, pattern: u8) -> Vec<u8>
  pub fn generate_modbus_request() -> Vec<u8>
  pub fn generate_at_command() -> String
  ```

- **mod.rs** - Common types and configuration
  ```rust
  pub struct BenchConfig {
      pub warmup_iters: usize,
      pub measure_iters: usize,
      pub sample_size: usize,
  }
  ```

## Implementation Framework

### Criterion Configuration

We use Criterion 0.5 as the benchmarking framework, configured in `Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.5"

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

### Benchmark Structure Pattern

Each benchmark file follows this structure:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use serial_cli::serial_core::{PortManager, SerialConfig};
use common::{virtual_serial, data_generator};

fn benchmark_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("group_name");

    for size in [64, 256, 1024, 4096].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                // Benchmark code here
                black_box(test_function(size))
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

## Running Benchmarks

### Basic Commands

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench io_throughput
cargo bench --bench protocol_parsing

# Run specific benchmark group
cargo bench --bench io_throughput -- single_port_throughput

# Generate detailed comparison report
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

### CI/CD Integration

```bash
# Save baseline after main branch builds
cargo bench -- --save-baseline main

# Compare PR against baseline (fail if regression > 5%)
cargo bench -- --baseline main
```

## Success Criteria

1. **All benchmarks run without errors** - No panics or unwraps
2. **Reproducible results** - Low variance across runs (< 5%)
3. **Clear metrics** - Throughput, latency, and resource usage visible
4. **Baseline tracking** - Can compare against previous runs
5. **CI compatible** - Runs successfully in virtual environments

## Future Enhancements

1. **HTML Reports** - Generate visual performance charts
2. **Flamegraphs** - CPU profiling integration
3. **Memory Profiling** - Track memory allocation patterns
4. **Custom Metrics** - Add platform-specific performance counters
5. **Historical Tracking** - Store results over time for trend analysis

## References

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/index.html)
- [Rust Benchmark Guidelines](https://doc.rust-lang.org/1.70.0/book/ch15-01-box.html)
- [Serial Port Performance Best Practices](https://en.wikipedia.org/wiki/Serial_port#Settings)
