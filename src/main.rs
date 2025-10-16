//! Game Boy模拟器主程序

use gameboy_emulator::GameBoy;

fn main() {
    println!("🎮 Game Boy模拟器启动！");
    
    // 创建模拟器实例
    let mut gameboy = GameBoy::new();
    
    // 加载测试程序
    let test_program = [
        0x00, // NOP指令
        0x01, // LD BC, 0x1234指令
        0x11, // LD DE, 0x5678指令
        0x02, // LD (BC), A指令
        0x12, // LD (DE), A指令
        0x0A, // LD A, (BC)指令
        0x1A, // LD A, (DE)指令
        0x18, // JR +5指令（相对跳转）
        0x81, // ADD A, C指令（会被跳过）
        0x79, // LD A, C指令（会被跳过）
        0x00, // NOP指令（会被跳过）
        0x00, // NOP指令（会被跳过）
        0xC3, // JP 0x200指令（绝对跳转）
        0x00, // NOP指令（会被跳过）
    ];
    
    // 在0x200处放置一些指令
    let additional_program = [
        0x0D, // DEC C指令
        0x00, // NOP指令
    ];
    
    gameboy.load_program(0x100, &test_program);
    gameboy.load_program(0x200, &additional_program);
    
    // 显示初始状态
    let initial_state = gameboy.get_cpu_state();
    println!("初始状态: A={}, B={}, C={}, PC=0x{:x}, SP=0x{:x}", 
             initial_state.registers.a, initial_state.registers.b, 
             initial_state.registers.c, initial_state.pc, initial_state.sp);
    println!("16位寄存器: BC=0x{:x}, DE=0x{:x}, HL=0x{:x}", 
             initial_state.registers.get_bc(), initial_state.registers.get_de(), 
             initial_state.registers.get_hl());
    println!("标志位: Z={}, N={}, H={}, C={}", 
             initial_state.flags.zero, initial_state.flags.subtract, 
             initial_state.flags.half_carry, initial_state.flags.carry);
    
    // 执行指令序列
    let instructions = [
        "NOP", "LD BC, 0x1234", "LD DE, 0x5678", "LD (BC), A", 
        "LD (DE), A", "LD A, (BC)", "LD A, (DE)", "JR +5", 
        "ADD A, C", "LD A, C", "NOP", "NOP", "JP 0x200", 
        "NOP", "DEC C", "NOP"
    ];
    
    for (i, instruction_name) in instructions.iter().enumerate() {
        let old_state = gameboy.get_cpu_state();
        
        match gameboy.step() {
            Ok(()) => {
                let new_state = gameboy.get_cpu_state();
                println!("步骤{} - 执行{}: A={}, B={}, C={}, PC: 0x{:x} -> 0x{:x}", 
                         i + 1, instruction_name, new_state.registers.a, 
                         new_state.registers.b, new_state.registers.c, 
                         old_state.pc, new_state.pc);
                println!("  16位寄存器: BC=0x{:x}, DE=0x{:x}, HL=0x{:x}, SP=0x{:x}", 
                         new_state.registers.get_bc(), new_state.registers.get_de(), 
                         new_state.registers.get_hl(), new_state.sp);
                println!("  标志位: Z={}, N={}, H={}, C={}", 
                         new_state.flags.zero, new_state.flags.subtract, 
                         new_state.flags.half_carry, new_state.flags.carry);
            }
            Err(e) => {
                println!("❌ 执行错误: {}", e);
                break;
            }
        }
        
        // 防止无限循环
        if i > 20 {
            println!("⚠️  防止无限循环，停止执行");
            break;
        }
    }
    
    // 显示最终状态
    let final_state = gameboy.get_cpu_state();
    println!("\n🎉 模拟器运行完成！");
    println!("最终结果: A={}, B={}, C={}, PC=0x{:x}, SP=0x{:x}", 
             final_state.registers.a, final_state.registers.b, 
             final_state.registers.c, final_state.pc, final_state.sp);
    println!("最终16位寄存器: BC=0x{:x}, DE=0x{:x}, HL=0x{:x}", 
             final_state.registers.get_bc(), final_state.registers.get_de(), 
             final_state.registers.get_hl());
    println!("最终标志位: Z={}, N={}, H={}, C={}", 
             final_state.flags.zero, final_state.flags.subtract, 
             final_state.flags.half_carry, final_state.flags.carry);
}