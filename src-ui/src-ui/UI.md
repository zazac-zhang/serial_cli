# Serial CLI - GUI 设计文档

## 概述

Serial CLI GUI 是基于 **Tauri 2.x + SolidJS** 构建的跨平台串口管理工具，采用独特的**赛博工业风格**设计，提供强大的串口通信、数据监控和脚本管理功能。

---

## 技术栈

### 后端 (Rust + Tauri)
- **Tauri 2.x**: 轻量级桌面应用框架 (~10MB vs Electron ~150MB)
- **复用现有代码**: 完全集成现有的 6,700+ 行 Rust 代码库
- **异步 I/O**: 基于 Tokio 的高性能串口通信
- **事件系统**: Rust → Frontend 实时事件推送

### 前端 (SolidJS + TypeScript)
- **SolidJS**: 细粒度响应式系统，适合高频数据更新
- **TypeScript**: 类型安全的状态管理
- **Vite**: 快速开发构建工具
- **包大小**: ~10KB (SolidJS core)

---

## 设计系统

### 配色方案

```css
/* 主色调 */
--color-cyan: #00F0FF;        /* 主操作色 */
--color-magenta: #FF006E;     /* 强调色 */
--color-purple: #8338EC;      /* 辅助色 */
--color-dark: #0a0a0f;        /* 主背景 */
--color-darker: #050508;      /* 深色背景 */

/* UI 颜色 */
--color-bg: var(--color-dark);
--color-bg-secondary: #12121a;
--color-bg-elevated: #1a1a25;
--color-border: #2a2a3a;
--color-text: #e0e0e0;
--color-text-muted: #808090;

/* 状态颜色 */
--color-success: #00ff88;
--color-warning: #ffaa00;
--color-error: #ff3366;
```

### 字体系统

```css
/* 标题字体 - Orbitron (科技感) */
--font-display: 'Orbitron', sans-serif;

/* 数据/代码 - JetBrains Mono (等宽) */
--font-mono: 'JetBrains Mono', monospace;

/* 正文 - Inter (清晰易读) */
--font-body: 'Inter', system-ui, sans-serif;
```

### 动画效果

```css
/* 过渡时间 */
--transition-fast: 150ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-base: 250ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-slow: 350ms cubic-bezier(0.4, 0, 0.2, 1);

/* 特效 */
--shadow-glow: 0 0 20px rgba(0, 240, 255, 0.3);
--shadow-glow-strong: 0 0 30px rgba(0, 240, 255, 0.5);
```

---

## 架构设计

### 项目结构

```
serial_cli/
├── src/                    # Rust 库代码 (现有)
├── src-tauri/              # Tauri 后端
│   ├── src/
│   │   ├── main.rs        # Tauri 入口
│   │   ├── commands/      # Tauri 命令 (Rust → Frontend)
│   │   │   ├── port.rs    # 端口管理
│   │   │   ├── serial.rs  # 串口 I/O
│   │   │   ├── protocol.rs # 协议管理
│   │   │   ├── script.rs  # Lua 脚本
│   │   │   └── config.rs  # 配置管理
│   │   ├── state/         # 全局状态
│   │   └── events/        # 事件系统
├── src-ui/                 # SolidJS 前端
│   ├── src/
│   │   ├── components/    # UI 组件
│   │   │   ├── layout/    # 布局组件
│   │   │   ├── ports/     # 端口管理
│   │   │   ├── data/      # 数据显示
│   │   │   ├── commands/  # 命令输入
│   │   │   ├── scripting/ # 脚本编辑
│   │   │   └── protocols/ # 协议管理
│   │   ├── stores/        # SolidJS 状态管理
│   │   ├── hooks/         # 自定义 hooks
│   │   └── styles/        # 样式文件
```

### 数据流架构

```
[Frontend UI] ←→ [Tauri Commands] ←→ [Rust Modules]
     ↓                                ↓
[SolidJS Stores]              [serial_core, protocol, lua]
     ↓
[Event Emitter] ←→ [Async I/O] ←→ [Serial Ports]
```

---

## 核心组件

### 1. 导航系统

#### navigationStore.ts
- **功能**: 全局导航状态管理
- **视图**: ports, data, scripts, protocols, settings
- **API**: `setCurrentView()`, `getCurrentView()`

#### Sidebar.tsx
- **功能**: 侧边栏导航菜单
- **特性**:
  - 使用 For 组件渲染菜单项
  - 点击切换视图
  - 活动状态高亮显示
  - 响应式状态同步

### 2. 端口管理 (Phase 2 ✅)

#### PortList.tsx
- **功能**: 自动刷新端口列表（3秒间隔）
- **特性**:
  - 实时端口状态指示（打开/关闭）
  - 端口类型显示（USB/虚拟）
  - 点击选择端口
  - 刷新按钮

#### PortConfig.tsx
- **功能**: 端口参数配置对话框
- **配置项**:
  - 波特率: 9600 - 921600
  - 数据位: 5, 6, 7, 8
  - 停止位: 1, 2
  - 校验位: None, Odd, Even
  - 流控制: None, Software, Hardware
  - 超时: 1-5000ms

#### PortsPanel.tsx
- **功能**: 端口管理主面板
- **集成**: PortList + PortConfig + 状态显示

### 3. 命令输入 (Phase 2 ✅)

#### CommandInput.tsx
- **功能**: 快速命令发送
- **特性**:
  - 支持 Hex (0x01 0x02) 和 ASCII (AT+CMD)
  - 命令历史记录（↑↓ 导航）
  - Enter 发送，Shift+Enter 换行
  - 连接状态指示
  - 发送中状态

### 4. 数据显示 (Phase 2 ✅)

