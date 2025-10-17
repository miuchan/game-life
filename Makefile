# Game Boy模拟器 Makefile
# 重构后的简化编译和运行过程

.PHONY: all build run clean check test release help rom games demos

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

# 运行新的生命游戏
new-life: release
	@echo "🧬 运行新的生命游戏..."
	cargo run --release --bin new-life-game
	@echo "✅ 新的生命游戏完成！"

# 运行井字棋游戏
tic-tac-toe: release
	@echo "❌ 运行井字棋游戏..."
	cargo run --release --bin tic-tac-toe
	@echo "✅ 井字棋游戏完成！"

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

# 运行所有游戏
games: release
	@echo "🎮 运行所有游戏..."
	@echo "🍭 运行甜甜的生命游戏..."
	cargo run --release --bin sweet-life-game
	@echo "🍭 运行甜甜的生命游戏（凸优化版）..."
	cargo run --release --bin sweet-life-optimized
	@echo "🧬 运行新的生命游戏..."
	cargo run --release --bin new-life-game
	@echo "❌ 运行井字棋游戏..."
	cargo run --release --bin tic-tac-toe
	@echo "✅ 所有游戏运行完成！"

# 运行所有演示
demos: release
	@echo "🎮 运行所有演示程序..."
	@echo "🔐 运行抗量子算法演示..."
	cargo run --release --bin quantum-resistant-demo
	@echo "🏓 运行乒乓自动机演示..."
	cargo run --release --bin ping-pong-automaton
	@echo "🌌 运行时空纠缠演示..."
	cargo run --release --bin spacetime-entanglement
	@echo "🎮 运行任天堂不动点演示..."
	cargo run --release --bin nintendo-fixed-point
	@echo "🎮 运行GBA演示..."
	cargo run --release --bin gba-demo
	@echo "🔬 运行熵源演示..."
	cargo run --release --bin entropy-demo
	@echo "✅ 所有演示运行完成！"

# 显示帮助信息
help:
	@echo "Game Boy模拟器 - 重构后的可用命令："
	@echo ""
	@echo "📦 构建命令："
	@echo "  build        - 构建项目"
	@echo "  release      - 构建发布版本"
	@echo "  check        - 检查代码"
	@echo "  clean        - 清理构建文件"
	@echo ""
	@echo "🚀 运行命令："
	@echo "  run          - 构建并运行主模拟器"
	@echo "  run-fast     - 快速运行（不重新构建）"
	@echo "  run-release  - 运行发布版本"
	@echo ""
	@echo "🎮 游戏命令："
	@echo "  games        - 运行所有游戏"
	@echo "  sweet-life   - 运行甜甜的生命游戏"
	@echo "  sweet-life-opt - 运行甜甜的生命游戏（凸优化版）"
	@echo "  new-life     - 运行新的生命游戏"
	@echo "  tic-tac-toe  - 运行井字棋游戏"
	@echo ""
	@echo "🎯 演示命令："
	@echo "  demos        - 运行所有演示"
	@echo "  quantum-demo - 运行抗量子算法演示"
	@echo "  ping-pong    - 运行乒乓自动机演示"
	@echo "  spacetime    - 运行时空纠缠演示"
	@echo "  nintendo     - 运行任天堂不动点演示"
	@echo "  gba          - 运行GBA演示"
	@echo ""
	@echo "🛠️ 工具命令："
	@echo "  rom          - 生成Game Boy ROM文件"
	@echo "  rom-debug    - 生成ROM文件（调试版本）"
	@echo "  fmt          - 格式化代码"
	@echo "  lint         - 代码检查"
	@echo "  test         - 运行测试"
	@echo "  help         - 显示此帮助信息"
	@echo ""
	@echo "📁 项目结构："
	@echo "  src/core/    - 核心模拟器模块"
	@echo "  src/games/   - 游戏实现"
	@echo "  src/lib/     - 通用库模块"
	@echo "  docs/        - 项目文档"
	@echo "  scripts/     - 构建和演示脚本"
	@echo "  assets/      - 资源文件"
	@echo ""
	@echo "示例："
	@echo "  make games   - 运行所有游戏"
	@echo "  make demos   - 运行所有演示"
	@echo "  make release - 构建优化版本"
	@echo "  make clean   - 清理所有构建文件"
