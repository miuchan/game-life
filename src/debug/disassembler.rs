//! 反汇编器模块

use crate::memory::MemoryBus;
use crate::instructions::Instruction;

/// 反汇编器
#[derive(Debug, Clone)]
pub struct Disassembler {
    pub show_addresses: bool,
    pub show_bytes: bool,
}

impl Disassembler {
    /// 创建新的反汇编器
    pub fn new() -> Self {
        Self {
            show_addresses: true,
            show_bytes: true,
        }
    }

    /// 反汇编指定地址的指令
    pub fn disassemble_at(&self, pc: u16, memory: &MemoryBus) -> String {
        let instruction_byte = memory.read_byte(pc);
        
        if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.format_instruction(pc, instruction_byte, instruction)
        } else {
            format!("0x{:04X}: 0x{:02X} ???", pc, instruction_byte)
        }
    }

    /// 反汇编内存区域
    pub fn disassemble_range(&self, start: u16, end: u16, memory: &MemoryBus) -> Vec<String> {
        let mut result = Vec::new();
        let mut pc = start;

        while pc < end {
            let instruction_byte = memory.read_byte(pc);
            
            if let Some(instruction) = Instruction::from_byte(instruction_byte) {
                result.push(self.format_instruction(pc, instruction_byte, instruction));
                pc += self.get_instruction_size(&instruction);
            } else {
                result.push(format!("0x{:04X}: 0x{:02X} ???", pc, instruction_byte));
                pc += 1;
            }
        }

        result
    }

    /// 格式化指令
    fn format_instruction(&self, pc: u16, byte: u8, instruction: Instruction) -> String {
        if self.show_addresses && self.show_bytes {
            format!("0x{:04X}: 0x{:02X} {}", pc, byte, self.instruction_to_string(instruction))
        } else if self.show_addresses {
            format!("0x{:04X}: {}", pc, self.instruction_to_string(instruction))
        } else if self.show_bytes {
            format!("0x{:02X} {}", byte, self.instruction_to_string(instruction))
        } else {
            self.instruction_to_string(instruction)
        }
    }

    /// 将指令转换为字符串
    fn instruction_to_string(&self, instruction: Instruction) -> String {
        match instruction {
            Instruction::NOP => "NOP".to_string(),
            Instruction::ADD(target) => format!("ADD A, {:?}", target),
            Instruction::SUB(target) => format!("SUB A, {:?}", target),
            Instruction::INC(target) => format!("INC {:?}", target),
            Instruction::DEC(target) => format!("DEC {:?}", target),
            Instruction::LD(target, source) => format!("LD {:?}, {:?}", target, source),
            Instruction::LD16(target, source) => format!("LD {:?}, {:?}", target, source),
            Instruction::INC16(target) => format!("INC {:?}", target),
            Instruction::DEC16(target) => format!("DEC {:?}", target),
            Instruction::JP(target) => format!("JP {:?}", target),
            Instruction::JR(target) => format!("JR {:?}", target),
        }
    }

    /// 获取指令大小
    fn get_instruction_size(&self, instruction: &Instruction) -> u16 {
        match instruction {
            Instruction::NOP => 1,
            Instruction::ADD(_) => 1,
            Instruction::SUB(_) => 1,
            Instruction::INC(_) => 1,
            Instruction::DEC(_) => 1,
            Instruction::LD(_, _) => 1,
            Instruction::LD16(_, _) => 3,
            Instruction::INC16(_) => 1,
            Instruction::DEC16(_) => 1,
            Instruction::JP(_) => 3,
            Instruction::JR(_) => 2,
        }
    }
}

impl Default for Disassembler {
    fn default() -> Self {
        Self::new()
    }
}
