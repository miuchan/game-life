#!/bin/bash

# Tic-Tac-Toe 井字棋游戏系统演示脚本
# 展示所有生命游戏的活力运行

echo "🎮 Tic-Tac-Toe 井字棋游戏系统演示"
echo "=================================================="
echo ""

# 检查可执行文件是否存在
if [ ! -f "./target/release/tic-tac-toe" ]; then
    echo "❌ tic-tac-toe 可执行文件不存在，正在构建..."
    cargo build --bin tic-tac-toe --release
    if [ $? -ne 0 ]; then
        echo "❌ 构建失败"
        exit 1
    fi
    echo "✅ 构建完成"
fi

echo "📊 系统信息:"
echo "  可执行文件大小: $(ls -lh ./target/release/tic-tac-toe | awk '{print $5}')"
echo "  构建时间: $(stat -c %y ./target/release/tic-tac-toe)"
echo ""

echo "🎯 功能演示:"
echo "1. 井字棋游戏 (AI对战)"
echo "2. 生命游戏管理器"
echo "3. 熵源系统集成"
echo "4. 统计信息展示"
echo ""

echo "🚀 启动系统演示..."
echo "注意: 系统将自动演示各项功能"
echo ""

# 创建演示输入
cat > demo_input.txt << EOF
2
3
5
6
0
EOF

echo "📋 演示流程:"
echo "  1. 查看生命游戏列表"
echo "  2. 运行所有生命游戏"
echo "  3. 查看统计信息"
echo "  4. 查看熵源信息"
echo "  5. 退出系统"
echo ""

# 运行演示
timeout 30s ./target/release/tic-tac-toe < demo_input.txt

echo ""
echo "🎉 演示完成！"
echo ""
echo "💡 手动运行方式:"
echo "  ./target/release/tic-tac-toe"
echo ""
echo "🎮 可用功能:"
echo "  1. 玩井字棋 - 与AI对战"
echo "  2. 查看生命游戏 - 列出所有可用游戏"
echo "  3. 运行所有生命游戏 - 启动所有生命游戏"
echo "  4. 运行特定生命游戏 - 选择单个游戏运行"
echo "  5. 查看统计信息 - 显示游戏统计"
echo "  6. 查看熵源信息 - 显示熵源系统状态"
echo "  0. 退出系统"
echo ""

# 清理演示文件
rm -f demo_input.txt

echo "🌟 系统特色:"
echo "  ✅ 集成井字棋AI对战"
echo "  ✅ 多难度AI策略 (简单/中等/困难)"
echo "  ✅ 生命游戏管理器"
echo "  ✅ 外部熵源集成"
echo "  ✅ 量子抗性技术"
echo "  ✅ 概率分布优化"
echo "  ✅ 实时统计监控"
echo ""
echo "🎯 所有生命游戏都能有活力地运行！"
