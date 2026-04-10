# Serial CLI 待修复功能清单

> 核心功能已基本完成，以下为待修复和优化项

## 总体进度：75% 完成 ⚠️
**状态**: Beta版本，测试有失败，非生产就绪

---

## 🚨 紧急修复（P0 - 阻塞发布）

### 1. Lua集成测试失败 [阻塞]
**位置**: `tests/lua_integration_tests.rs`

**失败测试**:
- `test_protocol_api_integration` - assertion failed
- `test_end_to_end_modbus_workflow` - Lua runtime error

**影响**: 生产环境可能出问题，Lua集成功能不稳定

**修复方案**:
- 检查Lua bindings的execute_script实现
- 修复runtime错误处理
- 验证协议API集成逻辑

---

### 2. Windows信号控制未实现 [阻塞]
**位置**: `src/serial_core/signals.rs:299-308`

**问题**: WindowsSignalController只更新状态，没有实际调用硬件API
```rust
pub fn set_dtr(&mut self, enable: bool) -> Result<SignalState> {
    // Windows implementation will use EscapeCommFunction
    // For now, we update the state and log  // ⚠️ 假装成功
    tracing::debug!("DTR signal state updated to {} on Windows", enable);
    Ok(SignalState::Set(enable))
}
```

**影响**: Windows用户无法真正控制DTR/RTS信号

**修复方案**:
- 实现EscapeCommFunction调用
- 使用winapi绑定或windows-rs crate

---

### 3. 清理未使用代码 [紧急]
**位置**: 多个文件

**未使用的代码**:
- `error_handling.rs:157` - `include_stack_trace` 字段
- `signals.rs:66` - `validate_fd()` 方法
- `signals.rs:160` - `set_modem_bit()` 方法
- `sniffer.rs:249` - `display_packet()` 方法
- `sniffer.rs:215` - `display_enabled` 字段

**修复方案**:
- 使用 `#[allow(dead_code)]` 或实现功能
- 删除未使用的方法

---

## 🔧 高优先级修复（P1 - 本周完成）

### 4. 错误恢复策略未实现 [高优先级]
**位置**: `src/error_handling.rs:380-386`

**问题**: `RecoveryStrategy::Fallback` 返回错误而不是fallback值
```rust
RecoveryStrategy::Fallback => {
    Err(SerialError::Serial(SerialPortError::IoError(
        "Fallback not implemented".to_string()
    )))
}
```

**修复方案**:
- 实现真实的fallback逻辑
- 或删除此选项

---

### 5. Lua性能优化 [中优先级]
**位置**: `src/protocol/lua_ext.rs:57-60`

**问题**: 每次调用创建新Lua实例
```rust
fn execute_callback(&self, callback_name: &str, data: &[u8]) -> Result<Vec<u8>> {
    let lua = Lua::new();  // ⚠️ 每次创建新实例
    lua.load(script).exec()?;
}
```

**影响**: 高频场景性能开销大

**修复方案**:
- 添加Lua实例池
- 缓存编译后的脚本

---

### 6. 任务并发测试不足 [中优先级]
**位置**: `src/task/executor.rs`

**问题**: 虽然有Mutex保护，但缺乏并发压力测试

**修复方案**:
- 添加并发任务测试
- 验证竞态条件修复

---

## 📝 低优先级改进（P2 - 下个版本）

### 7. Windows监控功能缺失
**位置**: `src/monitoring.rs`

**问题**: Windows平台内存监控返回0
```rust
#[cfg(windows)]
fn get_memory_usage_windows() -> usize {
    // Windows implementation would use GetProcessMemoryInfo
    // For now, return 0  // ⚠️ 未实现
}
```

**修复方案**:
- 实现GetProcessMemoryInfo调用
- 或使用windows-rs的system-info APIs

---

### 8. IoLoop任务取消机制
**位置**: `src/serial_core/port.rs` (io_loop相关)

**问题**: 异步任务缺乏AbortHandle，可能无法正确取消

**修复方案**:
- 添加AbortHandle到任务spawn
- 实现优雅的取消逻辑

---

## ✅ 已修复功能

- [x] 串口管理（打开/关闭/配置/状态）
- [x] **DTR/RTS控制** - Unix平台完整实现 ✅
- [x] **协议系统** - ProtocolManager注册已修复 ✅
- [x] **数据包嗅探** - 完整capture逻辑已实现 ✅
- [x] **任务执行器** - 并发安全性改善 ✅
- [x] Lua集成（需要修复测试）
- [x] CLI交互模式
- [x] 配置管理
- [x] 批处理功能
- [x] 实用工具模块
- [x] 性能监控模块
- [x] 增强错误处理
- [x] 单元测试（463个通过，2个失败）

---

## 📊 质量指标

| 指标 | 当前状态 | 目标状态 |
|------|----------|----------|
| 测试通过率 | 99.5% (463/465) | 100% |
| 代码警告 | 5个未使用字段/方法 | 0 |
| 功能完整度 | 75% | 90% |
| 平台支持 | Unix: 85%, Windows: 60% | Unix: 95%, Windows: 85% |

---

## 🎯 修复计划

### 阶段1: 紧急修复（2-3天）
1. 修复Lua集成测试失败
2. 实现Windows信号控制
3. 清理未使用代码

### 阶段2: 稳定化（1周）
4. 实现错误恢复fallback
5. 添加并发测试
6. Lua性能优化

### 阶段3: 完善功能（2周）
7. Windows监控实现
8. IoLoop取消机制

---

**最后更新**: 2026-04-10
**状态**: Beta版，需要修复阻塞问题后才能发布RC
**阻塞问题**: 3个（Lua测试、Windows信号、未使用代码）
