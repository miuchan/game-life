//! GBA演示程序
//! 
//! 这个程序展示了GBA模拟器的功能，
//! 包括CPU执行、图形渲染等

use gameboy_emulator::gba::{GBASystem, GBAState};
use std::time::Instant;

fn main() -> Result<(), String> {
    println!("🎮 GBA模拟器演示");
    println!("=====================================");
    println!("🎯 展示Game Boy Advance模拟器功能");

    // 创建GBA模拟器实例
    let mut gba = GBASystem::new();
    
    // 创建简单的GBA ROM数据
    let rom_data = create_gba_demo_rom();
    
    // 加载ROM
    gba.load_rom(rom_data)?;
    
    // 启动模拟器
    gba.start()?;
    
    println!("🚀 开始GBA模拟器演示...");
    println!();
    
    // 显示初始状态
    println!("{}", gba.get_debug_info());
    println!();
    
    // 性能计时
    let start_time = Instant::now();
    let mut step_count = 0;
    
    // 运行演示循环
    for frame in 0..100 {
        match gba.run_frame() {
            Ok(()) => {
                step_count += 1;
                
                if frame % 10 == 0 {
                    println!("🎮 第{}帧: PC=0x{:08X}", 
                        frame + 1, 
                        gba.get_cpu_state().get_pc()
                    );
                    
                    let cpu_state = gba.get_cpu_state();
                    let stats = gba.get_stats();
                    
                    println!("   💻 CPU状态: Thumb={}, 周期={}, 指令={}", 
                        cpu_state.thumb_mode,
                        cpu_state.get_stats().cycles,
                        cpu_state.get_stats().instructions
                    );
                    
                    println!("   🎨 GPU状态: 帧数={}, 像素={}, 扫描线={}", 
                        gba.get_gpu_state().frame_count,
                        gba.get_gpu_state().get_stats().pixels_drawn,
                        gba.get_gpu_state().current_scanline
                    );
                    
                    println!("   📊 性能统计: FPS={:.2}, CPU使用率={:.2}%", 
                        stats.fps,
                        stats.cpu_usage * 100.0
                    );
                    
                    if frame % 30 == 0 {
                        println!("{}", gba.get_debug_info());
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("❌ GBA模拟器错误: {}", e);
                break;
            }
        }
    }
    
    // 计算性能统计
    let elapsed_time = start_time.elapsed();
    
    // 显示最终统计
    println!();
    println!("🎉 GBA模拟器演示完成！");
    println!("📊 演示统计:");
    println!("   ⏱️  总执行时间: {:.2}ms", elapsed_time.as_millis());
    println!("   🔄 总周期数: {}", gba.get_cpu_state().get_stats().cycles);
    println!("   📝 总指令数: {}", gba.get_cpu_state().get_stats().instructions);
    println!("   🎨 总帧数: {}", gba.get_gpu_state().get_stats().frames_rendered);
    println!("   🖼️  总像素数: {}", gba.get_gpu_state().get_stats().pixels_drawn);
    println!("   ⚡ 平均FPS: {:.2}", gba.get_stats().fps);
    println!("   💻 CPU使用率: {:.2}%", gba.get_stats().cpu_usage * 100.0);
    println!("   🎮 演示步数: {}", step_count);
    
    // 显示GBA分析
    println!();
    println!("🎮 GBA分析:");
    let cpu_stats = gba.get_cpu_state().get_stats();
    let gpu_stats = gba.get_gpu_state().get_stats();
    let memory_stats = gba.get_memory_state().get_stats();
    
    println!("   🔍 CPU类型: ARM7TDMI");
    println!("   📱 显示模式: {:?}", gba.get_gpu_state().get_display_mode());
    println!("   🎨 背景层: {}/4", 
        (0..4).filter(|&i| gba.get_gpu_state().is_background_enabled(i)).count()
    );
    println!("   🖼️  精灵数量: 128");
    println!("   🎨 调色板: 512颜色");
    println!("   💾 内存读取: {}", memory_stats.reads);
    println!("   💾 内存写入: {}", memory_stats.writes);
    println!("   🔄 ARM指令: {}", cpu_stats.arm_instructions);
    println!("   🔄 Thumb指令: {}", cpu_stats.thumb_instructions);
    println!("   🎯 最终PC: 0x{:08X}", gba.get_cpu_state().get_pc());
    
    println!("{}", gba.get_debug_info());
    
    Ok(())
}

/// 创建GBA演示ROM
fn create_gba_demo_rom() -> Vec<u8> {
    let mut rom = vec![0u8; 0x200]; // 512字节的最小ROM
    
    // Nintendo标志 (0x04-0xA0)
    let nintendo_logo = [
        0x24, 0xFF, 0xAE, 0x51, 0x69, 0x9A, 0xA2, 0x21, 0x3D, 0x84, 0x82, 0x8A, 0x84, 0x24, 0x04, 0x51,
        0x11, 0x40, 0x9C, 0x00, 0x21, 0x13, 0x82, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    
    for (i, &byte) in nintendo_logo.iter().enumerate() {
        rom[0x04 + i] = byte;
    }
    
    // 游戏标题 (0xA0-0xAC)
    let title = b"GBA DEMO ROM";
    for (i, &byte) in title.iter().enumerate() {
        rom[0xA0 + i] = byte;
    }
    
    // 游戏代码 (0xAC-0xB0)
    rom[0xAC] = b'G';
    rom[0xAD] = b'B';
    rom[0xAE] = b'A';
    rom[0xAF] = b'D';
    
    // 制造商代码 (0xB0-0xB2)
    rom[0xB0] = 0x00;
    rom[0xB1] = 0x00;
    
    // 固定值 (0xB2)
    rom[0xB2] = 0x96;
    
    // 主单元代码 (0xB3)
    rom[0xB3] = 0x00;
    
    // 设备类型 (0xB4)
    rom[0xB4] = 0x00;
    
    // 保留区域 (0xB5-0xBC)
    for i in 0xB5..=0xBC {
        rom[i] = 0x00;
    }
    
    // 软件版本 (0xBC)
    rom[0xBC] = 0x00;
    
    // 校验和 (0xBD)
    rom[0xBD] = 0x00;
    
    // 保留区域 (0xBE-0xBF)
    rom[0xBE] = 0x00;
    rom[0xBF] = 0x00;
    
    // 程序代码区域 (0x100开始)
    // 简单的ARM指令序列
    let program = create_gba_program();
    for (i, &byte) in program.iter().enumerate() {
        if 0x100 + i < rom.len() {
            rom[0x100 + i] = byte;
        }
    }
    
    rom
}

/// 创建GBA程序代码
fn create_gba_program() -> Vec<u8> {
    vec![
        // ARM指令序列
        // MOV R0, #0x100
        0x00, 0x00, 0xA0, 0xE3,
        
        // MOV R1, #0x200
        0x00, 0x01, 0xA0, 0xE3,
        
        // ADD R2, R0, R1
        0x01, 0x20, 0x80, 0xE0,
        
        // MOV R3, #0x300
        0x00, 0x03, 0xA0, 0xE3,
        
        // SUB R4, R2, R3
        0x03, 0x40, 0x82, 0xE0,
        
        // MOV R5, #0x400
        0x00, 0x05, 0xA0, 0xE3,
        
        // AND R6, R4, R5
        0x05, 0x60, 0x84, 0xE0,
        
        // ORR R7, R6, R0
        0x00, 0x70, 0x86, 0xE1,
        
        // EOR R8, R7, R1
        0x01, 0x80, 0x87, 0xE0,
        
        // MOV R9, #0x500
        0x00, 0x09, 0xA0, 0xE3,
        
        // ADD R10, R8, R9
        0x09, 0xA0, 0x88, 0xE0,
        
        // MOV R11, #0x600
        0x00, 0x0B, 0xA0, 0xE3,
        
        // SUB R12, R10, R11
        0x0B, 0xC0, 0x8A, 0xE0,
        
        // MOV R13, #0x700
        0x00, 0x0D, 0xA0, 0xE3,
        
        // ADD R14, R12, R13
        0x0D, 0xE0, 0x8C, 0xE0,
        
        // MOV R15, #0x800
        0x00, 0x0F, 0xA0, 0xE3,
        
        // 跳转回开始
        0x00, 0x00, 0x00, 0xEA,
    ]
}
