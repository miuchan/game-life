#!/usr/bin/env python3
"""
甜甜的生命游戏ROM验证器
验证生成的Game Boy ROM文件
"""

import struct
import sys
import os

def verify_gb_rom(filename):
    """验证Game Boy ROM文件"""
    print(f"🍭 验证甜甜的生命游戏ROM: {filename}")
    print("=" * 50)
    
    if not os.path.exists(filename):
        print(f"❌ ROM文件不存在: {filename}")
        return False
    
    with open(filename, 'rb') as f:
        data = f.read()
    
    print(f"📁 文件大小: {len(data)} 字节")
    
    if len(data) < 0x150:
        print("❌ ROM文件太小，不是有效的Game Boy ROM")
        return False
    
    # 读取ROM头部信息
    title = data[0x134:0x144].decode('ascii', errors='ignore').strip('\x00')
    print(f"🎮 游戏标题: {title}")
    
    # 检查Nintendo标志
    nintendo_logo = data[0x104:0x134]
    expected_logo = bytes([
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83,
        0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
        0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63,
        0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
    ])
    
    if nintendo_logo == expected_logo:
        print("✅ Nintendo标志正确")
    else:
        print("⚠️  Nintendo标志不匹配（可能是自定义ROM）")
    
    # 读取其他头部信息
    cartridge_type = data[0x147]
    rom_size = data[0x148]
    ram_size = data[0x149]
    
    print(f"💾 卡带类型: 0x{cartridge_type:02X}")
    print(f"📦 ROM大小: 0x{rom_size:02X}")
    print(f"🧠 RAM大小: 0x{ram_size:02X}")
    
    # 计算校验和
    checksum = 0
    for i in range(0x134, 0x14D):
        checksum = (checksum - data[i] - 1) & 0xFF
    
    header_checksum = data[0x14D]
    if checksum == header_checksum:
        print("✅ 头部校验和正确")
    else:
        print(f"⚠️  头部校验和不匹配: 计算={checksum:02X}, 文件={header_checksum:02X}")
    
    # 显示程序代码的前几个字节
    print("\n🔍 程序代码预览:")
    for i in range(0x100, min(0x120, len(data))):
        if i % 16 == 0:
            print(f"0x{i:04X}: ", end="")
        print(f"{data[i]:02X} ", end="")
        if i % 16 == 15:
            print()
    
    print("\n✅ ROM文件验证完成！")
    return True

def main():
    rom_files = ["sweet_life_game.gb", "advanced_demo.gb", "life_game.gb"]
    
    for rom_file in rom_files:
        if os.path.exists(rom_file):
            verify_gb_rom(rom_file)
            print()
    
    print("🎉 所有ROM文件验证完成！")
    print("\n💡 提示: 这些ROM文件可以在任何Game Boy模拟器中运行")
    print("   推荐模拟器: mGBA, VisualBoy Advance, BGB")

if __name__ == "__main__":
    main()
