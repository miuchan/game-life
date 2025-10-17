#!/bin/bash

# Windows俄罗斯方块游戏演示脚本

echo "🎮 Windows俄罗斯方块游戏演示"
echo "=================================================="
echo ""

# 检查可执行文件是否存在
if [ ! -f "./target/release/windows-tetris" ]; then
    echo "❌ Windows俄罗斯方块可执行文件不存在，正在构建..."
    cargo build --bin windows-tetris --release
    if [ $? -ne 0 ]; then
        echo "❌ 构建失败"
        exit 1
    fi
    echo "✅ 构建完成"
fi

echo "📊 系统信息:"
echo "  可执行文件大小: $(ls -lh ./target/release/windows-tetris | awk '{print $5}')"
echo "  构建时间: $(stat -c %y ./target/release/windows-tetris)"
echo ""

echo "🎯 游戏特色:"
echo "  ✅ 完整的俄罗斯方块游戏逻辑"
echo "  ✅ 基于GBA模拟器底层支持"
echo "  ✅ 实时性能统计"
echo "  ✅ 幽灵方块预览"
echo "  ✅ 完整的UI界面"
echo "  ✅ Windows控制台界面"
echo ""

echo "🎮 控制说明:"
echo "  A/D - 左右移动"
echo "  S - 快速下降"
echo "  W - 旋转方块"
echo "  空格 - 硬降落"
echo "  P - 暂停游戏"
echo "  R - 重新开始"
echo "  Q - 退出游戏"
echo ""

echo "🚀 启动游戏..."
echo "注意: 游戏将在控制台中运行"
echo ""

# 运行游戏
./target/release/windows-tetris

echo ""
echo "✅ 游戏演示完成！"
echo "感谢游玩Windows俄罗斯方块！"
