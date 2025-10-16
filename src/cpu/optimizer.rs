//! CPU优化器模块 - 提供性能优化功能

use super::{CPU, Registers, FlagsRegister};
use crate::memory::MemoryBus;
use crate::instructions::Instruction;

/// 指令缓存条目
#[derive(Debug, Clone)]
pub struct InstructionCacheEntry {
    pub instruction: Instruction,
    pub cycles: u8,
    pub size: u8,
}

/// CPU优化器
#[derive(Debug)]
pub struct CPUOptimizer {
    instruction_cache: Vec<Option<InstructionCacheEntry>>,
    cache_size: usize,
    hit_count: u64,
    miss_count: u64,
}

impl CPUOptimizer {
    /// 创建新的CPU优化器
    pub fn new(cache_size: usize) -> Self {
        Self {
            instruction_cache: vec![None; cache_size],
            cache_size,
            hit_count: 0,
            miss_count: 0,
        }
    }

    /// 获取缓存的指令
    pub fn get_cached_instruction(&mut self, pc: u16) -> Option<&InstructionCacheEntry> {
        let index = (pc as usize) % self.cache_size;
        
        if let Some(entry) = &self.instruction_cache[index] {
            self.hit_count += 1;
            Some(entry)
        } else {
            self.miss_count += 1;
            None
        }
    }

    /// 缓存指令
    pub fn cache_instruction(&mut self, pc: u16, instruction: Instruction, cycles: u8, size: u8) {
        let index = (pc as usize) % self.cache_size;
        self.instruction_cache[index] = Some(InstructionCacheEntry {
            instruction,
            cycles,
            size,
        });
    }

    /// 获取缓存统计
    pub fn get_cache_stats(&self) -> (u64, u64, f64) {
        let total = self.hit_count + self.miss_count;
        let hit_rate = if total > 0 {
            self.hit_count as f64 / total as f64
        } else {
            0.0
        };
        (self.hit_count, self.miss_count, hit_rate)
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.instruction_cache.fill(None);
        self.hit_count = 0;
        self.miss_count = 0;
    }
}

/// 优化的CPU结构
#[derive(Debug)]
pub struct OptimizedCPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub flags: FlagsRegister,
    pub bus: MemoryBus,
    pub optimizer: CPUOptimizer,
    pub cycle_count: u64,
    pub instruction_count: u64,
}

impl OptimizedCPU {
    /// 创建新的优化CPU实例
    pub fn new(bus: MemoryBus) -> Self {
        Self {
            registers: Registers::new(),
            pc: 0x100,
            sp: 0xFFFE,
            flags: FlagsRegister::new(),
            bus,
            optimizer: CPUOptimizer::new(1024), // 1KB指令缓存
            cycle_count: 0,
            instruction_count: 0,
        }
    }

    /// 优化的指令执行
    pub fn step_optimized(&mut self) -> Result<(), String> {
        let pc = self.pc;
        
        // 暂时禁用缓存优化，直接执行指令
        let instruction_byte = self.bus.read_byte(pc);
        let instruction = Instruction::from_byte(instruction_byte)
            .ok_or_else(|| format!("未知指令: 0x{:02X}", instruction_byte))?;
        
        // 执行指令
        let (cycles, size) = self.execute_instruction(&instruction)?;
        
        // 更新PC（除非指令改变了PC，如跳转指令）
        if !matches!(instruction, Instruction::JP(_) | Instruction::JR(_)) {
            self.pc += size as u16;
        }
        
        // 缓存指令（用于统计）
        self.optimizer.cache_instruction(pc, instruction, cycles, size);
        
        // 更新统计信息
        self.cycle_count += cycles as u64;
        self.instruction_count += 1;
        
        // 调试信息
        println!("DEBUG: PC={:04X}, 指令={:?}, 周期={}, 大小={}, 总周期={}, 总指令={}", 
                pc, instruction, cycles, size, self.cycle_count, self.instruction_count);
        
        Ok(())
    }

