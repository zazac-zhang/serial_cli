# Frontend TODO

> 最后更新: 2026-04-15
> 当前完成度: ~95%（Phase 1-3 全部完成）

---

## UI 存在但功能缺失（Bug 级）

### 1. Auto-scroll 复选框纯装饰

**文件**: `DataViewer.tsx:514-521`

`autoScroll` 仅绑定 checkbox 状态，从未用于实际滚动行为。收到新数据包时不会自动滚动到底部。

**修复**: 添加 `useRef` 指向数据列表容器，`packets` 变化时若 `autoScroll=true` 则 `scrollTo({ top: scrollHeight })`

### 2. 协议验证状态不展示

**文件**: `ProtocolPanel.tsx:58`

`validationStatus` Map 在加载协议时设置 `'valid'` / `'invalid'`，但 JSX 从未读取。用户上传 `.lua` 后看不到验证反馈。

**修复**: 在自定义协议列表卡片上显示验证状态 badge

### 3. DataViewer 显示设置不写回 SettingsContext

**文件**: `DataViewer.tsx:505-513`

Timestamp checkbox 直接改 `DataContext.displayOptions`，不写回 `SettingsContext`。用户在 DataViewer 关时间戳 → 切到 Settings 看到仍是开启 → 刷新后丢失。

**修复**: `setDisplayOptions` 时同步调用 `updateSettings({ display: { showTimestamp: checked } })`

### 4. 导出格式偏好不持久化

**文件**: `DataViewer.tsx:99-100`

`exportFormat` 和 `exportOption` 每次打开重置为 `txt` / `all`，不记忆用户上次选择。

**修复**: 从 localStorage 初始化或存入 Settings 的 display 字段

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
