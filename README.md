# MWXDump - 微信数据导出工具

## 项目概述

MWXDump 是一个用于微信数据导出的 Rust 工具集，支持微信聊天记录、文件等数据的安全导出和处理。

## 功能特性

- 🔐 **密钥提取**: 自动提取微信加密密钥
- 📊 **数据解密**: 安全解密微信数据库文件
- 🚀 **高性能**: 基于 Rust 的高性能实现
- 🛡️ **安全可靠**: 内存安全和类型安全保证
- 📝 **日志系统**: 完整的日志记录和监控

## 快速开始

### 构建项目

```bash
# 克隆项目
git clone <repository-url>
cd mwxdump

# 构建所有组件
cargo build --release

# 运行 CLI 工具
cargo run --package mwxdump-cli -- key
```

### 基本使用

```bash
# 提取微信密钥
mwxdump key

# 查看帮助
mwxdump --help
```

## 项目结构

```
mwxdump/
├── core/           # 核心库 - 共享功能和算法
├── cli/            # 命令行工具
├── ui/             # 图形界面（规划中）
├── docs/           # 项目文档
└── tests/          # 集成测试
```

## 最新更新

### v0.1.0 - 日志系统重构 (2025-01-20)

- ✅ **性能优化**: 实现高效的时间格式化缓存机制
- ✅ **架构升级**: 建立模块化的日志系统架构  
- ✅ **向后兼容**: 保持原有使用方式和输出格式
- ✅ **错误处理**: 完善的错误处理和降级机制

详细信息请查看 [日志系统重构报告](docs/progress/logging-system-refactor-report.md)

## 当前状态

- **开发阶段**: Alpha
- **核心功能**: 基本完成
- **测试状态**: 功能验证通过
- **文档状态**: 持续完善中

## 下一步计划

1. 完善数据解密功能
2. 添加更多微信版本支持
3. 开发图形用户界面
4. 性能优化和稳定性改进

## 技术栈

- **语言**: Rust 2021 Edition
- **异步运行时**: Tokio
- **日志系统**: tracing + tracing-subscriber
- **CLI 框架**: clap
- **序列化**: serde
- **错误处理**: thiserror + anyhow

## 文档

- [项目文档索引](docs/README.md)
- [架构设计文档](docs/architecture/)
- [进度报告](docs/progress/)

## 许可证

MIT License - 详见 LICENSE 文件

## 贡献

欢迎提交 Issue 和 Pull Request！

---

**注意**: 本工具仅用于个人数据备份和学习目的，请遵守相关法律法规。