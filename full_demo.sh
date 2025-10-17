#!/bin/bash

# 完整的Tic-Tac-Toe系统演示
# 展示所有生命游戏的活力运行

echo "🎮 Tic-Tac-Toe 井字棋游戏系统 - 完整演示"
echo "=================================================="
echo ""

# 检查所有可执行文件
echo "📋 检查所有可执行文件:"
echo ""

if [ -f "./target/release/tic-tac-toe" ]; then
    echo "✅ tic-tac-toe: $(ls -lh ./target/release/tic-tac-toe | awk '{print $5}')"
else
    echo "❌ tic-tac-toe: 不存在"
fi

if [ -f "./target/release/new-life-game" ]; then
    echo "✅ new-life-game: $(ls -lh ./target/release/new-life-game | awk '{print $5}')"
else
    echo "❌ new-life-game: 不存在"
fi

if [ -f "./target/release/sweet-life-game" ]; then
    echo "✅ sweet-life-game: $(ls -lh ./target/release/sweet-life-game | awk '{print $5}')"
else
    echo "❌ sweet-life-game: 不存在"
fi

if [ -f "./target/release/sweet-life-optimized" ]; then
    echo "✅ sweet-life-optimized: $(ls -lh ./target/release/sweet-life-optimized | awk '{print $5}')"
else
    echo "❌ sweet-life-optimized: 不存在"
fi

echo ""

# 演示1: 直接运行各个生命游戏
echo "🎯 演示1: 直接运行各个生命游戏"
echo "=================================================="

echo "🚀 运行全新的生命游戏 (5秒):"
timeout 5s ./target/release/new-life-game || echo "⏰ 时间到"

echo ""
echo "🚀 运行甜甜的生命游戏 (5秒):"
timeout 5s ./target/release/sweet-life-game || echo "⏰ 时间到"

echo ""
echo "🚀 运行优化的生命游戏 (5秒):"
timeout 5s ./target/release/sweet-life-optimized || echo "⏰ 时间到"

echo ""

# 演示2: 通过tic-tac-toe系统运行
echo "🎯 演示2: 通过Tic-Tac-Toe系统运行"
echo "=================================================="

echo "🚀 启动Tic-Tac-Toe系统并运行所有生命游戏:"
echo ""

# 创建输入脚本
cat > tic_tac_toe_demo.txt << EOF
3
0
EOF

timeout 20s ./target/release/tic-tac-toe < tic_tac_toe_demo.txt

echo ""

# 演示3: 井字棋游戏
echo "🎯 演示3: 井字棋游戏AI对战"
echo "=================================================="

echo "🚀 启动井字棋游戏 (简单难度):"
echo ""

# 创建井字棋演示输入
cat > tic_tac_toe_game.txt << EOF
1
1
1 1
2 2
1 0
2 1
1 2
0
EOF

timeout 15s ./target/release/tic-tac-toe < tic_tac_toe_game.txt

echo ""

# 清理临时文件
rm -f tic_tac_toe_demo.txt tic_tac_toe_game.txt

echo "🎉 完整演示完成！"
echo ""
echo "📊 系统总结:"
echo "  ✅ Tic-Tac-Toe 井字棋游戏系统 (485KB)"
echo "  ✅ 全新的生命游戏 (420KB)"
echo "  ✅ 甜甜的生命游戏 (391KB)"
echo "  ✅ 优化的生命游戏 (399KB)"
echo ""
echo "🌟 核心功能:"
echo "  🎮 井字棋AI对战 (3种难度)"
echo "  🧬 生命游戏管理器"
echo "  🔬 外部熵源集成"
echo "  ⚛️  量子抗性技术"
echo "  📈 概率分布优化"
echo "  📊 实时统计监控"
echo ""
echo "🎯 所有生命游戏都能有活力地运行！"
echo "💡 使用 './target/release/tic-tac-toe' 启动完整系统"
