//! 寄存器模块 - 包含8位和16位寄存器操作

/// 8位寄存器结构
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    /// 创建新的寄存器实例
    pub fn new() -> Self {
        Self {
            a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0
        }
    }

    /// 获取BC寄存器对的值
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    /// 设置BC寄存器对的值
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    /// 获取DE寄存器对的值
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    /// 设置DE寄存器对的值
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    /// 获取HL寄存器对的值
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    /// 设置HL寄存器对的值
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    /// 获取指定寄存器的值
    pub fn get_register(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    /// 设置指定寄存器的值
    pub fn set_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::F => self.f = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }
}

/// 8位寄存器枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    A, B, C, D, E, F, H, L,
}

/// 16位寄存器对枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterPair {
    BC, DE, HL, SP,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_creation() {
        let regs = Registers::new();
        assert_eq!(regs.a, 0);
        assert_eq!(regs.b, 0);
        assert_eq!(regs.c, 0);
    }

    #[test]
    fn test_bc_register_pair() {
        let mut regs = Registers::new();
        regs.set_bc(0x1234);
        assert_eq!(regs.get_bc(), 0x1234);
        assert_eq!(regs.b, 0x12);
        assert_eq!(regs.c, 0x34);
    }

    #[test]
    fn test_de_register_pair() {
        let mut regs = Registers::new();
        regs.set_de(0x5678);
        assert_eq!(regs.get_de(), 0x5678);
        assert_eq!(regs.d, 0x56);
        assert_eq!(regs.e, 0x78);
    }

    #[test]
    fn test_hl_register_pair() {
        let mut regs = Registers::new();
        regs.set_hl(0x9ABC);
        assert_eq!(regs.get_hl(), 0x9ABC);
        assert_eq!(regs.h, 0x9A);
        assert_eq!(regs.l, 0xBC);
    }

    #[test]
    fn test_register_access() {
        let mut regs = Registers::new();
        regs.set_register(Register::A, 0x42);
        regs.set_register(Register::B, 0x84);
        assert_eq!(regs.get_register(Register::A), 0x42);
        assert_eq!(regs.get_register(Register::B), 0x84);
    }
}