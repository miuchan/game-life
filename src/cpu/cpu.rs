//! CPU核心模块 - 包含CPU执行逻辑

use crate::memory::MemoryBus;
use super::{Registers, FlagsRegister};
use super::registers::Register;

/// CPU结构
#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub pc: u16,        // 程序计数器
    pub sp: u16,        // 栈指针
    pub flags: FlagsRegister,
    pub bus: MemoryBus,
}

impl CPU {
    /// 创建新的CPU实例
    pub fn new(bus: MemoryBus) -> Self {
        Self {
            registers: Registers::new(),
            pc: 0x100,      // Game Boy程序通常从0x100开始
            sp: 0xFFFE,     // 栈指针初始化为0xFFFE
            flags: FlagsRegister::new(),
            bus,
        }
    }

    /// 执行一步指令
    pub fn step(&mut self) -> Result<(), String> {
        let instruction_byte = self.bus.read_byte(self.pc);
        
        let next_pc = match crate::instructions::Instruction::from_byte(instruction_byte) {
            Some(instruction) => self.execute(instruction)?,
            None => return Err(format!("未知指令: 0x{:02X}", instruction_byte)),
        };

        self.pc = next_pc;
        Ok(())
    }

    /// 执行指令
    fn execute(&mut self, instruction: crate::instructions::Instruction) -> Result<u16, String> {
        match instruction {
            crate::instructions::Instruction::ADD(target) => {
                self.execute_add(target)
            }
            crate::instructions::Instruction::SUB(target) => {
                self.execute_sub(target)
            }
            crate::instructions::Instruction::INC(target) => {
                self.execute_inc(target)
            }
            crate::instructions::Instruction::DEC(target) => {
                self.execute_dec(target)
            }
            crate::instructions::Instruction::LD(target, source) => {
                self.execute_ld(target, source)
            }
            crate::instructions::Instruction::LD16(target, source) => {
                self.execute_ld16(target, source)
            }
            crate::instructions::Instruction::INC16(target) => {
                self.execute_inc16(target)
            }
            crate::instructions::Instruction::DEC16(target) => {
                self.execute_dec16(target)
            }
            crate::instructions::Instruction::JP(target) => {
                self.execute_jp(target)
            }
            crate::instructions::Instruction::JR(target) => {
                self.execute_jr(target)
            }
            crate::instructions::Instruction::NOP => {
                Ok(self.pc + 1)
            }
        }
    }

    /// 执行ADD指令
    fn execute_add(&mut self, target: crate::instructions::ArithmeticTarget) -> Result<u16, String> {
        let value = self.get_register_value(target)?;
        let result = self.add(value);
        self.registers.a = result;
        Ok(self.pc + 1)
    }

    /// 执行SUB指令
    fn execute_sub(&mut self, target: crate::instructions::ArithmeticTarget) -> Result<u16, String> {
        let value = self.get_register_value(target)?;
        let result = self.sub(value);
        self.registers.a = result;
        Ok(self.pc + 1)
    }

    /// 执行INC指令
    fn execute_inc(&mut self, target: crate::instructions::ArithmeticTarget) -> Result<u16, String> {
        let reg = self.arithmetic_target_to_register(target)?;
        let value = self.registers.get_register(reg);
        let result = self.inc(value);
        self.registers.set_register(reg, result);
        Ok(self.pc + 1)
    }

    /// 执行DEC指令
    fn execute_dec(&mut self, target: crate::instructions::ArithmeticTarget) -> Result<u16, String> {
        let reg = self.arithmetic_target_to_register(target)?;
        let value = self.registers.get_register(reg);
        let result = self.dec(value);
        self.registers.set_register(reg, result);
        Ok(self.pc + 1)
    }

    /// 执行LD指令
    fn execute_ld(&mut self, target: crate::instructions::LoadTarget, source: crate::instructions::LoadSource) -> Result<u16, String> {
        let value = self.get_load_source_value(source)?;
        let reg = self.load_target_to_register(target)?;
        self.registers.set_register(reg, value);
        Ok(self.pc + 1)
    }

    /// 执行LD16指令
    fn execute_ld16(&mut self, target: crate::instructions::LoadTarget16, source: crate::instructions::LoadSource16) -> Result<u16, String> {
        let value = self.get_load_source16_value(source)?;
        self.set_load_target16_value(target, value)?;
        Ok(self.pc + 1)
    }

