# Frontend TODO

> 最后更新: 2026-04-15
> 当前完成度: ~98%

---

## Phase 4 — 功能增强（可选）

### 数据过滤/搜索

**涉及**: `DataViewer.tsx`
**缺失**: 按端口/方向过滤、内容搜索/高亮

### 快捷发送预设管理

**涉及**: `DataViewer.tsx`
**现状**: 硬编码 3 个按钮
**建议**: 可自定义预设列表，持久化到 localStorage

### 多端口同时发送

**涉及**: `DataViewer.tsx`
**现状**: 一次只能选一个目标端口
**建议**: 支持多选端口批量发送

### 协议编码/解码 UI

**涉及**: `DataViewer.tsx`, `ProtocolPanel.tsx`
**缺失**: 后端有 `protocol_encode` / `protocol_decode`
**建议**: 发送面板添加"使用当前协议编码"按钮

### 脚本预检（validate_script）

**涉及**: `ScriptPanel.tsx`
**缺失**: 后端有 `validate_script`
**建议**: 运行前自动预检

### 窗口状态持久化

**涉及**: `useWindow.ts`, `App.tsx`
**问题**: `windowStateStorage` 已定义但从未使用
