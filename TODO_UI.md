# TODO UI

## 待完成任务

### 🔴 P0 - 核心功能

#### 1. 脚本执行系统
**状态**: ScriptPanel 有 UI，需要集成真实 LuaJIT 执行

**需要实现**:
- [ ] 前端调用 `execute_script` 命令
- [ ] 实现脚本运行/停止功能
- [ ] 串口 API 绑定（serial.write, serial.read）

**文件**: 
- `frontend/src/components/scripting/ScriptPanel.tsx`
- `src-tauri/src/commands/lua.rs`

---

### 🟡 P1 - 增强功能

#### 2. 通知系统
**状态**: UI 完整，需要集成 Tauri 通知插件

**需要实现**:
- [ ] 端口连接/断开通知
- [ ] 错误通知
- [ ] 脚本完成通知

**文件**: `src-tauri/src/commands/notifications.rs`

#### 3. 数据导出增强
**状态**: 基础导出已实现

**需要实现**:
- [ ] 导出选项 UI（时间范围、方向筛选）
- [ ] 自动导出功能

---

### 🔵 P2 - 可选功能

#### 4. 串口配置预设
- [ ] 常用设备配置预设
- [ ] 配置导入/导出

#### 5. 性能优化
- [ ] 大数据量时的虚拟滚动（DataViewer）

---

## ✅ 已完成功能

### 核心功能
- [x] 串口管理（打开/关闭/配置）- PortsPanel
- [x] Tauri 后端 API 集成 - list_ports, open_port, close_port
- [x] 数据流系统 - DataContext 监听 Tauri 事件
- [x] 串口状态监控 - PortContext heartbeat

### UI 组件
- [x] DataViewer - 实时数据、统计、导出
- [x] ScriptPanel - Monaco Editor、文件管理
- [x] ProtocolPanel - 协议列表 UI
- [x] SettingsPanel - 多标签设置
- [x] NotificationSettings - 通知偏好设置

### 辅助功能
- [x] CommandPalette - 命令面板
- [x] KeyboardShortcutsHelp - 快捷键帮助
- [x] 全局快捷键系统
- [x] 数据持久化 - SettingsContext + storage.ts
- [x] 错误处理 - 统一的错误展示

---

**最后更新**: 2026-04-09
**总体完成度**: ~95%
