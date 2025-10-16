//! 高级GameBoy模拟器 - 集成所有功能

use crate::cpu::{OptimizedCPU, PerformanceStats, Registers, FlagsRegister};
use crate::memory::MemoryBus;
use crate::gpu::LCD;
use crate::debug::{Debugger, DebuggerState, LogLevel};
use crate::instructions::Instruction;

/// CPU状态快照
#[derive(Debug, Clone)]
pub struct CPUState {
    pub pc: u16,
    pub sp: u16,
    pub registers: Registers,
    pub flags: FlagsRegister,
    pub cycle_count: u64,
}

/// 高级GameBoy模拟器
#[derive(Debug)]
pub struct AdvancedGameBoy {
    pub cpu: OptimizedCPU,
    pub lcd: LCD,
    pub debugger: Debugger,
    pub running: bool,
    pub frame_count: u64,
    pub target_fps: u32,
    pub frame_time: std::time::Duration,
}

impl AdvancedGameBoy {
    /// 创建新的高级GameBoy模拟器
    pub fn new() -> Self {
        let bus = MemoryBus::new();
        Self {
            cpu: OptimizedCPU::new(bus),
            lcd: LCD::new(),
            debugger: Debugger::new(),
            running: false,
            frame_count: 0,
            target_fps: 60,
            frame_time: std::time::Duration::from_millis(16), // ~60 FPS
        }
    }

    /// 启动模拟器
    pub fn start(&mut self) {
        self.running = true;
        self.debugger.log(LogLevel::Info, "高级GameBoy模拟器启动");
    }

    /// 停止模拟器
    pub fn stop(&mut self) {
        self.running = false;
        self.debugger.log(LogLevel::Info, "高级GameBoy模拟器停止");
    }

    /// 重置模拟器
    pub fn reset(&mut self) {
        let bus = MemoryBus::new();
        self.cpu = OptimizedCPU::new(bus);
        self.lcd.reset();
        self.debugger = Debugger::new();
        self.running = false;
        self.frame_count = 0;
        self.debugger.log(LogLevel::Info, "模拟器已重置");
    }

    /// 执行一步模拟
    pub fn step(&mut self) -> Result<(), String> {
        if !self.running {
            return Ok(());
        }

        // 检查调试器状态
        match self.debugger.state {
            DebuggerState::Paused => return Ok(()),
            DebuggerState::BreakpointHit => return Ok(()),
            _ => {}
        }

        // 检查断点
        if self.debugger.check_breakpoint(self.cpu.pc, &self.cpu.registers, &self.cpu.flags) {
            return Ok(());
        }

        // 执行CPU指令
        self.cpu.step_optimized()?;
        self.debugger.increment_step_count();

        // 更新LCD
        self.lcd.update(1); // 假设每个指令1个周期

        // 检查最大步数限制
        if self.debugger.check_max_steps() {
            self.stop();
            self.debugger.log(LogLevel::Info, "达到最大步数限制，停止执行");
        }

        Ok(())
    }

    /// 运行指定步数
    pub fn run_steps(&mut self, steps: u64) -> Result<(), String> {
        self.debugger.set_max_steps(Some(self.debugger.step_count + steps));
        self.start();

        for _ in 0..steps {
            self.step()?;
        }

        Ok(())
    }

    /// 运行到断点
    pub fn run_to_breakpoint(&mut self) -> Result<(), String> {
        self.start();
        
        while self.running && self.debugger.state == DebuggerState::Running {
            self.step()?;
        }

        Ok(())
    }

    /// 单步执行
    pub fn step_once(&mut self) -> Result<(), String> {
        println!("STEP_ONCE: running={}, state={:?}", self.running, self.debugger.state);
        if !self.running {
            return Ok(());
        }

        // 检查断点
        if self.debugger.check_breakpoint(self.cpu.pc, &self.cpu.registers, &self.cpu.flags) {
            return Ok(());
        }

        // 执行CPU指令
        println!("BEFORE: PC={:04X}, 周期={}, 指令={}", self.cpu.pc, self.cpu.cycle_count, self.cpu.instruction_count);
        self.cpu.step_optimized()?;
        println!("AFTER: PC={:04X}, 周期={}, 指令={}", self.cpu.pc, self.cpu.cycle_count, self.cpu.instruction_count);
        self.debugger.increment_step_count();

        // 更新LCD
        self.lcd.update(1); // 假设每个指令1个周期

        // 检查最大步数限制
        if self.debugger.check_max_steps() {
            self.stop();
            self.debugger.log(LogLevel::Info, "达到最大步数限制，停止执行");
        }

        Ok(())
    }

