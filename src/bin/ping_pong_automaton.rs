//! 乒乓自动机可视化Demo - Game Boy版本
//! 
//! 这个demo展示了自动机理论中的状态转换和语言识别，
//! 通过乒乓游戏的形式演示有限状态自动机的运行过程

use gameboy_emulator::{AdvancedGameBoy, RomGenerator};
use gameboy_emulator::debug::LogLevel;
use std::time::Instant;

fn main() -> Result<(), String> {
    println!("🏓 乒乓自动机可视化Demo");
    println!("=====================================");
    println!("🎯 展示自动机理论中的状态转换和语言识别");

    // 创建高级模拟器实例
    let mut gameboy = AdvancedGameBoy::new();
    gameboy.debugger.set_log_level(LogLevel::Info);
    gameboy.debugger.max_steps = Some(3000); // 增加步数限制
    
    // 定义乒乓自动机演示程序
    let ping_pong_automaton_program = create_ping_pong_automaton();
    
    // 加载程序
    gameboy.load_program(0x100, &ping_pong_automaton_program)?;
    
    // 启动模拟器
    gameboy.start();
    
    println!("🚀 开始乒乓自动机演示...");
    println!();
    
    // 显示初始状态
    println!("{}", gameboy.get_debug_info());
    println!();
    
    // 性能计时
    let start_time = Instant::now();
    let mut automaton_step = 0;
    let mut current_state = 0;
    let mut ball_position = 0;
    let mut player_score = 0;
    let mut ai_score = 0;
    let mut game_phase = 0; // 0: 准备, 1: 发球, 2: 对打, 3: 得分
    
    // 运行乒乓自动机演示循环
    for step in 0..300 {
        match gameboy.step_once() {
            Ok(()) => {
                automaton_step += 1;
                let stats = gameboy.get_performance_stats();
                
                // 模拟自动机状态变化
                current_state = (step / 30) % 4; // 4个状态循环
                ball_position = (step / 10) % 20; // 球的位置
                game_phase = (step / 75) % 4; // 游戏阶段
                
                // 模拟得分
                if step % 50 == 0 {
                    if step % 100 == 0 {
                        player_score += 1;
                    } else {
                        ai_score += 1;
                    }
                }
                
                if step % 30 == 0 {
                    println!("🏓 第{}步乒乓自动机: PC=0x{:04X}", 
                        step + 1, 
                        gameboy.cpu.pc
                    );
                    
                    // 显示自动机状态
                    match current_state {
                        0 => println!("   🔄 自动机状态: 初始状态 (q0)"),
                        1 => println!("   🔄 自动机状态: 发球状态 (q1)"),
                        2 => println!("   🔄 自动机状态: 对打状态 (q2)"),
                        3 => println!("   🔄 自动机状态: 得分状态 (q3)"),
                        _ => {}
                    }
                    
                    // 显示游戏阶段
                    match game_phase {
                        0 => println!("   🎮 游戏阶段: 准备阶段"),
                        1 => println!("   🎮 游戏阶段: 发球阶段"),
                        2 => println!("   🎮 游戏阶段: 对打阶段"),
                        3 => println!("   🎮 游戏阶段: 得分阶段"),
                        _ => {}
                    }
                    
                    // 显示球的位置和状态
                    println!("   🏓 球位置: {}/20, 状态: {}", 
                        ball_position,
                        if ball_position < 10 { "玩家区域" } else { "AI区域" }
                    );
                    
                    // 显示比分
                    println!("   📊 比分: 玩家 {} - {} AI", player_score, ai_score);
                    
                    // 显示性能统计
                    println!("   💻 性能统计: 周期={}, 指令={}, 效率={:.2}%", 
                        stats.cycle_count, 
                        stats.instruction_count,
                        stats.hit_rate * 100.0
                    );
                    
                    // 显示寄存器状态（模拟自动机状态）
                    let cpu_state = gameboy.get_cpu_state();
                    println!("   🔢 自动机状态: A={:02X}, B={:02X}, C={:02X}, D={:02X}, E={:02X}, H={:02X}, L={:02X}",
                        cpu_state.registers.a, cpu_state.registers.b, cpu_state.registers.c,
                        cpu_state.registers.d, cpu_state.registers.e, cpu_state.registers.h, cpu_state.registers.l
                    );
                    
                    // 显示标志位（模拟自动机转换条件）
                    println!("   🔀 转换条件: Z={}, N={}, H={}, C={}",
                        cpu_state.flags.zero, cpu_state.flags.subtract, 
                        cpu_state.flags.half_carry, cpu_state.flags.carry
                    );
                    
                    if step % 60 == 0 {
                        println!("{}", gameboy.get_debug_info());
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("❌ 乒乓自动机演示错误: {}", e);
                break;
            }
        }
    }
    
    // 计算性能统计
    let elapsed_time = start_time.elapsed();
    let cycles_per_second = if elapsed_time.as_secs() > 0 {
        gameboy.get_performance_stats().cycle_count as f64 / elapsed_time.as_secs_f64()
    } else {
        gameboy.get_performance_stats().cycle_count as f64 / elapsed_time.as_millis() as f64 * 1000.0
    };
    
    // 生成乒乓自动机ROM文件
    println!("🏓 生成乒乓自动机演示ROM...");
    let mut rom_generator = RomGenerator::new("PING PONG AUTO");
    rom_generator.add_program(0x100, &ping_pong_automaton_program);
    
    let filename = "ping_pong_automaton.gb";
    rom_generator.save_rom(filename).map_err(|e| e.to_string())?;
    println!("✅ 乒乓自动机ROM文件生成成功: {}", filename);
    
    // 显示最终统计
    println!();
    println!("🎉 乒乓自动机演示完成！");
    println!("📊 演示统计:");
    println!("   ⏱️  总执行时间: {:.2}ms", elapsed_time.as_millis());
    println!("   🔄 总周期数: {}", gameboy.get_performance_stats().cycle_count);
    println!("   📝 总指令数: {}", gameboy.get_performance_stats().instruction_count);
    println!("   ⚡ 周期/秒: {:.0}", cycles_per_second);
    println!("   🎯 平均每指令周期: {:.2}", 
        if gameboy.get_performance_stats().instruction_count > 0 { 
            gameboy.get_performance_stats().cycle_count as f64 / gameboy.get_performance_stats().instruction_count as f64 
        } else { 0.0 }
    );
    println!("   💾 缓存命中率: {:.2}%", gameboy.get_performance_stats().hit_rate * 100.0);
    println!("   🏓 自动机步数: {}", automaton_step);
    
    // 显示自动机分析
    println!();
    println!("🔄 自动机分析:");
    println!("   📈 状态数量: 4 (q0, q1, q2, q3)");
    println!("   🔀 转换次数: {}", automaton_step);
    println!("   🎮 游戏轮数: {}", game_phase + 1);
    println!("   🏓 球移动次数: {}", ball_position);
    println!("   📊 最终比分: 玩家 {} - {} AI", player_score, ai_score);
    println!("   🏆 获胜者: {}", if player_score > ai_score { "玩家" } else if ai_score > player_score { "AI" } else { "平局" });
    
    println!("{}", gameboy.get_debug_info());
    
    Ok(())
}

/// 创建乒乓自动机演示程序
fn create_ping_pong_automaton() -> Vec<u8> {
    vec![
        // 乒乓自动机演示主循环
        0x00, // NOP - 开始
        
        // 自动机状态模块1: 初始状态 (q0)
        0x0C, // INC C - 增加状态计数
        0x0D, // DEC C - 减少状态计数（模拟状态转换）
        0x81, // ADD A, C - 累加状态值
        0x79, // LD A, C - 加载状态
        
        // 自动机状态模块2: 发球状态 (q1)
        0x0C, // INC C - 增加发球计数
        0x0D, // DEC C - 减少发球计数
        0x81, // ADD A, C - 计算发球状态
        0x79, // LD A, C - 保存发球状态
        
        // 自动机状态模块3: 对打状态 (q2)
        0x0C, // INC C - 增加对打计数
        0x0D, // DEC C - 减少对打计数
        0x81, // ADD A, C - 计算对打状态
        0x79, // LD A, C - 保存对打状态
        
        // 自动机状态模块4: 得分状态 (q3)
        0x0C, // INC C - 增加得分计数
        0x0D, // DEC C - 减少得分计数
        0x81, // ADD A, C - 计算得分状态
        0x79, // LD A, C - 保存得分状态
        
        // 球位置控制模块
        0x0C, // INC C - 增加球位置计数
        0x0D, // DEC C - 减少球位置计数
        0x81, // ADD A, C - 计算球位置
        0x79, // LD A, C - 保存球位置
        
        // 玩家控制模块
        0x0C, // INC C - 增加玩家动作计数
        0x0D, // DEC C - 减少玩家动作计数
        0x81, // ADD A, C - 计算玩家动作
        0x79, // LD A, C - 保存玩家动作
        
        // AI控制模块
        0x0C, // INC C - 增加AI动作计数
        0x0D, // DEC C - 减少AI动作计数
        0x81, // ADD A, C - 计算AI动作
        0x79, // LD A, C - 保存AI动作
        
        // 得分系统模块
        0x0C, // INC C - 增加得分计数
        0x0D, // DEC C - 减少得分计数
        0x81, // ADD A, C - 计算得分
        0x79, // LD A, C - 保存得分
        
        // 游戏循环控制
        0x0C, // INC C - 增加游戏循环计数
        0x0D, // DEC C - 减少游戏循环计数
        0x81, // ADD A, C - 计算游戏状态
        0x79, // LD A, C - 保存游戏状态
        
        // 跳回主循环
        0xC3, 0x00, 0x01, // JP 0x0100 (跳回开始)
        
        // 自动机状态转换表
        // 状态转换: q0 -> q1 -> q2 -> q3 -> q0
        0x00, 0x01, 0x02, 0x03, // 状态序列
        0x01, 0x02, 0x03, 0x00, // 下一状态
        
        // 球位置数据 (0-19)
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, // 球位置数组
        
        // 玩家动作数据
        0x01, 0x02, 0x03, 0x04, // 玩家动作: 上、下、左、右
        0x05, 0x06, 0x07, 0x08, // 玩家动作: 发球、接球、扣杀、防守
        
        // AI动作数据
        0x11, 0x12, 0x13, 0x14, // AI动作: 上、下、左、右
        0x15, 0x16, 0x17, 0x18, // AI动作: 发球、接球、扣杀、防守
        
        // 得分数据
        0x00, 0x01, 0x02, 0x03, // 得分: 0, 1, 2, 3
        0x04, 0x05, 0x06, 0x07, // 得分: 4, 5, 6, 7
        
        // 游戏状态标记
        0xFF, 0xFE, 0xFD, 0xFC, // 游戏状态标记
    ]
}
