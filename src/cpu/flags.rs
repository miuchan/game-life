//! 标志寄存器模块 - 包含CPU标志位处理

/// 标志寄存器结构
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagsRegister {
    /// 创建新的标志寄存器实例
    pub fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }

    /// 重置所有标志位
    pub fn reset(&mut self) {
        self.zero = false;
        self.subtract = false;
        self.half_carry = false;
        self.carry = false;
    }

    /// 设置零标志
    pub fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }

    /// 设置减法标志
    pub fn set_subtract(&mut self, value: bool) {
        self.subtract = value;
    }

    /// 设置半进位标志
    pub fn set_half_carry(&mut self, value: bool) {
        self.half_carry = value;
    }

    /// 设置进位标志
    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
}

/// 标志位在字节中的位置
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<FlagsRegister> for u8 {
    fn from(flags: FlagsRegister) -> u8 {
        (if flags.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flags.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flags.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flags.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        Self {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

impl Default for FlagsRegister {
    fn default() -> Self {
        Self::new()
    }
}
