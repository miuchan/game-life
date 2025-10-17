# Game Boy Emulator - 项目结构

## 📁 目录结构

```
gameboy-emulator/
├── 📁 assets/                    # 资源文件
│   ├── 📁 roms/                 # ROM文件
│   ├── 📁 saves/                # 存档文件
│   └── 📁 configs/              # 配置文件
├── 📁 docs/                     # 文档
│   ├── 📁 architecture/         # 架构文档
│   ├── 📁 guides/               # 使用指南
│   └── 📁 api/                  # API文档
├── 📁 scripts/                  # 脚本文件
│   ├── 📁 build/                # 构建脚本
│   ├── 📁 demo/                 # 演示脚本
│   └── 📁 test/                 # 测试脚本
├── 📁 src/                      # 源代码
│   ├── 📁 core/                 # 核心模块
│   ├── 📁 games/                # 游戏模块
│   ├── 📁 entropy/              # 熵源系统
│   ├── 📁 bin/                  # 可执行文件
│   └── 📁 lib/                  # 库文件
├── 📁 target/                   # 构建输出
├── 📄 Cargo.toml                # 项目配置
├── 📄 Cargo.lock                # 依赖锁定
├── 📄 Makefile                  # 构建配置
└── 📄 README.md                 # 项目说明
```

## 🎯 设计原则

### 1. 模块化设计
- **核心模块**: CPU、内存、GPU等基础组件
- **游戏模块**: 各种游戏实现
- **熵源系统**: 随机数生成和优化
- **工具模块**: 调试、测试、演示工具

### 2. 清晰的依赖关系
- 核心模块不依赖游戏模块
- 游戏模块依赖核心模块
- 熵源系统独立可复用
- 工具模块依赖所有其他模块

### 3. 统一的接口设计
- 所有模块都实现标准接口
- 错误处理统一化
- 配置管理集中化
- 日志系统标准化

## 🚀 快速开始

```bash
# 构建项目
make build

# 运行演示
make demo

# 运行测试
make test

# 生成文档
make docs
```

## 📚 文档导航

- [架构设计](docs/architecture/)
- [使用指南](docs/guides/)
- [API参考](docs/api/)
- [项目总结](docs/PROJECT_SUMMARY.md)
