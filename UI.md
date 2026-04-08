# Serial CLI GUI - 前端设计文档

## 概述

Serial CLI GUI 采用 **Tauri 2.x + SolidJS** 技术栈，实现了高性能、跨平台的图形化串口通信工具。设计理念融合了**赛博朋克与工业控制**美学，创造出独特且功能强大的用户界面。

---

## 技术架构

### 技术栈选择

#### 后端：Tauri 2.x + Rust
- **Tauri 2.x**: 轻量级跨平台框架（~10MB vs Electron ~150MB）
- **Rust**: 直接复用现有的 6,700+ 行串口通信代码
- **优势**:
  - 极小的安装包体积
  - 原生性能和安全性
  - 完美的跨平台支持
  - 与现有 CLI 代码无缝集成

#### 前端：SolidJS + TypeScript
- **SolidJS**: 细粒度响应式系统，专为高频数据更新优化
- **TypeScript**: 类型安全，提升代码可维护性
- **Vite**: 快速的开发服务器和构建工具
- **优势**:
  - 出色的运行时性能（虚拟 DOM 细粒度更新）
  - 小巧的包体积（~10KB vs React ~45KB）
  - 简洁的编程模型（内置 signals，无需复杂状态管理）

### 项目结构

```
src-ui/
├── src/
│   ├── main.tsx              # 应用入口
│   ├── App.tsx               # 根组件
│   ├── components/           # UI 组件
│   │   ├── layout/           # 布局组件
│   │   │   ├── Sidebar.tsx   # 侧边栏导航
│   │   │   ├── TopBar.tsx    # 顶部栏
│   │   │   └── Panel.tsx     # 面板容器
│   │   ├── ports/            # 端口管理组件
│   │   ├── data/             # 数据监控组件
│   │   ├── scripting/        # 脚本编辑组件
│   │   └── protocols/        # 协议管理组件
│   ├── hooks/                # 自定义 Hooks
│   ├── stores/               # SolidJS 状态管理
│   └── styles/               # 样式文件
│       └── index.css         # 全局样式和设计系统
├── public/                   # 静态资源
├── package.json              # 依赖配置
├── tsconfig.json             # TypeScript 配置
└── vite.config.ts            # Vite 配置
```

---

## 设计系统

### 视觉概念：赛博工业指挥中心

设计灵感来源于：
- **赛博朋克**: 霓虹灯效、高科技感、未来主义
- **工业控制**: 功能性、清晰的信息层级、可靠性
- **开发者工具**: 高效操作、数据可视化、快捷键支持

### 配色方案

#### 主色调 - 赛博霓虹
```css
--color-cyan: #00F0FF;        /* 主色调 - 青色霓虹 */
--color-magenta: #FF006E;     /* 强调色 - 洋红霓虹 */
--color-purple: #8338EC;      /* 辅助色 - 紫色霓虹 */
```

#### 背景色系 - 深邃太空
```css
--color-dark: #0a0a0f;        /* 主背景 */
--color-darker: #050508;      /* 深色背景 */
--color-bg: #0a0a0f;          /* UI 背景 */
--color-bg-secondary: #12121a; /* 次级背景 */
--color-bg-elevated: #1a1a25;  /* 悬浮背景 */
```

#### 功能色
```css
--color-success: #00ff88;     /* 成功状态 */
--color-warning: #ffaa00;     /* 警告状态 */
--color-error: #ff3366;       /* 错误状态 */
--color-info: #00aaff;        /* 信息提示 */
```

#### 文本色
```css
--color-text: #e0e0e0;        /* 主要文本 */
--color-text-muted: #808090;  /* 次要文本 */
--color-text-dim: #404050;    /* 暗淡文本 */
```

### 字体系统

#### 显示字体 - Orbitron
```css
--font-display: 'Orbitron', sans-serif;
```
- **用途**: 标题、标签、品牌元素
- **特点**: 科技感、未来主义、几何化
- **字重**: 400-900

#### 等宽字体 - JetBrains Mono
```css
--font-mono: 'JetBrains Mono', monospace;
```
- **用途**: 代码、数据、十六进制显示
- **特点**: 高度可读、编程优化、连字支持
- **字重**: 300-700

#### 正文字体 - Inter
```css
--font-body: 'Inter', -apple-system, sans-serif;
```
- **用途**: 界面文本、说明文档
- **特点**: 现代、清晰、多语言支持
- **字重**: 300-700

### 字体大小层级
```css
--text-xs: 0.75rem;      /* 12px - 辅助信息 */
--text-sm: 0.875rem;     /* 14px - 次要文本 */
--text-base: 1rem;       /* 16px - 正文 */
--text-lg: 1.125rem;     /* 18px - 小标题 */
--text-xl: 1.25rem;      /* 20px - 卡片标题 */
--text-2xl: 1.5rem;      /* 24px - 区块标题 */
--text-3xl: 1.875rem;    /* 30px - 页面标题 */
--text-4xl: 2.25rem;     /* 36px - 主标题 */
```

### 间距系统

基于 **4px 栅格**：
```css
--spacing-xs: 0.25rem;   /* 4px */
--spacing-sm: 0.5rem;    /* 8px */
--spacing-md: 1rem;      /* 16px */
--spacing-lg: 1.5rem;    /* 24px */
--spacing-xl: 2rem;      /* 32px */
--spacing-2xl: 3rem;     /* 48px */
--spacing-3xl: 4rem;     /* 64px */
```

### 动画和过渡

#### 过渡时长
```css
--transition-fast: 150ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-base: 250ms cubic-bezier(0.4, 0, 0.2, 1);
--transition-slow: 350ms cubic-bezier(0.4, 0, 0.2, 1);
```

