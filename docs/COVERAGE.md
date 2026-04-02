# 测试覆盖率目标

本文档定义了Serial CLI项目的测试覆盖率目标和策略。

## 覆盖率目标

| 模块 | 当前覆盖率 | 目标覆盖率 | 优先级 |
|------|-----------|-----------|--------|
| 核心串口管理 | ~80% | >90% | 高 |
| 协议系统 | ~75% | >85% | 高 |
| Lua API | ~70% | >80% | 中 |
| 任务调度 | ~70% | >80% | 中 |
| CLI接口 | ~60% | >75% | 中 |
| 错误处理 | ~85% | >90% | 低 |
| 配置管理 | ~80% | >85% | 低 |
| **总体** | **~70%** | **>85%** | - |

## 覆盖率类型

### 行覆盖率（Line Coverage）
- **目标**: >85%
- **说明**: 每行代码是否被执行

### 分支覆盖率（Branch Coverage）
- **目标**: >75%
- **说明**: 每个条件分支是否都被测试

### 函数覆盖率（Function Coverage）
- **目标**: >90%
- **说明**: 每个函数是否被调用

## 测试策略

### 单元测试
- 所有公共API必须有单元测试
- 私有函数在复杂情况下需要测试
- 边界条件和错误情况必须测试

### 集成测试
- 主要工作流需要集成测试
- 跨模块功能需要集成测试
- 真实场景模拟需要集成测试

### 特殊测试
- 硬件相关代码需要mock测试
- 错误路径需要专门测试
- 并发场景需要压力测试

## 测试工具

### 本地测试
```bash
# 运行所有测试
cargo test

# 生成覆盖率报告（本地）
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# 查看HTML报告
open target/tarpaulin.html
```

### CI/CD测试
- 每次PR自动运行测试
- 覆盖率报告自动上传到Codecov
- 覆盖率下降会发出警告

## 覆盖率报告

### 查看覆盖率
- **本地**: `cargo tarpaulin --out Html`
- **CI**: GitHub Actions中的Coverage job
- **在线**: Codecov.io（如果配置）

### 覆盖率徽章
在README中添加覆盖率徽章：
```markdown
[![Coverage](https://codecov.io/gh/zazac-zhang/serial_cli/branch/master/graph/badge.svg)](https://codecov.io/gh/zazac-zhang/serial_cli)
```

## 提高覆盖率的建议

### 优先级1：关键路径
- 串口打开/关闭流程
- 数据读写操作
- 协议编解码
- 错误处理

### 优先级2：边界情况
- 无效输入
- 资源耗尽
- 超时处理
- 并发冲突

### 优先级3：工具函数
- 数据转换
- 字符串处理
- 配置解析

## 覆盖率检查清单

在提交PR前，请确认：

- [ ] 新代码有对应的测试
- [ ] 覆盖率未下降
- [ ] 关键路径都有测试
- [ ] 错误情况都有测试
- [ ] 本地覆盖率报告已检查

## 忽略的代码

以下代码可以不计入覆盖率：

- 测试代码本身
- 构建脚本
- 示例代码
- 死代码（应该移除）
- 平台特定代码（在对应平台测试）

## 参考资源

- [Rust测试指南](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tarpaulin文档](https://github.com/xd009642/tarpaulin)
- [Codecov文档](https://docs.codecov.com/)
