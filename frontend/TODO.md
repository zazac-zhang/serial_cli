# Frontend TODO

> 最后更新: 2026-04-15
> 当前完成度: ~95%（Phase 1-3 全部完成）

---

## 已完成 (Phase 1-3)

| # | 修复 | 状态 |
|---|------|------|
| 1 | ToggleSwitch 改为受控组件 | ✅ |
| 2 | SettingsPanel 连接 SettingsContext | ✅ |
| 3 | DataContext 初始化读取 settings | ✅ |
| 4 | ProtocolPanel useState→useEffect | ✅ |
| 5 | ProtocolPanel toggleProtocol 调后端 | ✅ |
| 6 | ScriptPanel 启用 ScriptActionContext | ✅ |
| 7 | useGlobalShortcuts 使用 Context | ✅ |
| 8 | shortcuts.ts 改为纯数据 | ✅ |
| 9 | 测试按钮调真实通知 | ✅ |
| 10 | SettingsPanel alert→Toast | ✅ |
| 11 | 活跃端口设置按钮 | ✅ |
| 12 | DataViewer autoScroll/maxPackets 从 settings 读取 | ✅ |
| 13 | 移除未使用的 sonner 依赖 | ✅ |
| 14 | SettingsPanel tab 添加图标 | ✅ |
| 15 | 导航持久化到 localStorage | ✅ |

5/5 核心用户路径全部打通。

---

## Phase 4 — 功能增强（可选）

### 数据过滤/搜索

**涉及**: `DataViewer.tsx`
**缺失**:
- 按端口过滤（只看某个端口的数据）
- 按方向过滤（只看 RX 或 TX）
- 数据内容搜索/高亮

### 快捷发送预设管理

**涉及**: `DataViewer.tsx`
**现状**: 硬编码 3 个按钮（Hello / AT / CRLF）
**建议**: 可自定义预设列表，持久化到 localStorage

### 多端口同时发送

**涉及**: `DataViewer.tsx`
**现状**: 一次只能选一个目标端口
**建议**: 支持多选端口批量发送

### 协议编码/解码 UI

**涉及**: `DataViewer.tsx`, `ProtocolPanel.tsx`
**缺失**: 后端有 `protocol_encode` / `protocol_decode`，前端无入口
**建议**: 在发送面板添加"使用当前协议编码"按钮

### 脚本预检（validate_script）

**涉及**: `ScriptPanel.tsx`
**缺失**: 后端有 `validate_script`，前端只在运行时才报错误
**建议**: 运行前自动预检，给出语法错误提示

### 窗口状态持久化

**涉及**: `useWindow.ts`, `App.tsx`
**问题**: `windowStateStorage` 已定义但从未被使用
**建议**: 在窗口变化时保存尺寸/位置，启动时恢复
