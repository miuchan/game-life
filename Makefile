# Game Boy模拟器 Makefile
# 简化编译和运行过程

.PHONY: all build run clean check test release help rom

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
	rm -f *.gb *.rom
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

# 生成ROM文件
rom: release
	@echo "🎮 生成Game Boy ROM文件..."
	cargo run --release --bin rom-generator
	@echo "✅ ROM文件生成完成！"

# 运行高级演示
advanced-demo: release
	@echo "🎮 运行高级演示程序..."
	cargo run --release --bin advanced-demo
	@echo "✅ 高级演示完成！"

# 运行甜甜的生命游戏
sweet-life: release
	@echo "🍭 运行甜甜的生命游戏..."
	cargo run --release --bin sweet-life-game
	@echo "✅ 甜甜的生命游戏完成！"

# 运行甜甜的生命游戏（凸优化版）
sweet-life-opt: release
	@echo "🍭 运行甜甜的生命游戏（凸优化版）..."
	cargo run --release --bin sweet-life-optimized
	@echo "✅ 甜甜的生命游戏（凸优化版）完成！"

# 运行抗量子算法演示
quantum-demo: release
	@echo "🔐 运行抗量子算法可视化演示..."
	cargo run --release --bin quantum-resistant-demo
	@echo "✅ 抗量子算法演示完成！"

# 运行乒乓自动机演示
ping-pong: release
	@echo "🏓 运行乒乓自动机可视化演示..."
	cargo run --release --bin ping-pong-automaton
	@echo "✅ 乒乓自动机演示完成！"

# 运行时空纠缠演示
spacetime: release
	@echo "🌌 运行时空纠缠可视化演示..."
	cargo run --release --bin spacetime-entanglement
	@echo "✅ 时空纠缠演示完成！"

# 运行任天堂不动点演示
nintendo: release
	@echo "🎮 运行任天堂不动点算法演示..."
	cargo run --release --bin nintendo-fixed-point
	@echo "✅ 任天堂不动点演示完成！"

# 运行GBA演示
gba: release
	@echo "🎮 运行GBA模拟器演示..."
	cargo run --release --bin gba-demo
	@echo "✅ GBA演示完成！"

# 生成ROM文件（调试版本）
rom-debug: build
	@echo "🎮 生成Game Boy ROM文件（调试版本）..."
	cargo run --bin rom-generator
	@echo "✅ ROM文件生成完成！"

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
	@echo "  rom          - 生成Game Boy ROM文件"
	@echo "  rom-debug    - 生成ROM文件（调试版本）"
	@echo "  fmt          - 格式化代码"
	@echo "  lint         - 代码检查"
	@echo "  help         - 显示此帮助信息"
	@echo ""
	@echo "示例："
	@echo "  make run     - 构建并运行模拟器"
	@echo "  make rom     - 生成可在Game Boy模拟器中运行的ROM文件"
	@echo "  make clean   - 清理所有构建文件"
	@echo "  make release - 构建优化版本"
