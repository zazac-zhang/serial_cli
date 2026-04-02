# 性能测试策略

Serial CLI是一个需要硬件支持的串口工具，性能测试采用分层策略。

## 🎯 测试分层

### 1. 无需硬件的性能测试（可立即运行）

#### 协议算法测试
```bash
# 测试Modbus协议编解码性能
cargo test --release protocol_benches

# 测试CRC16校验性能
cargo test --release crc_benches

# 测试数据转换性能
cargo test --release conversion_benches
```

#### 可以测试的内容
- ✅ Modbus RTU/ASCII协议编码/解码
- ✅ CRC16/CRC8/LRC校验算法
- ✅ 十六进制转换
- ✅ 数据序列化/反序列化
- ✅ 字符串处理
- ✅ 内存分配效率

### 2. 需要硬件的性能测试（本地运行）

#### 真实硬件测试
```bash
# 在有串口硬件的机器上运行
RUST_LOG=info cargo test --release hardware_benches -- --ignored
```

#### 测试环境要求
- 串口适配器（FTDI, CP2102, CH340等）
- 真实设备（PLC、传感器、串口设备）
- 或虚拟串口对

## 🛠️ 虚拟串口设置

### Linux
```bash
# 方法1: 使用socat
sudo apt-get install socat
socat -d -d pty,raw,echo=0,link=/tmp/ttyVIRTUAL0 pty,raw,echo=0,link=/tmp/ttyVIRTUAL1

# 方法2: 使用tty0p/tty0c
# 这些是系统自带的虚拟串口对

# 测试
cargo test --test virtual_serial_tests
```

### macOS
```bash
# 使用socat
brew install socat
socat -d -d pty,raw,echo=0,link=/tmp/ttyVIRTUAL0 pty,raw,echo=0,link=/tmp/ttyVIRTUAL1
```

### Windows
```bash
# 使用com0com
# 下载: https://sourceforge.net/projects/com0com/

# 安装后创建虚拟串口对
# setupc.exe install
# setupc.exe list
# setupc.exe portname CNCA0 COM10
# setupc.exe portname CNCB0 COM11
```

## 📊 性能基准

### 协议处理性能
| 操作 | 目标性能 | 测试方法 |
|------|---------|---------|
| Modbus编码 | <100μs | 单元测试 |
| Modbus解码 | <100μs | 单元测试 |
| CRC16计算 | <10μs per 256B | 单元测试 |
| 十六进制编码 | <1μs per KB | 单元测试 |

### 串口性能（需硬件）
| 操作 | 目标性能 | 测试方法 |
|------|---------|---------|
| 写入吞吐量 | >1MB/s | 硬件测试 |
| 读取延迟 | <10ms | 硬件测试 |
| 并发连接 | >10个端口 | 硬件测试 |

## 🔬 实际测试命令

### 日常开发（无需硬件）
```bash
# 运行所有算法测试
cargo test --release

# 运行性能关键路径测试
cargo test --release -- --test-threads=1

# 检查测试覆盖率
cargo tarpaulin --out Html
```

### 发布前测试（需要硬件）
```bash
# 1. 运行所有测试
cargo test --release

# 2. 在真实设备上测试
RUST_LOG=debug cargo test --release -- --ignored

# 3. 压力测试
cargo test --release stress_tests -- --test-threads=1

# 4. 内存泄漏检查
cargo test --release -- --ignored --test-threads=1
valgrind --leak-check=full target/release/serial-cli-*
```

## 🎯 CI/CD策略

### GitHub Actions
```yaml
# .github/workflows/ci.yml
- name: Run algorithm tests
  run: cargo test --release

- name: Run hardware tests (scheduled)
  if: github.event_name == 'schedule'
  run: cargo test --release -- --ignored
  # 在有硬件的self-hosted runner上运行
```

### 定期硬件测试
- 每周在真实设备上运行完整测试
- 使用self-hosted GitHub Actions runner
- 或在本地测试后上传结果

## 📈 性能回归检测

### 算法性能（CI中）
```rust
// tests/benches/algorithm_benches.rs
#[cfg(test)]
mod bench_tests {
    use super::*;

    #[test]
    fn bench_modbus_encode_performance() {
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            encode_modbus_read(0x01, 0x0000, 10);
        }
        let duration = start.elapsed();

        // 性能回归检测：如果超过阈值则失败
        assert!(duration.as_millis() < 100, "Modbus encoding too slow");
    }
}
```

## 📝 开发指南

### 本地开发（无硬件）
```bash
# 1. 正常开发和测试
cargo test

# 2. 专注于算法性能
cargo test --release protocol_*

# 3. 使用虚拟串口测试（可选）
socat -d -d pty,raw,echo=0 pty,raw,echo=0 &
SERIAL_PORT=/dev/pts/0 cargo test --release integration_tests
```

### 准备发布（有硬件）
```bash
# 1. 完整测试套件
cargo test --release

# 2. 在真实设备上验证
RUST_LOG=debug cargo run --release -- list-ports
RUST_LOG=debug cargo run --release -- interactive

# 3. 性能验证
# 使用示波器或逻辑分析仪验证实际性能

# 4. 长时间稳定性测试
# 让工具运行24小时，检查内存泄漏和稳定性
```

## 🔧 调试性能问题

### 性能分析工具
```bash
# 1. CPU性能分析
cargo install flamegraph
cargo flamegraph --bin serial-cli -- list-ports

# 2. 内存分析
cargo install valgrind
cargo valgrind test

# 3. 堆分析
cargo install heaptrack
heaptrack target/release/serial-cli
```

## 📚 参考资源

- [Rust Benchmark Guide](https://doc.rust-lang.org/stable/cargo/reference/benchmarks.html)
- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/index.html)
- [Serial Port Testing](https://en.wikibooks.org/wiki/Serial_Programming/Testing)

---

**总结**：Serial CLI采用分层测试策略，大部分性能测试可以在无硬件环境下运行，硬件测试在发布前或定期执行。