#### DataViewer.tsx
- **功能**: 实时数据监控
- **显示模式**:
  - Hex: 十六进制显示
  - ASCII: 字符显示
  - Both: 同时显示
- **特性**:
  - 数据过滤
  - 自动滚动
  - 时间戳
  - TX/RX 标识
  - 数据包大小

### 5. 事件系统 (Phase 2 ✅)

#### useEvents.ts
- **监听事件**:
  - `data-received`: 数据接收
  - `data-sent`: 数据发送
  - `port-status-changed`: 端口状态变化
  - `error-occurred`: 错误发生
- **自动清理**: 组件卸载时移除监听器

---

## 状态管理

### SolidJS Stores

#### navigationStore.ts
```typescript
interface NavigationStore {
  currentView: NavigationView;
}
```

**Actions**:
- `setCurrentView(view)`: 设置当前视图
- `getCurrentView()`: 获取当前视图

#### portStore.ts
```typescript
interface PortStore {
  availablePorts: SerialPort[];      // 可用端口列表
  activePorts: Map<string, PortStatus>; // 活动端口
  selectedPortId: string | null;     // 当前选中端口
  isLoading: boolean;
  error: string | null;
}
```

**Actions**:
- `listPorts()`: 列出可用端口
- `openPort(portName, config)`: 打开端口
- `closePort(portId)`: 关闭端口
- `getPortStatus(portId)`: 获取端口状态

#### dataStore.ts
```typescript
interface DataStore {
  packets: DataPacket[];             // 数据包列表
  displayOptions: DisplayOptions;    // 显示选项
  isMonitoring: boolean;             // 监控状态
  filter: string;                    // 数据过滤
}
```

**Actions**:
- `addPacket(packet)`: 添加数据包
- `clearPackets()`: 清空数据
- `setDisplayOptions(options)`: 设置显示选项
- `toggleMonitoring()`: 切换监控
- `setFilter(filter)`: 设置过滤

---

## Tauri 命令接口

### 端口命令
```rust
list_ports() -> Result<Vec<SerialPort>, String>
open_port(port_name: String, config: SerialConfig) -> Result<String, String>
close_port(port_id: String) -> Result<(), String>
get_port_status(port_id: String) -> Result<PortStatus, String>
```

### 串口命令
```rust
send_data(port_id: String, data: Vec<u8>) -> Result<usize, String>
read_data(port_id: String) -> Result<Vec<u8>, String>
start_sniffing(port_name: String) -> Result<(), String>
stop_sniffing() -> Result<(), String>
```

### 协议命令
```rust
list_protocols() -> Result<Vec<ProtocolInfo>, String>
load_protocol(path: String) -> Result<(), String>
unload_protocol(name: String) -> Result<(), String>
get_protocol_info(name: String) -> Result<ProtocolInfo, String>
```

### 脚本命令
```rust
execute_script(script: String) -> Result<String, String>
validate_script(script: String) -> Result<bool, String>
```

### 配置命令
```rust
get_config() -> Result<ConfigData, String>
update_config(config: ConfigData) -> Result<(), String>
```

---

## 开发进度

### ✅ Phase 1: 基础架构 (已完成)
- Tauri + SolidJS 项目搭建
- 设计系统实现
- 基础 UI 组件（Sidebar, TopBar, Panel）

### ✅ Phase 2: 核心功能 (已完成)
- 端口管理界面
- 命令输入组件
- 数据显示组件
- 事件系统集成
- **导航系统** (2026-04-05 新增)

### 🔄 Phase 3: 高级功能 (待开发)
- Lua 脚本编辑器（Monaco Editor）
- 协议管理界面
- 增强数据分析
- 虚拟滚动优化

### 📋 Phase 4: 优化完善 (待开发)
- 性能优化
- 动画效果
- 键盘快捷键
- 测试和文档

---

## 性能优化策略

### 前端优化
1. **细粒度响应式**: SolidJS 仅更新变化的数据
2. **虚拟滚动**: 处理大量数据包（1000+）
3. **数据批量更新**: 50-100ms 批处理
4. **Web Workers**: 数据转换离主线程

### 后端优化
1. **异步 I/O**: Tokio 多线程串口通信
2. **事件批处理**: 减少前端更新频率
3. **零拷贝**: 数据传输避免复制

---

## 开发命令

### 构建和运行
```bash
# 开发模式
cargo tauri dev

# 生产构建
cargo tauri build

# 仅前端
cd src-ui && npm run dev
```

### 测试
```bash
# Rust 测试
cargo test

# 前端测试
cd src-ui && npm test
```

---

## 设计亮点

### 🎨 独特视觉
- **赛博工业风格**: 避免通用的紫色渐变 + 白色背景
- **科技感字体**: Orbitron 标题 + JetBrains Mono 数据
- **动画效果**: 渐变背景、发光边框、平滑过渡

### ⚡ 高性能
- **轻量级**: Tauri (~10MB) vs Electron (~150MB)
- **细粒度响应**: SolidJS 优化高频数据更新
- **原生性能**: Rust 后端复用现有代码

### 🔧 易维护
- **模块化**: 组件职责单一
- **类型安全**: TypeScript 全覆盖
- **代码复用**: 直接使用现有 Rust 库

---

## 未来计划

### 短期 (Phase 3)
- [ ] Monaco 编辑器集成
- [ ] 协议管理 UI
- [ ] 数据导出功能
- [ ] 虚拟滚动优化

### 中期 (Phase 4)
- [ ] 键盘快捷键
- [ ] 主题切换
- [ ] 工具提示系统
- [ ] 性能监控

### 长期
- [ ] 插件系统
- [ ] 数据可视化
- [ ] 协议自动识别
- [ ] 多语言支持

---

**文档版本**: 1.0
**最后更新**: 2026-04-05
**作者**: Serial CLI Team
