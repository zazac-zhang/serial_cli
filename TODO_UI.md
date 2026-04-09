# UI 改进任务清单

## 📊 总体进度

- **前端 UI 完成度**: 90%
- **功能完成度**: 45%
- **后端集成**: 10%
- **总体完成度**: ~50%

---

## ✅ 已完成功能

### 1. 设计系统重构
- [x] 安装并配置 `lucide-react` 图标库
- [x] 重构 Sidebar 组件，替换 Emoji 为 SVG 图标
- [x] 增强 Panel 组件视觉设计（彩色强调条、折叠功能）
- [x] 重构 TopBar 组件，增强赛博风格
- [x] 优化 Toast 通知组件（类型图标、左侧彩色条）
- [x] 添加页面切换动画（fade-in + slide-up）

### 2. 功能面板 UI 实现
- [x] DataViewer - 实时数据表格、统计卡片、导出功能
- [x] ScriptPanel - Monaco Editor 集成、文件管理 UI
- [x] ProtocolPanel - 内置协议列表、自定义协议加载 UI
- [x] SettingsPanel - 多标签设置、完整表单控件
- [x] NotificationSettings - 通知权限和偏好设置

### 3. 辅助功能
- [x] CommandPalette - 命令面板 UI
- [x] KeyboardShortcutsHelp - 快捷键帮助
- [x] 全局快捷键系统

---

## ❌ 待完成任务

### 🔴 P0 - 阻塞性问题（核心功能）

#### 1. 串口管理功能（PortsPanel）
**当前状态**: 仅展示端口列表，无法进行任何操作

**需要实现**:
- [ ] 串口打开/关闭功能
- [ ] 串口参数配置（波特率、数据位、停止位、校验位）
- [ ] 流控制设置（RTS/CTS）
- [ ] 串口状态显示（打开/关闭、连接状态）
- [ ] 串口数据收发基础 UI

**文件**: `frontend/src/components/ports/PortsPanel.tsx`

#### 2. Tauri 后端 API 集成
**当前状态**: 前端独立运行，无后端通信

**需要实现的 Tauri Commands**:
```rust
// src-tauri/src/commands/serial.rs
#[tauri::command]
async fn list_ports() -> Result<Vec<PortInfo>, String>
async fn open_port(port_name: String, config: SerialConfig) -> Result<String, String>
async fn close_port(handle: String) -> Result<(), String>
async fn write_port(handle: String, data: Vec<u8>) -> Result<usize, String>
async fn read_port(handle: String) -> Result<Vec<u8>, String>
```

**文件**: `src-tauri/src/commands/serial.rs`

#### 3. 数据流实现
**当前状态**: DataContext 使用模拟数据

**需要实现**:
- [ ] 从 Tauri 接收实时串口数据
- [ ] 使用 Tauri 事件系统监听数据
- [ ] 更新 DataContext 以处理真实数据
- [ ] 实现 TX/RX 数据流区分

**文件**:
- `frontend/src/contexts/DataContext.tsx`
- `src-tauri/src/events.rs`

---

### 🟡 P1 - 重要功能

#### 4. 脚本执行系统
**当前状态**: ScriptPanel 只有模拟执行

**需要实现**:
- [ ] 移除 CommandPalette 中的 TODO 注释
  - `// TODO: Implement new script logic`
  - `// TODO: Create new script`
  - `// TODO: Run current script`
- [ ] 集成 LuaJIT 执行引擎
- [ ] 实现脚本运行/停止功能
- [ ] 捕获脚本输出并显示
- [ ] 实现串口 API 绑定（serial.write, serial.read 等）

**文件**:
- `frontend/src/components/scripting/ScriptPanel.tsx`
- `frontend/src/hooks/useGlobalShortcuts.ts`
- `src-tauri/src/commands/lua.rs`

#### 5. 数据持久化
**当前状态**: 刷新页面后所有数据丢失

**需要实现**:
- [ ] 设置保存到 localStorage
- [ ] 脚本文件持久化
- [ ] 协议配置持久化
- [ ] 最近使用的串口配置
- [ ] 应用状态恢复

**文件**:
- `frontend/src/lib/storage.ts` (新建)
- `frontend/src/contexts/SettingsContext.tsx` (新建)

#### 6. 协议系统
**当前状态**: ProtocolPanel 只展示，无实际功能

**需要实现**:
- [ ] 解析自定义 Lua 协议文件
- [ ] 协议激活/停用逻辑
- [ ] 协议参数配置界面
- [ ] 内置协议的实际加载

