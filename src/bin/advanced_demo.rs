//! 高级GameBoy模拟器演示程序

use gameboy_emulator::{AdvancedGameBoy, RomGenerator};
use gameboy_emulator::debug::LogLevel;

fn main() -> Result<(), String> {
    println!("🎮 高级GameBoy模拟器演示");
    println!("================================");

    // 创建高级模拟器实例
    let mut gameboy = AdvancedGameBoy::new();
    
    // 设置日志级别
    gameboy.set_log_level(LogLevel::Info);
    
    // 设置目标FPS
    gameboy.set_target_fps(60);
    
    // 定义测试程序
    let program = vec![
        0x00, // NOP
        0x01, 0x34, 0x12, // LD BC, 0x1234
        0x11, 0x78, 0x56, // LD DE, 0x5678
        0x21, 0xBC, 0x9A, // LD HL, 0x9ABC
        0x3E, 0x42, // LD A, 0x42
        0x02, // LD (BC), A
        0x12, // LD (DE), A
        0x77, // LD (HL), A
        0x0A, // LD A, (BC)
        0x1A, // LD A, (DE)
        0x7E, // LD A, (HL)
        0x3C, // INC A
        0x3D, // DEC A
        0x80, // ADD A, B
        0x90, // SUB B
        0x18, 0x05, // JR +5
        0x00, // NOP (跳过)
        0x00, // NOP (跳过)
        0x00, // NOP (跳过)
        0x00, // NOP (跳过)
        0xC3, 0x00, 0x02, // JP 0x200
        0x00, // NOP (跳过)
    ];
    
    // 在0x200处放置更多指令
    let program_at_200 = vec![
        0x3E, 0xFF, // LD A, 0xFF
        0x06, 0x10, // LD B, 0x10
        0x05, // DEC B
        0x20, 0xFD, // JR NZ, -3 (循环)
        0x00, // NOP
    ];
    
    // 加载程序
    gameboy.load_program(0x100, &program)?;
    gameboy.load_program(0x200, &program_at_200)?;
    
    // 设置一些断点
    gameboy.set_breakpoint(0x200, None);
    // 暂时移除0x108的断点，因为它会阻止程序执行
    // gameboy.set_breakpoint(0x108, Some("A == 0x42".to_string()));
    
    println!("🚀 开始执行程序...");
    println!();
    
    // 启动模拟器
    gameboy.start();
    
    // 显示初始状态
    println!("{}", gameboy.get_debug_info());
    println!();
    
    // 执行程序
    for i in 0..20 {
        match gameboy.step_once() {
            Ok(()) => {
                println!("步骤 {}: PC=0x{:04X}, 指令={}", 
                    i + 1, 
                    gameboy.cpu.pc, 
                    gameboy.disassemble_instruction(gameboy.cpu.pc)
                );
                
                // 每5步显示一次详细状态
                if (i + 1) % 5 == 0 {
                    println!("{}", gameboy.get_debug_info());
                    println!();
                }
            }
            Err(e) => {
                println!("❌ 执行错误: {}", e);
                break;
            }
        }
    }
    
    // 显示性能统计
    let stats = gameboy.get_performance_stats();
    println!("📊 性能统计:");
    println!("  周期数: {}", stats.cycle_count);
    println!("  指令数: {}", stats.instruction_count);
    println!("  平均每指令周期数: {:.2}", stats.cycles_per_instruction());
    println!("  缓存命中率: {:.2}%", stats.hit_rate * 100.0);
    println!();
    
    // 生成ROM文件
    println!("🎮 生成ROM文件...");
    let mut rom_generator = RomGenerator::new("ADVANCED DEMO");
    rom_generator.add_program(0x100, &program);
    
    let filename = "advanced_demo.gb";
    rom_generator.save_rom(filename).map_err(|e| e.to_string())?;
    println!("✅ ROM文件生成成功: {}", filename);
    
    // 显示最终状态
    println!();
    println!("🎉 演示完成！");
    println!("{}", gameboy.get_debug_info());
    
    Ok(())
}