#### 特效
```css
--shadow-glow: 0 0 20px rgba(0, 240, 255, 0.3);
--shadow-glow-strong: 0 0 30px rgba(0, 240, 255, 0.5);
--shadow-glow-magenta: 0 0 20px rgba(255, 0, 110, 0.3);
```

---

## UI 组件库

### 布局组件

#### Sidebar（侧边栏）
**功能**: 主导航、模块切换、状态指示

**特性**:
- 渐变背景动画
- 激活项霓虹灯效果
- 悬停状态反馈
- 底部状态指示器（脉冲动画）

**样式**:
- 固定宽度: 260px
- 玻璃态边框
- 非对称布局
- 图标 + 文本导航

#### TopBar（顶部栏）
**功能**: 品牌标识、连接状态、全局操作

**特性**:
- 渐变顶边动画
- 连接状态徽章
- 快捷操作按钮
- 版本信息显示

**样式**:
- 固定高度: 60px
- 左中右三段布局
- 发光边框效果

#### Panel（面板容器）
**功能**: 内容分组、模块容器

**特性**:
- 可配置发光效果（cyan/magenta/purple）
- 悬停阴影动画
- 标题 + 操作区
- 响应式内容区域

**样式**:
- 圆角: 0.75rem
- 边框: 1px solid
- 悬停效果: 边框高亮 + 发光阴影

### 数据展示组件

#### PortList（端口列表）
**功能**: 显示可用串口、连接状态

**特性**:
- 自动刷新
- 状态指示灯
- 快速连接按钮
- 端口详情展示

#### DataViewer（数据查看器）
**功能**: 实时数据显示、十六进制/ASCII切换

**特性**:
- 虚拟滚动（大数据量）
- 协议帧高亮
- 时间戳显示
- 数据过滤搜索
- 导出功能

#### ScriptEditor（脚本编辑器）
**功能**: Lua 脚本编辑、执行、调试

**特性**:
- Monaco 编辑器集成
- 语法高亮
- 错误提示
- 执行按钮
- 输出面板

---

## 交互模式

### 导航流程
1. **侧边栏导航**: 主功能模块切换
2. **标签页**: 多任务并行处理
3. **面包屑**: 快速返回上级

### 数据操作
1. **点击选择**: 选中端口/数据
2. **双击操作**: 快速打开/编辑
3. **右键菜单**: 上下文操作
4. **拖放**: 数据移动/排序

### 快捷键
- `Ctrl+O`: 打开端口
- `Ctrl+W`: 关闭端口
- `Ctrl+Enter`: 发送数据
- `Ctrl+L`: 清空显示
- `Ctrl+S`: 保存数据
- `F5`: 刷新端口列表

---

## 响应式设计

### 断点系统
```css
/* 移动设备 */
@media (max-width: 768px) {
  /* 单列布局 */
  /* 侧边栏折叠 */
}

/* 平板设备 */
@media (min-width: 769px) and (max-width: 1024px) {
  /* 两列布局 */
  /* 优化间距 */
}

/* 桌面设备 */
@media (min-width: 1025px) {
  /* 三列布局 */
  /* 完整功能 */
}
```

### 适配策略
- **移动端**: 简化界面、折叠菜单、触摸优化
- **平板**: 平衡布局、适中字号、手势支持
- **桌面**: 完整功能、多窗口、键盘快捷键

---

## 性能优化策略

### 渲染优化
1. **细粒度更新**: SolidJS signals 避免不必要的重渲染
2. **虚拟滚动**: 处理大量数据（1000+ 行）
3. **防抖节流**: 用户输入优化
4. **懒加载**: 按需加载组件（Monaco 编辑器）

### 数据优化
1. **批量更新**: 50-100ms 合并数据更新
2. **Web Workers**: 耗时操作后台处理
3. **增量渲染**: 分块显示大数据
4. **内存管理**: 及时清理旧数据

### 网络优化
1. **命令缓存**: 减少重复调用
2. **批量请求**: 合并多个操作
3. **错误重试**: 自动重试机制
4. **离线支持**: 本地数据缓存

---

## 可访问性

### 键盘导航
- Tab 键导航
- 快捷键支持
- 焦点可见性

### 屏幕阅读器
- ARIA 标签
- 语义化 HTML
- 状态通知

### 视觉辅助
- 高对比度模式
- 字体大小调节
- 色盲友好配色

---

## 开发规范

### 命名约定
- **组件**: PascalCase (e.g., `PortList.tsx`)
- **函数**: camelCase (e.g., `handleClick`)
- **常量**: UPPER_SNAKE_CASE (e.g., `MAX_PORTS`)
- **CSS**: kebab-case (e.g., `.sidebar-nav-item`)

### 文件组织
- 每个组件一个文件夹
- 样式与组件同目录
- 测试文件 `*.test.tsx`
- 类型定义 `*.types.ts`

### 代码风格
- 使用 TypeScript 严格模式
- 遵循 ESLint 规则
- 使用 Prettier 格式化
- 编写单元测试

---

## 构建和部署

### 开发环境
```bash
cd src-ui
npm install
npm run dev  # http://localhost:1420
```

### 生产构建
```bash
npm run build  # 生成 dist/ 目录
```

### Tauri 集成
```bash
cargo tauri dev   # 开发模式
cargo tauri build # 生产构建
```

---

## 未来扩展

### 计划功能
- [ ] 暗色/亮色主题切换
- [ ] 自定义配色方案
- [ ] 插件系统
- [ ] 多语言支持
- [ ] 云同步配置
- [ ] 移动端适配

### 性能目标
- 应用启动: < 2秒
- 数据更新: 1000+ packets/秒
- 内存占用: < 200MB
- 包体积: < 15MB

---

**文档版本**: 1.0.0
**最后更新**: 2026-04-05
**维护者**: Serial CLI Team