    /// 执行指令并返回周期数和大小
    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(u8, u8), String> {
        match instruction {
            Instruction::NOP => {
                Ok((1, 1)) // 1周期，1字节
            }
            Instruction::ADD(target) => {
                let value = self.get_arithmetic_target_value(*target);
                self.add(value);
                Ok((1, 1)) // 1周期，1字节
            }
            Instruction::SUB(target) => {
                let value = self.get_arithmetic_target_value(*target);
                self.sub(value);
                Ok((1, 1)) // 1周期，1字节
            }
            Instruction::INC(target) => {
                self.inc(*target);
                Ok((1, 1)) // 1周期，1字节
            }
            Instruction::DEC(target) => {
                self.dec(*target);
                Ok((1, 1)) // 1周期，1字节
            }
            Instruction::LD(target, source) => {
                self.ld(*target, *source);
                Ok((1, 1)) // 1周期，1字节
            }
            Instruction::LD16(target, source) => {
                self.ld16(*target, *source);
                Ok((3, 3)) // 3周期，3字节
            }
            Instruction::INC16(target) => {
                self.inc16(*target);
                Ok((2, 1)) // 2周期，1字节
            }
            Instruction::DEC16(target) => {
                self.dec16(*target);
                Ok((2, 1)) // 2周期，1字节
            }
            Instruction::JP(target) => {
                self.jp(*target);
                Ok((4, 3)) // 4周期，3字节
            }
            Instruction::JR(target) => {
                self.jr(*target);
                Ok((3, 2)) // 3周期，2字节
            }
        }
    }

