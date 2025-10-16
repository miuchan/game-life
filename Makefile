# Game Boy模拟器 Makefile
# 简化编译和运行过程

.PHONY: all build run clean check test release help

# 默认目标
all: build

# 构建项目
build:
	@echo "🔨 构建Game Boy模拟器..."
	cargo build
	@echo "✅ 构建完成！"

# 运行模拟器
run: build
	@echo "🚀 运行Game Boy模拟器..."
	cargo run

# 快速运行（不重新构建）
run-fast:
	@echo "⚡ 快速运行Game Boy模拟器..."
	cargo run

# 检查代码（不构建）
check:
	@echo "🔍 检查代码..."
	cargo check

# 运行测试
test:
	@echo "🧪 运行测试..."
	cargo test

# 清理构建文件
clean:
	@echo "🧹 清理构建文件..."
	cargo clean
	@echo "✅ 清理完成！"

# 发布构建（优化版本）
release:
	@echo "🚀 构建发布版本..."
	cargo build --release
	@echo "✅ 发布版本构建完成！"

# 运行发布版本
run-release: release
	@echo "🚀 运行发布版本..."
	cargo run --release

# 格式化代码
fmt:
	@echo "🎨 格式化代码..."
	cargo fmt

# 代码检查（包含clippy）
lint:
	@echo "🔍 代码检查..."
	cargo clippy

# 显示帮助信息
help:
	@echo "Game Boy模拟器 - 可用命令："
	@echo ""
	@echo "  build        - 构建项目"
	@echo "  run          - 构建并运行"
	@echo "  run-fast     - 快速运行（不重新构建）"
	@echo "  check        - 检查代码"
	@echo "  test         - 运行测试"
	@echo "  clean        - 清理构建文件"
	@echo "  release      - 构建发布版本"
	@echo "  run-release  - 运行发布版本"
	@echo "  fmt          - 格式化代码"
	@echo "  lint         - 代码检查"
	@echo "  help         - 显示此帮助信息"
	@echo ""
	@echo "示例："
	@echo "  make run     - 构建并运行模拟器"
	@echo "  make clean   - 清理所有构建文件"
	@echo "  make release - 构建优化版本"
