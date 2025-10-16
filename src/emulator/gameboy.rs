//! Game Boy模拟器核心

use crate::cpu::CPU;
use crate::memory::MemoryBus;

/// Game Boy模拟器主结构
#[derive(Debug)]
pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    /// 创建新的Game Boy模拟器实例
    pub fn new() -> Self {
        let bus = MemoryBus::new();
        let cpu = CPU::new(bus);
        
        Self { cpu }
    }

    /// 加载程序到模拟器
    pub fn load_program(&mut self, start_address: u16, program: &[u8]) {
        self.cpu.bus.load_program(start_address, program);
    }

    /// 执行一步指令
    pub fn step(&mut self) -> Result<(), String> {
        self.cpu.step()
    }

    /// 执行多步指令
    pub fn run_steps(&mut self, steps: usize) -> Result<(), String> {
        for _ in 0..steps {
            self.step()?;
        }
        Ok(())
    }

    /// 获取CPU状态
    pub fn get_cpu_state(&self) -> CPUState {
        CPUState {
            registers: self.cpu.registers,
            pc: self.cpu.pc,
            sp: self.cpu.sp,
            flags: self.cpu.flags,
        }
    }

    /// 获取内存引用
    pub fn memory(&self) -> &[u8] {
        self.cpu.bus.memory()
    }
}

/// CPU状态快照
#[derive(Debug, Clone, Copy)]
pub struct CPUState {
    pub registers: crate::cpu::Registers,
    pub pc: u16,
    pub sp: u16,
    pub flags: crate::cpu::FlagsRegister,
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::new()
    }
}
