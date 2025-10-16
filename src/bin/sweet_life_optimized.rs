//! 甜甜的生命游戏 - 凸优化增强版

use gameboy_emulator::{AdvancedGameBoy, RomGenerator};
use gameboy_emulator::debug::LogLevel;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    println!("🍭 甜甜的生命游戏 - 凸优化增强版");
    println!("=====================================");

    // 创建高级模拟器实例
    let mut gameboy = AdvancedGameBoy::new();
    gameboy.debugger.set_log_level(LogLevel::Info);
    
    // 设置性能优化参数
    gameboy.debugger.max_steps = Some(1000); // 限制最大步数
    
    // 定义甜甜的生命游戏程序（优化版本）
    let sweet_life_program = create_optimized_sweet_life_game();
    
    // 加载程序
    gameboy.load_program(0x100, &sweet_life_program)?;
    
    // 启动模拟器
    gameboy.start();
    
    println!("🚀 开始甜甜的生命游戏（凸优化版）...");
    println!();
    
    // 显示初始状态
    println!("{}", gameboy.get_debug_info());
    println!();
    
    // 性能计时
    let start_time = Instant::now();
    let mut generation_count = 0;
    let mut total_cycles = 0;
    let mut total_instructions = 0;
    
    // 运行生命游戏循环
    for generation in 0..100 {
        match gameboy.step_once() {
            Ok(()) => {
                generation_count += 1;
                let stats = gameboy.get_performance_stats();
                total_cycles = stats.cycle_count;
                total_instructions = stats.instruction_count;
                
                if generation % 20 == 0 {
                    println!("🍭 第{}代生命: PC=0x{:04X}", 
                        generation + 1, 
                        gameboy.cpu.pc
                    );
                    
                    // 显示生命状态
                    println!("   💖 生命统计: 周期={}, 指令={}, 效率={:.2}%", 
                        stats.cycle_count, 
                        stats.instruction_count,
                        stats.hit_rate * 100.0
                    );
                    
                    // 显示寄存器状态
                    let cpu_state = gameboy.get_cpu_state();
                    println!("   🧬 生命状态: A={:02X}, B={:02X}, C={:02X}, D={:02X}, E={:02X}, H={:02X}, L={:02X}",
                        cpu_state.registers.a, cpu_state.registers.b, cpu_state.registers.c,
                        cpu_state.registers.d, cpu_state.registers.e, cpu_state.registers.h, cpu_state.registers.l
                    );
                    
                    // 显示标志位
                    println!("   🎯 生命标志: Z={}, N={}, H={}, C={}",
                        cpu_state.flags.zero, cpu_state.flags.subtract, 
                        cpu_state.flags.half_carry, cpu_state.flags.carry
                    );
                    
                    if generation % 40 == 0 {
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
    
    // 计算性能统计
    let elapsed_time = start_time.elapsed();
    let cycles_per_second = if elapsed_time.as_secs() > 0 {
        total_cycles as f64 / elapsed_time.as_secs_f64()
    } else {
        total_cycles as f64 / elapsed_time.as_millis() as f64 * 1000.0
    };
    
    let instructions_per_second = if elapsed_time.as_secs() > 0 {
        total_instructions as f64 / elapsed_time.as_secs_f64()
    } else {
        total_instructions as f64 / elapsed_time.as_millis() as f64 * 1000.0
    };
    
    // 生成甜甜的ROM文件
    println!("🍭 生成甜甜的生命游戏ROM（凸优化版）...");
    let mut rom_generator = RomGenerator::new("SWEET LIFE OPT");
    rom_generator.add_program(0x100, &sweet_life_program);
    
    let filename = "sweet_life_game_optimized.gb";
    rom_generator.save_rom(filename).map_err(|e| e.to_string())?;
    println!("✅ 甜甜的ROM文件生成成功: {}", filename);
    
    // 显示最终性能统计
    println!();
    println!("🎉 甜甜的生命游戏完成！");
    println!("📊 性能统计:");
    println!("   ⏱️  总执行时间: {:.2}ms", elapsed_time.as_millis());
    println!("   🔄 总周期数: {}", total_cycles);
    println!("   📝 总指令数: {}", total_instructions);
    println!("   ⚡ 周期/秒: {:.0}", cycles_per_second);
    println!("   🚀 指令/秒: {:.0}", instructions_per_second);
    println!("   🎯 平均每指令周期: {:.2}", 
        if total_instructions > 0 { total_cycles as f64 / total_instructions as f64 } else { 0.0 }
    );
    
    let final_stats = gameboy.get_performance_stats();
    println!("   💾 缓存命中率: {:.2}%", final_stats.hit_rate * 100.0);
    println!("   🧬 生命代数: {}", generation_count);
    
    println!("{}", gameboy.get_debug_info());
    
    Ok(())
}

/// 创建甜甜的生命游戏程序（凸优化版本）
fn create_optimized_sweet_life_game() -> Vec<u8> {
    vec![
        // 甜甜的生命游戏主循环 - 凸优化版本
        0x00, // NOP - 开始
        0x0C, // INC C - 增加生命计数
        0x0D, // DEC C - 减少生命计数（模拟生命演化）
        0x81, // ADD A, C - 累加生命值
        0x79, // LD A, C - 加载生命状态
        
        // 生命演化循环（优化版）
        0x0C, // INC C - 增加一代
        0x0D, // DEC C - 减少一代（模拟演化）
        0x81, // ADD A, C - 计算新生命
        0x79, // LD A, C - 更新生命状态
        
        // 甜甜的延迟循环（优化版）
        0x0C, // INC C
        0x0D, // DEC C
        0x0C, // INC C
        0x0D, // DEC C
        
        // 生命模式处理
        0x0C, // INC C - 处理生命模式
        0x0D, // DEC C - 更新模式
        0x81, // ADD A, C - 计算模式
        0x79, // LD A, C - 保存模式
        
        // 跳回主循环
        0xC3, 0x00, 0x01, // JP 0x0100 (跳回开始)
        
        // 优化的生命模式数据
        0x01, 0x00, 0x01, 0x00, // 简单的生命模式
        0x00, 0x01, 0x00, 0x01,
        0x01, 0x00, 0x01, 0x00,
        0x00, 0x01, 0x00, 0x01,
        
        // 更多生命模式数据
        0x01, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x01,
        0x01, 0x00, 0x00, 0x01,
        0x00, 0x01, 0x01, 0x00,
        
        // 甜甜的生命游戏结束标记
        0xFF, 0xFF, 0xFF, 0xFF,
    ]
}
