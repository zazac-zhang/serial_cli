# UI 改进任务清单

## 核心问题分析

### 1. 图标系统不统一
- **问题**: Sidebar 使用 Emoji 图标，与赛博工业风格不协调
- **解决**: 替换为 `lucide-react` SVG 图标，统一线性风格（1.5px 描边）

### 2. Panel 组件视觉层次不足
- **问题**: 设计过于简单，缺少深度和特征
- **解决**: 
  - 添加顶部状态条（彩色边框强调）
  - 增加微妙的内阴影和外阴影组合
  - 标题栏添加可折叠/操作按钮区域

### 3. TopBar 设计平淡
- **问题**: 状态指示器简单，窗口控制按钮样式普通
- **解决**: 
  - 添加实时数据流视觉指示器
  - 窗口控制按钮改为赛博风格（圆形带光圈）
  - 状态文字改为单色显示 + 彩色状态灯

### 4. 缺少微交互和动画
- **问题**: 定义了动画但未广泛使用
- **解决**: 
  - 页面切换添加淡入 + 轻微上移过渡
  - 侧边栏选中项添加左边界高亮条
  - 按钮 hover 状态增强

### 5. Toast 通知设计不够精致
- **问题**: 缺少图标，动画单一
- **解决**: 
  - 添加类型图标（✓ ! ⚠ ℹ）
  - 左侧彩色强调条
  - 滑入动画优化

### 6. 功能页面完成度低
- **问题**: DataViewer, ScriptPanel, ProtocolPanel, SettingsPanel 都是占位内容
- **解决**: 
  - 数据监控：实时图表/表格
  - 脚本编辑器：Monaco Editor 集成
  - 协议配置：表单 + 可视化配置器
  - 设置面板：完整表单控件

---

## 任务清单

- [x] **任务 1**: 安装并配置 `lucide-react` 图标库
- [x] **任务 2**: 重构 Sidebar 组件，替换 Emoji 为 SVG 图标
- [x] **任务 3**: 增强 Panel 组件视觉设计
- [x] **任务 4**: 重构 TopBar 组件，增强赛博风格
- [x] **任务 5**: 优化 Toast 通知组件
- [x] **任务 6**: 添加页面切换动画
- [x] **任务 7**: 实现 DataViewer 实时监控 UI
- [x] **任务 8**: 实现 ScriptPanel 脚本编辑器（Monaco）
- [x] **任务 9**: 实现 ProtocolPanel 协议配置
- [x] **任务 10**: 实现 SettingsPanel 完整设置表单

---

## 设计规范

### 色彩系统（已定义，保持不变）
```
signal: #00ff41 (黑客绿主色)
alert: #ff4757 (错误红)
amber: #ffb142 (警告黄)
info: #53a0fd (信息蓝)
```

### 字体系统（已定义，保持不变）
```
sans: Instrument Sans
mono: JetBrains Mono
display: Instrument Serif
```

### 新增图标规范
- 使用 `lucide-react` 图标
- 统一 `strokeWidth={1.5}`
- 尺寸：small=16px, medium=20px, large=24px
- 颜色跟随文本颜色 (`text-current`)

### 动画规范
- 页面切换：`fade-in` + `slide-up` 组合
- 按钮交互：`transition-all duration-200`
- 状态指示：`pulse-slow` (3s 循环)

---

## 已完成总结

### 已实现功能

1. **图标系统** - 全面替换 Emoji 为 lucide-react SVG 图标
2. **Sidebar** - 新的图标 + 改进的选中状态 + 优化的视觉效果
3. **Panel 组件** - 新增彩色强调条、折叠功能、操作按钮区域
4. **TopBar** - 数据流指示器、改进的状态显示、优化的窗口控制
5. **Toast** - 类型图标、左侧彩色条、更精致的视觉设计
6. **DataViewer** - 实时数据表格、统计卡片、导出功能、显示选项
7. **ScriptPanel** - Monaco Editor 集成、文件管理、运行/保存功能
8. **ProtocolPanel** - 内置协议列表、自定义协议加载、详情展示
9. **SettingsPanel** - 多标签设置、串口配置、数据设置、完整表单

### 修改文件列表

- `frontend/src/components/layout/Sidebar.tsx` - 图标替换
- `frontend/src/components/ui/panel.tsx` - 视觉增强
- `frontend/src/components/layout/TopBar.tsx` - 风格改进
- `frontend/src/components/ui/toast.tsx` - 优化设计
- `frontend/src/components/data/DataViewer.tsx` - 完整实现
- `frontend/src/components/scripting/ScriptPanel.tsx` - 完整实现
- `frontend/src/components/protocols/ProtocolPanel.tsx` - 完整实现
- `frontend/src/components/settings/SettingsPanel.tsx` - 完整实现
- `frontend/src/App.tsx` - 页面切换动画