    /// 获取性能统计
    pub fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            cycle_count: self.cycle_count,
            instruction_count: self.instruction_count,
            cache_hits: self.optimizer.hit_count,
            cache_misses: self.optimizer.miss_count,
            hit_rate: self.optimizer.get_cache_stats().2,
        }
    }

    // 算术运算方法
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let half_carry = ((self.registers.a & 0xF) + (value & 0xF)) > 0xF;
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = false;
        self.flags.half_carry = half_carry;
        self.flags.carry = did_overflow;
        
        self.registers.a = new_value;
        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let half_carry = (self.registers.a & 0xF) < (value & 0xF);
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = true;
        self.flags.half_carry = half_carry;
        self.flags.carry = did_overflow;
        
        self.registers.a = new_value;
        new_value
    }

    fn inc(&mut self, target: crate::instructions::ArithmeticTarget) -> u8 {
        let value = self.get_arithmetic_target_value(target);
        let (new_value, _did_overflow) = value.overflowing_add(1);
        let half_carry = (value & 0xF) == 0xF;
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = false;
        self.flags.half_carry = half_carry;
        // INC指令不影响进位标志
        
        self.set_arithmetic_target_value(target, new_value);
        new_value
    }

    fn dec(&mut self, target: crate::instructions::ArithmeticTarget) -> u8 {
        let value = self.get_arithmetic_target_value(target);
        let (new_value, _did_overflow) = value.overflowing_sub(1);
        let half_carry = (value & 0xF) == 0;
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = true;
        self.flags.half_carry = half_carry;
        // DEC指令不影响进位标志
        
        self.set_arithmetic_target_value(target, new_value);
        new_value
    }

    fn ld(&mut self, target: crate::instructions::LoadTarget, source: crate::instructions::LoadSource) {
        let value = self.get_load_source_value(source).unwrap_or(0);
        self.set_load_target_value(target, value);
    }

    fn ld16(&mut self, target: crate::instructions::LoadTarget16, source: crate::instructions::LoadSource16) {
        let value = self.get_load_source16_value(source).unwrap_or(0);
        self.set_load_target16_value(target, value);
    }

    fn inc16(&mut self, target: crate::instructions::LoadTarget16) {
        let value = self.get_load_target16_value(target).unwrap_or(0);
        let new_value = value.wrapping_add(1);
        self.set_load_target16_value(target, new_value);
    }

    fn dec16(&mut self, target: crate::instructions::LoadTarget16) {
        let value = self.get_load_target16_value(target).unwrap_or(0);
        let new_value = value.wrapping_sub(1);
        self.set_load_target16_value(target, new_value);
    }

    fn jp(&mut self, target: crate::instructions::JumpTarget) {
        match target {
            crate::instructions::JumpTarget::Immediate(addr) => {
                self.pc = addr;
            }
            _ => {} // 其他跳转类型暂未实现
        }
    }

    fn jr(&mut self, target: crate::instructions::JumpTarget) {
        match target {
            crate::instructions::JumpTarget::Relative(offset) => {
                self.pc = self.pc.wrapping_add(offset as u16);
            }
            _ => {} // 其他跳转类型暂未实现
        }
    }

    // 辅助方法
    fn get_arithmetic_target_value(&self, target: crate::instructions::ArithmeticTarget) -> u8 {
        match target {
            crate::instructions::ArithmeticTarget::A => self.registers.a,
            crate::instructions::ArithmeticTarget::B => self.registers.b,
            crate::instructions::ArithmeticTarget::C => self.registers.c,
            crate::instructions::ArithmeticTarget::D => self.registers.d,
            crate::instructions::ArithmeticTarget::E => self.registers.e,
            crate::instructions::ArithmeticTarget::H => self.registers.h,
            crate::instructions::ArithmeticTarget::L => self.registers.l,
        }
    }

    fn set_arithmetic_target_value(&mut self, target: crate::instructions::ArithmeticTarget, value: u8) {
        match target {
            crate::instructions::ArithmeticTarget::A => self.registers.a = value,
            crate::instructions::ArithmeticTarget::B => self.registers.b = value,
            crate::instructions::ArithmeticTarget::C => self.registers.c = value,
            crate::instructions::ArithmeticTarget::D => self.registers.d = value,
            crate::instructions::ArithmeticTarget::E => self.registers.e = value,
            crate::instructions::ArithmeticTarget::H => self.registers.h = value,
            crate::instructions::ArithmeticTarget::L => self.registers.l = value,
        }
    }

    fn get_load_source_value(&self, source: crate::instructions::LoadSource) -> Result<u8, String> {
        match source {
            crate::instructions::LoadSource::A => Ok(self.registers.a),
            crate::instructions::LoadSource::B => Ok(self.registers.b),
            crate::instructions::LoadSource::C => Ok(self.registers.c),
            crate::instructions::LoadSource::D => Ok(self.registers.d),
            crate::instructions::LoadSource::E => Ok(self.registers.e),
            crate::instructions::LoadSource::H => Ok(self.registers.h),
            crate::instructions::LoadSource::L => Ok(self.registers.l),
            crate::instructions::LoadSource::Immediate(val) => Ok(val),
        }
    }

    fn set_load_target_value(&mut self, target: crate::instructions::LoadTarget, value: u8) {
        match target {
            crate::instructions::LoadTarget::A => self.registers.a = value,
            crate::instructions::LoadTarget::B => self.registers.b = value,
            crate::instructions::LoadTarget::C => self.registers.c = value,
            crate::instructions::LoadTarget::D => self.registers.d = value,
            crate::instructions::LoadTarget::E => self.registers.e = value,
            crate::instructions::LoadTarget::H => self.registers.h = value,
            crate::instructions::LoadTarget::L => self.registers.l = value,
        }
    }

    fn get_load_source16_value(&self, source: crate::instructions::LoadSource16) -> Result<u16, String> {
        match source {
            crate::instructions::LoadSource16::BC => Ok(self.registers.get_bc()),
            crate::instructions::LoadSource16::DE => Ok(self.registers.get_de()),
            crate::instructions::LoadSource16::HL => Ok(self.registers.get_hl()),
            crate::instructions::LoadSource16::SP => Ok(self.sp),
            crate::instructions::LoadSource16::Immediate(val) => Ok(val),
        }
    }

    fn get_load_target16_value(&self, target: crate::instructions::LoadTarget16) -> Result<u16, String> {
        match target {
            crate::instructions::LoadTarget16::BC => Ok(self.registers.get_bc()),
            crate::instructions::LoadTarget16::DE => Ok(self.registers.get_de()),
            crate::instructions::LoadTarget16::HL => Ok(self.registers.get_hl()),
            crate::instructions::LoadTarget16::SP => Ok(self.sp),
        }
    }

    fn set_load_target16_value(&mut self, target: crate::instructions::LoadTarget16, value: u16) -> Result<(), String> {
        match target {
            crate::instructions::LoadTarget16::BC => self.registers.set_bc(value),
            crate::instructions::LoadTarget16::DE => self.registers.set_de(value),
            crate::instructions::LoadTarget16::HL => self.registers.set_hl(value),
            crate::instructions::LoadTarget16::SP => self.sp = value,
        }
        Ok(())
    }
}

/// 性能统计结构
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub cycle_count: u64,
    pub instruction_count: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub hit_rate: f64,
}

impl PerformanceStats {
    /// 计算平均每指令周期数
    pub fn cycles_per_instruction(&self) -> f64 {
        if self.instruction_count > 0 {
            self.cycle_count as f64 / self.instruction_count as f64
        } else {
            0.0
        }
    }

    /// 计算指令执行速度（指令/秒）
    pub fn instructions_per_second(&self, frequency_hz: u64) -> f64 {
        if self.cycle_count > 0 {
            (self.instruction_count as f64 * frequency_hz as f64) / self.cycle_count as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = CPUOptimizer::new(1024);
        assert_eq!(optimizer.cache_size, 1024);
    }

    #[test]
    fn test_performance_stats() {
        let stats = PerformanceStats {
            cycle_count: 1000,
            instruction_count: 500,
            cache_hits: 300,
            cache_misses: 200,
            hit_rate: 0.6,
        };
        
        assert_eq!(stats.cycles_per_instruction(), 2.0);
    }
}
