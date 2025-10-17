# Game Boy 模拟器

一个用Rust编写的Game Boy模拟器，支持完整的CPU指令集和内存管理。

## 🚀 快速开始

### 方法1：使用Makefile（推荐）
```bash
# 构建并运行
make run

# 快速运行（不重新构建）
make run-fast

# 构建发布版本
make release

# 查看所有可用命令
make help
```

### 方法2：使用构建脚本
```bash
# 构建并运行
./build.sh run

# 完整构建流程（检查+测试+构建）
./build.sh all

# 构建发布版本
./build.sh release

# 查看帮助
./build.sh help
```

### 方法3：直接使用Cargo
```bash
# 构建并运行
cargo run

# 快速运行（不重新构建）
cargo run --quiet

# 构建发布版本
cargo build --release
cargo run --release

# 代码检查
cargo check

# 运行测试
cargo test
```

### 方法4：使用快速启动脚本
```bash
# 一键运行
./quick-run.sh
```

## 📋 可用命令

### Makefile命令
- `make build` - 构建项目
- `make run` - 构建并运行
- `make run-fast` - 快速运行（不重新构建）
- `make check` - 检查代码
- `make test` - 运行测试
- `make clean` - 清理构建文件
- `make release` - 构建发布版本
- `make run-release` - 运行发布版本
- `make fmt` - 格式化代码
- `make lint` - 代码检查
- `make help` - 显示帮助信息

### 构建脚本命令
- `./build.sh build` - 构建项目
- `./build.sh run` - 构建并运行
- `./build.sh test` - 运行测试
- `./build.sh check` - 代码检查
- `./build.sh format` - 格式化代码
- `./build.sh clean` - 清理构建文件
- `./build.sh release` - 构建发布版本
- `./build.sh run-release` - 运行发布版本
- `./build.sh all` - 完整构建流程
- `./build.sh help` - 显示帮助信息

## 🎮 功能特性

### CPU功能
- ✅ 8个8位寄存器（A, B, C, D, E, F, H, L）
- ✅ 16位寄存器对（BC, DE, HL, SP）
- ✅ 程序计数器（PC）
- ✅ 栈指针（SP）
- ✅ 标志寄存器（Z, N, H, C）

### 指令集
- ✅ 算术指令：ADD, SUB, INC, DEC
- ✅ 数据传输指令：LD（8位和16位）
- ✅ 内存操作指令：LD (BC), A, LD A, (BC) 等
- ✅ 控制流指令：JP（绝对跳转）, JR（相对跳转）
- ✅ 16位操作指令：INC16, DEC16, LD16

### 内存系统
- ✅ 64KB内存空间（0x0000-0xFFFF）
- ✅ 内存读写操作
- ✅ 内存总线接口

## 🔧 开发

### 代码质量
```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test
```

### 性能优化
```bash
# 构建优化版本
cargo build --release

# 运行优化版本
cargo run --release
```

## 📁 项目结构

```
gameboy-emulator/
├── src/
│   └── main.rs          # 主程序文件
├── Cargo.toml           # 项目配置
├── Makefile            # 构建脚本
├── build.sh            # 高级构建脚本
├── quick-run.sh        # 快速运行脚本
└── README.md           # 项目说明
```

## 🎯 使用建议

1. **日常开发**：使用 `make run` 或 `cargo run`
2. **性能测试**：使用 `make release` 构建优化版本
3. **代码检查**：使用 `make lint` 或 `cargo clippy`
4. **快速测试**：使用 `./quick-run.sh`

## 🐛 故障排除

### 编译错误
```bash
# 清理并重新构建
make clean
make build
```

### 运行时错误
```bash
# 检查代码
cargo check

# 运行测试
cargo test
```

## 📄 许可证

MIT License - 详见 LICENSE 文件

## 🤝 贡献

欢迎提交Issue和Pull Request！

---

**享受你的Game Boy模拟器开发之旅！** 🎮✨
