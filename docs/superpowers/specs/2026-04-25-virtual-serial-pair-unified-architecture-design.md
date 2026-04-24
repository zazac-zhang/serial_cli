# VirtualSerialPair 统一架构设计

**Date**: 2026-04-25
**Status**: Draft

## Problem

`VirtualSerialPair::create()` 仅实现了 PTY 后端（Unix `libc::posix_openpt`），NamedPipe 和 Socat 分支返回 `"not yet implemented via old API"`。CLI `virtual create` 只能使用 PTY。

根因：bridge 逻辑（`tokio::select!` 轮询 + `libc::read/write`）耦合在 `VirtualSerialPair` 中，`VirtualBackend` trait 只有 `create_pair()/cleanup()/is_healthy()`，不包含 bridge。

## Solution

### Step 1: 扩展 VirtualBackend trait

在 `src/serial_core/backends/trait.rs` 的 `VirtualBackend` trait 中新增：

```rust
/// Start the bridge between the two port ends
/// Returns (path_a, path_b, error_rx, stats)
async fn start_bridge(
    &mut self,
    buffer_size: usize,
    capture: Option<Arc<Mutex<PacketCapture>>>,
) -> Result<(String, String, mpsc::Receiver<String>, Arc<Mutex<BackendStats>>)>;

/// Signal the bridge to stop
fn stop_bridge(&mut self);

/// Wait for bridge task to finish (with timeout)
async fn wait_bridge(&mut self, timeout: Duration) -> bool;
```

### Step 2: 迁移 PtyBackend bridge 逻辑

当前 `virtual_port.rs` 中的 `create_pty_pair()` bridge 代码（line 261-531）已经有一份几乎相同的副本在 `backends/pty.rs` 中。将 `virtual_port.rs` 中的版本删除，使用 `backends/pty.rs` 的版本作为唯一的 PTY bridge 实现。

在 `PtyBackend` 上实现 `start_bridge()` / `stop_bridge()` / `wait_bridge()`。

### Step 3: NamedPipe backend 实现 bridge

NamedPipe（Windows）的 bridge 需要在两个 pipe 端点之间转发数据。实现方式：

- `create_pair()` 创建 pipe 后不启动 bridge
- `start_bridge()` 使用 `tokio::net::windows::named_pipe`（Windows tokio 原生支持）在两个 pipe 间建立 `tokio::select!` bridge
- `stop_bridge()` 设置 running=false，`wait_bridge()` 等待退出

非 Windows 平台的 `NamedPipeBackend` stub 同样实现 trait 方法（返回错误）。

### Step 4: Socat backend 实现 bridge

Socat 的 bridge 由 socat 进程自身完成（`socat pty,link=A pty,link=B` 会自动在两端之间桥接）。

- `create_pair()` 已经启动了 socat 进程 — socat 就是 bridge
- `start_bridge()` 只需等待 socat 启动完成（已有的 200ms sleep + symlink 验证），返回端口路径。不需要额外的 bridge task。
- `stop_bridge()` 不需要操作（socat 在 cleanup 中 kill）
- `wait_bridge()` 检查 socat 进程是否仍在运行

### Step 5: 重构 VirtualSerialPair

将 `VirtualSerialPair` 字段替换为：

```rust
pub struct VirtualSerialPair {
    pub id: String,
    pub port_a: String,
    pub port_b: String,
    pub backend_type: BackendType,  // 不再持有 backend 实例
    sniffer: Option<SerialSniffer>,
    running: bool,
    pub created_at: SystemTime,
    stats: Arc<Mutex<VirtualPairStats>>,
    capture: Option<Arc<Mutex<PacketCapture>>>,
    backend: Option<Box<dyn VirtualBackend>>,  // 委托给后端
    error_rx: Option<mpsc::Receiver<String>>,  // bridge error channel
    bridge_monitor_task: Option<JoinHandle<()>>,  // 监控 bridge error 的 task
}
```

`VirtualSerialPair::create()` 改为：
1. 通过 `BackendFactory::create_backend()` 获取 `Box<dyn VirtualBackend>`
2. 调用 `backend.create_pair()` 获取端口路径
3. 调用 `backend.start_bridge(buffer_size, capture)` 启动 bridge
4. 接收 `error_rx`，spawn error monitoring task
5. 返回 `VirtualSerialPair`

`VirtualSerialPair::stop()` 改为：
1. 调用 `backend.stop_bridge()`
2. 调用 `backend.wait_bridge(100ms)`
3. 调用 `backend.cleanup()`
4. 保存 capture 数据

### Step 6: CLI handler 无需改动

CLI handler 继续调用 `VirtualSerialPair::create(config)`，现在所有后端都会正常工作。

## Files Changed

| File | Change |
|------|--------|
| `backends/trait.rs` | 扩展 trait，添加 bridge 方法 |
| `backends/pty.rs` | 实现 bridge trait 方法 |
| `backends/named_pipe.rs` | 实现 bridge（Windows: tokio named_pipe, 非 Windows: stub） |
| `backends/socat.rs` | 实现 bridge（socat 自身就是 bridge） |
| `virtual_port.rs` | 重构：持有 `Box<dyn VirtualBackend>`，删除 `create_pty_pair()` |
| `virtual_port.rs` tests | 更新测试适配新结构 |

## Risk Assessment

- **PTY**: 低风险 — bridge 代码已存在于 pty.rs，只是迁移
- **Socat**: 低风险 — socat 自身处理 bridge
- **NamedPipe**: 中风险 — Windows 平台，需要 tokio named_pipe API 验证；非 Windows stub 简单返回错误
- **Regression**: 现有的 PTY 功能需要确保重构后行为一致

## Success Criteria

- `serial-cli virtual create --backend socat` 在 macOS 上成功创建一对虚拟端口
- `serial-cli virtual create`（auto 检测 PTY）行为与重构前一致
- `serial-cli virtual stop <id>` 正确清理所有后端资源
- `just test` 全部通过
