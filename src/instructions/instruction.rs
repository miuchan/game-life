//! 指令定义模块

use super::{ArithmeticTarget, LoadTarget, LoadSource, LoadTarget16, LoadSource16, JumpTarget};

/// 指令枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    // 算术指令
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    
    // 数据传输指令
    LD(LoadTarget, LoadSource),
    LD16(LoadTarget16, LoadSource16),
    
    // 16位操作指令
    INC16(LoadTarget16),
    DEC16(LoadTarget16),
    
    // 控制流指令
    JP(JumpTarget),
    JR(JumpTarget),
    
    // 其他指令
    NOP,
}

impl Instruction {
    /// 从字节解码指令
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            // 算术指令
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            
            // 数据传输指令
            0x79 => Some(Instruction::LD(LoadTarget::A, LoadSource::C)),
            0x41 => Some(Instruction::LD(LoadTarget::B, LoadSource::C)),
            0x02 => Some(Instruction::LD(LoadTarget::A, LoadSource::B)), // LD (BC), A
            0x12 => Some(Instruction::LD(LoadTarget::A, LoadSource::D)), // LD (DE), A
            0x0A => Some(Instruction::LD(LoadTarget::A, LoadSource::B)), // LD A, (BC)
            0x1A => Some(Instruction::LD(LoadTarget::A, LoadSource::D)), // LD A, (DE)
            
            // 甜甜的生命游戏专用指令
            0x3E => Some(Instruction::LD(LoadTarget::A, LoadSource::Immediate(0x00))), // LD A, immediate
            0x06 => Some(Instruction::LD(LoadTarget::B, LoadSource::Immediate(0x10))), // LD B, immediate
            0x0E => Some(Instruction::LD(LoadTarget::C, LoadSource::Immediate(0x10))), // LD C, immediate
            0x21 => Some(Instruction::LD16(LoadTarget16::HL, LoadSource16::Immediate(0x0280))), // LD HL, immediate
            0x7E => Some(Instruction::LD(LoadTarget::A, LoadSource::H)), // LD A, (HL)
            0x23 => Some(Instruction::INC16(LoadTarget16::HL)), // INC HL
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)), // ADD A, B
            0xFE => Some(Instruction::SUB(ArithmeticTarget::A)), // CP (比较指令，用SUB模拟)
            0x28 => Some(Instruction::JR(JumpTarget::Relative(5))), // JR Z, relative
            0x77 => Some(Instruction::LD(LoadTarget::H, LoadSource::A)), // LD (HL), A
            
            // 16位操作指令
            0x01 => Some(Instruction::LD16(LoadTarget16::BC, LoadSource16::Immediate(0x1234))),
            0x11 => Some(Instruction::LD16(LoadTarget16::DE, LoadSource16::Immediate(0x5678))),
            0x03 => Some(Instruction::INC16(LoadTarget16::BC)),
            0x13 => Some(Instruction::INC16(LoadTarget16::DE)),
            0x0B => Some(Instruction::DEC16(LoadTarget16::BC)),
            0x1B => Some(Instruction::DEC16(LoadTarget16::DE)),
            
            // 控制流指令
            0xC3 => Some(Instruction::JP(JumpTarget::Immediate(0x200))),
            0x18 => Some(Instruction::JR(JumpTarget::Relative(5))),
            
            // 其他指令
            0x00 => Some(Instruction::NOP),
            
            _ => None,
        }
    }

    /// 获取指令名称
    pub fn name(&self) -> &'static str {
        match self {
            Instruction::ADD(_) => "ADD",
            Instruction::SUB(_) => "SUB",
            Instruction::INC(_) => "INC",
            Instruction::DEC(_) => "DEC",
            Instruction::LD(_, _) => "LD",
            Instruction::LD16(_, _) => "LD16",
            Instruction::INC16(_) => "INC16",
            Instruction::DEC16(_) => "DEC16",
            Instruction::JP(_) => "JP",
            Instruction::JR(_) => "JR",
            Instruction::NOP => "NOP",
        }
    }
}