**文件**:
- `frontend/src/components/protocols/ProtocolPanel.tsx`
- `src-tauri/src/protocols/`

#### 7. 实时数据流可视化
**当前状态**: TopBar 动画是静态的

**需要实现**:
- [ ] TopBar 数据流指示器显示真实流量
- [ ] 添加波特率显示
- [ ] 添加错误率统计
- [ ] 数据流量图表（可选）

**文件**: `frontend/src/components/layout/TopBar.tsx`

---

### 🔵 P2 - 增强功能

#### 8. 通知系统
**当前状态**: UI 完整，但通知未实际发送

**需要实现**:
- [ ] 集成 Tauri 通知插件
- [ ] 实现端口连接/断开通知
- [ ] 实现错误通知
- [ ] 实现脚本完成通知

**文件**:
- `frontend/src/contexts/NotificationContext.tsx`
- `src-tauri/src/commands/notifications.rs`

#### 9. 数据导出增强
**当前状态**: 仅有基础文本导出

**需要实现**:
- [ ] CSV 格式导出
- [ ] JSON 格式导出
- [ ] 导出选项（时间范围、方向筛选）
- [ ] 自动导出功能

**文件**: `frontend/src/components/data/DataViewer.tsx`

#### 10. 串口配置预设
**需要实现**:
- [ ] 常用设备配置预设
- [ ] 自定义配置保存
- [ ] 配置导入/导出

**文件**: `frontend/src/components/ports/PortConfig.tsx` (新建)

---

## 🎯 实施计划

### 阶段 1: 核心串口功能（1-2 周）
1. 实现 Tauri 串口 commands
2. 完成 PortsPanel 功能
3. 实现数据流和事件系统
4. 基础的发送/接收功能

### 阶段 2: 脚本和协议（1 周）
1. 集成 LuaJIT
2. 实现脚本执行
3. 实现协议加载和解析
4. 移除所有 TODO 标记

### 阶段 3: 数据和设置（3-5 天）
1. 数据持久化
2. 设置系统
3. 通知系统
4. 配置预设

### 阶段 4: 增强功能（1 周）
1. 数据导出
2. 高级可视化
3. 性能优化
4. 测试和修复

---

## 📁 关键文件清单

### 前端组件
- `frontend/src/components/ports/PortsPanel.tsx` - 需要重大增强
- `frontend/src/components/scripting/ScriptPanel.tsx` - 需要集成真实执行
- `frontend/src/components/protocols/ProtocolPanel.tsx` - 需要协议加载逻辑
- `frontend/src/components/data/DataViewer.tsx` - 需要真实数据源
- `frontend/src/components/layout/TopBar.tsx` - 需要实时流量显示

### Context 和 Hooks
- `frontend/src/contexts/DataContext.tsx` - 需要连接 Tauri 事件
- `frontend/src/contexts/PortContext.tsx` - 需要连接 Tauri commands
- `frontend/src/hooks/useGlobalShortcuts.ts` - 移除 TODO

### 后端（需要新建）
- `src-tauri/src/commands/mod.rs` - 命令入口
- `src-tauri/src/commands/serial.rs` - 串口命令
- `src-tauri/src/commands/lua.rs` - Lua 脚本命令
- `src-tauri/src/events.rs` - 事件系统
- `src-tauri/src/protocols/mod.rs` - 协议管理

---

## 🔧 技术债务

### 代码清理
- [ ] 移除所有模拟数据和 setTimeout
- [ ] 统一错误处理
- [ ] 添加 TypeScript 严格类型
- [ ] 移除未使用的 imports

### 性能优化
- [ ] 大数据量时的虚拟滚动
- [ ] Monaco Editor 懒加载
- [ ] 数据包缓存策略

### 测试
- [ ] 单元测试（关键函数）
- [ ] 集成测试（Tauri commands）
- [ ] E2E 测试（关键流程）

---

## 📝 备注

### 设计规范（保持不变）
- 色彩系统: signal (#00ff41), alert (#ff4757), amber (#ffb142), info (#53a0fd)
- 字体: Instrument Sans, JetBrains Mono, Instrument Serif
- 图标: lucide-react, strokeWidth={1.5}
- 动画: fade-in, slide-up, pulse-slow

### 依赖版本
- lucide-react: ^0.344.0 ✅
- @monaco-editor/react: ^4.6.0 ✅
- @tauri-apps/api: ^2.0.2 ✅
- react-hotkeys-hook: ^5.2.4 ✅

---

**最后更新**: 2025-04-09
**状态**: ✅ P0/P1 任务完成，P2 任务进行中
**总体完成度**: ~87%
