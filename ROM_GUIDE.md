# Game Boy ROM生成器使用说明

## 🎮 概述

这个工具可以将你的Game Boy模拟器程序转换为真正的Game Boy ROM文件（.gb格式），可以在任何Game Boy模拟器上运行。

## 🚀 快速开始

### 生成ROM文件
```bash
# 使用Makefile（推荐）
make rom

# 或直接使用cargo
cargo run --release --bin rom-generator
```

### 生成的文件
- **文件名**: `life_game.gb`
- **大小**: 32KB
- **格式**: Game Boy ROM
- **标题**: "LIFE GAME"

## 🎯 支持的模拟器

生成的ROM文件可以在以下模拟器上运行：

### Windows
- **VisualBoyAdvance (VBA)** - 最流行的Game Boy模拟器
- **mGBA** - 现代、高兼容性模拟器
- **BGB** - 高精度模拟器

### macOS
- **mGBA** - 跨平台模拟器
- **OpenEmu** - 多平台模拟器集合

### Linux
- **mGBA** - 跨平台模拟器
- **VisualBoyAdvance-M** - VBA的现代版本

### 在线模拟器
- **JSGB** - 基于JavaScript的在线模拟器
- **Game Boy Online** - 浏览器中的Game Boy模拟器

## 📁 ROM文件结构

生成的ROM文件包含：

### 头部信息 (0x100-0x14F)
- **Nintendo Logo**: 必需的Nintendo标志
- **游戏标题**: "LIFE GAME"
- **卡带类型**: ROM Only
- **ROM大小**: 32KB
- **校验和**: 自动计算的头部和全局校验和

### 程序代码 (0x150+)
- **生命游戏逻辑**: 简化的生命游戏实现
- **主循环**: 游戏主循环
- **规则应用**: 生命游戏的生存/死亡规则

## 🔧 自定义ROM

### 修改游戏标题
编辑 `src/bin/rom_generator.rs` 文件：
```rust
let mut rom_generator = RomGenerator::new("YOUR GAME NAME");
```

### 修改程序代码
编辑 `create_life_game_program()` 函数，添加你自己的Game Boy汇编代码。

### 添加更多功能
- 图形显示
- 输入处理
- 音效
- 更复杂的游戏逻辑

## 🧪 测试ROM

### 1. 在模拟器中测试
1. 下载并安装Game Boy模拟器
2. 打开模拟器
3. 加载 `life_game.gb` 文件
4. 运行并测试功能

### 2. 验证ROM格式
```bash
# 检查文件类型
file life_game.gb

# 应该显示：
# life_game.gb: Game Boy ROM image: "LIFE GAME" (Rev.00) [ROM ONLY], ROM: 256Kbit
```

## 📊 ROM文件信息

```
文件: life_game.gb
大小: 32,768 字节 (32KB)
格式: Game Boy ROM
标题: LIFE GAME
版本: 0.00
类型: ROM Only
目标: 日本市场
```

## 🎮 运行效果

ROM文件包含一个简单的生命游戏实现：
- 初始化游戏状态
- 计算邻居数量
- 应用生命游戏规则
- 循环执行

## 🔍 故障排除

### ROM无法运行
1. 检查ROM文件大小是否为32KB
2. 验证文件格式是否正确
3. 尝试不同的模拟器

### 编译错误
1. 确保Rust工具链已安装
2. 运行 `cargo check` 检查代码
3. 查看错误信息并修复

### 模拟器兼容性
1. 尝试不同的模拟器
2. 检查模拟器版本
3. 查看模拟器日志

## 📚 技术细节

### ROM格式规范
- 遵循Nintendo Game Boy ROM格式
- 包含完整的头部信息
- 正确的校验和计算
- 32KB标准大小

### 程序结构
- 从0x150开始执行
- 包含跳转指令
- 简单的循环结构
- 基本的算术运算

## 🚀 下一步

1. **增强游戏逻辑**: 添加更复杂的生命游戏规则
2. **图形显示**: 实现像素级图形渲染
3. **用户交互**: 添加按键输入处理
4. **音效**: 添加简单的音效支持
5. **存档功能**: 实现游戏状态保存

---

**现在你可以在任何Game Boy模拟器上运行你的生命游戏了！** 🎮✨
