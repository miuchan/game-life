//! 甜甜的生命游戏 - 凸优化版本

use gameboy_emulator::{AdvancedGameBoy, RomGenerator};
use gameboy_emulator::debug::LogLevel;

fn main() -> Result<(), String> {
    println!("🍭 甜甜的生命游戏 - 凸优化版本");
    println!("=====================================");

    // 创建高级模拟器实例
    let mut gameboy = AdvancedGameBoy::new();
    gameboy.debugger.set_log_level(LogLevel::Info);
    
    // 定义甜甜的生命游戏程序（使用已知指令）
    let sweet_life_program = create_sweet_life_game();
    
    // 加载程序
    gameboy.load_program(0x100, &sweet_life_program)?;
    
    // 启动模拟器
    gameboy.start();
    
    println!("🚀 开始甜甜的生命游戏...");
    println!();
    
    // 显示初始状态
    println!("{}", gameboy.get_debug_info());
    println!();
    
    // 运行生命游戏循环
    for generation in 0..50 {
        match gameboy.step_once() {
            Ok(()) => {
                if generation % 10 == 0 {
                    println!("🍭 第{}代生命: PC=0x{:04X}", 
                        generation + 1, 
                        gameboy.cpu.pc
                    );
                    
                    // 显示生命状态
                    let stats = gameboy.get_performance_stats();
                    println!("   💖 生命统计: 周期={}, 指令={}, 效率={:.2}%", 
                        stats.cycle_count, 
                        stats.instruction_count,
                        stats.hit_rate * 100.0
                    );
                    
                    if generation % 20 == 0 {
                        println!("{}", gameboy.get_debug_info());
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("❌ 生命演化错误: {}", e);
                break;
            }
        }
    }
    
    // 生成甜甜的ROM文件
    println!("🍭 生成甜甜的生命游戏ROM...");
    let mut rom_generator = RomGenerator::new("SWEET LIFE");
    rom_generator.add_program(0x100, &sweet_life_program);
    
    let filename = "sweet_life_game.gb";
    rom_generator.save_rom(filename).map_err(|e| e.to_string())?;
    println!("✅ 甜甜的ROM文件生成成功: {}", filename);
    
    // 显示最终生命状态
    println!();
    println!("🎉 甜甜的生命游戏完成！");
    println!("{}", gameboy.get_debug_info());
    
    Ok(())
}

/// 创建甜甜的生命游戏程序（使用已知指令）
fn create_sweet_life_game() -> Vec<u8> {
    vec![
        // 甜甜的生命游戏主循环
        0x00, // NOP - 开始
        0x0C, // INC C - 增加生命计数
        0x0D, // DEC C - 减少生命计数（模拟生命演化）
        0x81, // ADD A, C - 累加生命值
        0x79, // LD A, C - 加载生命状态
        0x0C, // INC C - 增加一代
        0x0D, // DEC C - 减少一代（模拟演化）
        0x81, // ADD A, C - 计算新生命
        0x79, // LD A, C - 更新生命状态
        
        // 甜甜的延迟循环
        0x0C, // INC C
        0x0D, // DEC C
        0x0C, // INC C
        0x0D, // DEC C
        0x0C, // INC C
        0x0D, // DEC C
        
        // 跳回主循环
        0xC3, 0x00, 0x01, // JP 0x0100 (跳回开始)
        
        // 生命模式数据
        0x01, 0x00, 0x01, 0x00, // 简单的生命模式
        0x00, 0x01, 0x00, 0x01,
        0x01, 0x00, 0x01, 0x00,
        0x00, 0x01, 0x00, 0x01,
    ]
}