    /// 设置断点
    pub fn set_breakpoint(&mut self, address: u16, condition: Option<String>) {
        self.debugger.set_breakpoint(address, condition);
    }

    /// 移除断点
    pub fn remove_breakpoint(&mut self, address: u16) {
        self.debugger.remove_breakpoint(address);
    }

    /// 获取性能统计
    pub fn get_performance_stats(&self) -> PerformanceStats {
        self.cpu.get_performance_stats()
    }

    /// 获取CPU状态
    pub fn get_cpu_state(&self) -> CPUState {
        CPUState {
            pc: self.cpu.pc,
            sp: self.cpu.sp,
            registers: self.cpu.registers.clone(),
            flags: self.cpu.flags.clone(),
            cycle_count: self.debugger.step_count,
        }
    }

    /// 获取调试信息
    pub fn get_debug_info(&self) -> String {
        let stats = self.get_performance_stats();
        let cpu_state = self.get_cpu_state();
        
        format!(
            "=== 调试信息 ===\n\
            状态: {:?}\n\
            PC: 0x{:04X}\n\
            SP: 0x{:04X}\n\
            寄存器: A=0x{:02X}, B=0x{:02X}, C=0x{:02X}, D=0x{:02X}, E=0x{:02X}, H=0x{:02X}, L=0x{:02X}\n\
            标志: Z={}, N={}, H={}, C={}\n\
            周期数: {}\n\
            指令数: {}\n\
            缓存命中率: {:.2}%\n\
            帧数: {}\n\
            ================",
            self.debugger.state,
            cpu_state.pc,
            cpu_state.sp,
            cpu_state.registers.a, cpu_state.registers.b, cpu_state.registers.c,
            cpu_state.registers.d, cpu_state.registers.e, cpu_state.registers.h, cpu_state.registers.l,
            cpu_state.flags.zero, cpu_state.flags.subtract, cpu_state.flags.half_carry, cpu_state.flags.carry,
            stats.cycle_count,
            stats.instruction_count,
            stats.hit_rate * 100.0,
            self.frame_count
        )
    }

    /// 获取LCD帧缓冲区
    pub fn get_framebuffer(&self) -> &[u8] {
        self.lcd.get_framebuffer()
    }

    /// 设置目标FPS
    pub fn set_target_fps(&mut self, fps: u32) {
        self.target_fps = fps;
        self.frame_time = std::time::Duration::from_millis(1000 / fps as u64);
    }

    /// 设置日志级别
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.debugger.set_log_level(level);
    }

    /// 加载程序
    pub fn load_program(&mut self, start_address: u16, program: &[u8]) -> Result<(), String> {
        self.cpu.bus.load_program(start_address, program);
        self.debugger.log(LogLevel::Info, &format!("程序已加载到地址 0x{:04X}", start_address));
        Ok(())
    }

    /// 获取内存引用
    pub fn memory(&self) -> &[u8] {
        self.cpu.bus.memory()
    }

    /// 反汇编指令
    pub fn disassemble_instruction(&self, pc: u16) -> String {
        self.debugger.disassemble_instruction(pc, &self.cpu.bus)
    }

    /// 反汇编内存区域
    pub fn disassemble_range(&self, start: u16, end: u16) -> Vec<String> {
        self.debugger.disassemble_range(start, end, &self.cpu.bus)
    }
}

impl Default for AdvancedGameBoy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_gameboy_creation() {
        let gameboy = AdvancedGameBoy::new();
        assert!(!gameboy.running);
        assert_eq!(gameboy.frame_count, 0);
    }

    #[test]
    fn test_breakpoint_management() {
        let mut gameboy = AdvancedGameBoy::new();
        gameboy.set_breakpoint(0x100, None);
        assert_eq!(gameboy.debugger.breakpoints.len(), 1);
        
        gameboy.remove_breakpoint(0x100);
        assert_eq!(gameboy.debugger.breakpoints.len(), 0);
    }

    #[test]
    fn test_reset() {
        let mut gameboy = AdvancedGameBoy::new();
        gameboy.start();
        gameboy.reset();
        assert!(!gameboy.running);
        assert_eq!(gameboy.frame_count, 0);
    }
}