    /// 执行INC16指令
    fn execute_inc16(&mut self, target: crate::instructions::LoadTarget16) -> Result<u16, String> {
        let current_value = self.get_load_target16_value(target)?;
        let new_value = current_value.wrapping_add(1);
        self.set_load_target16_value(target, new_value)?;
        Ok(self.pc + 1)
    }

    /// 执行DEC16指令
    fn execute_dec16(&mut self, target: crate::instructions::LoadTarget16) -> Result<u16, String> {
        let current_value = self.get_load_target16_value(target)?;
        let new_value = current_value.wrapping_sub(1);
        self.set_load_target16_value(target, new_value)?;
        Ok(self.pc + 1)
    }

    /// 执行JP指令
    fn execute_jp(&mut self, target: crate::instructions::JumpTarget) -> Result<u16, String> {
        match target {
            crate::instructions::JumpTarget::Immediate(address) => Ok(address),
            crate::instructions::JumpTarget::Relative(_) => Err("JP指令不支持相对跳转".to_string()),
        }
    }

    /// 执行JR指令
    fn execute_jr(&mut self, target: crate::instructions::JumpTarget) -> Result<u16, String> {
        match target {
            crate::instructions::JumpTarget::Relative(offset) => {
                let new_pc = (self.pc as i32 + offset as i32) as u16;
                Ok(new_pc)
            }
            crate::instructions::JumpTarget::Immediate(_) => Err("JR指令不支持绝对跳转".to_string()),
        }
    }

    // 辅助方法
    fn get_register_value(&self, target: crate::instructions::ArithmeticTarget) -> Result<u8, String> {
        let reg = self.arithmetic_target_to_register(target)?;
        Ok(self.registers.get_register(reg))
    }

    fn arithmetic_target_to_register(&self, target: crate::instructions::ArithmeticTarget) -> Result<Register, String> {
        match target {
            crate::instructions::ArithmeticTarget::A => Ok(Register::A),
            crate::instructions::ArithmeticTarget::B => Ok(Register::B),
            crate::instructions::ArithmeticTarget::C => Ok(Register::C),
            crate::instructions::ArithmeticTarget::D => Ok(Register::D),
            crate::instructions::ArithmeticTarget::E => Ok(Register::E),
            crate::instructions::ArithmeticTarget::H => Ok(Register::H),
            crate::instructions::ArithmeticTarget::L => Ok(Register::L),
        }
    }

    fn load_target_to_register(&self, target: crate::instructions::LoadTarget) -> Result<Register, String> {
        match target {
            crate::instructions::LoadTarget::A => Ok(Register::A),
            crate::instructions::LoadTarget::B => Ok(Register::B),
            crate::instructions::LoadTarget::C => Ok(Register::C),
            crate::instructions::LoadTarget::D => Ok(Register::D),
            crate::instructions::LoadTarget::E => Ok(Register::E),
            crate::instructions::LoadTarget::H => Ok(Register::H),
            crate::instructions::LoadTarget::L => Ok(Register::L),
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

    // 算术运算方法
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let half_carry = ((self.registers.a & 0xF) + (value & 0xF)) > 0xF;
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = false;
        self.flags.half_carry = half_carry;
        self.flags.carry = did_overflow;
        
        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let half_carry = (self.registers.a & 0xF) < (value & 0xF);
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = true;
        self.flags.half_carry = half_carry;
        self.flags.carry = did_overflow;
        
        new_value
    }

    fn inc(&mut self, value: u8) -> u8 {
        let (new_value, _did_overflow) = value.overflowing_add(1);
        let half_carry = (value & 0xF) == 0xF;
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = false;
        self.flags.half_carry = half_carry;
        // INC指令不影响进位标志
        
        new_value
    }

    fn dec(&mut self, value: u8) -> u8 {
        let (new_value, _did_overflow) = value.overflowing_sub(1);
        let half_carry = (value & 0xF) == 0;
        
        self.flags.zero = new_value == 0;
        self.flags.subtract = true;
        self.flags.half_carry = half_carry;
        // DEC指令不影响进位标志
        
        new_value
    }
}
