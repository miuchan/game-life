//! 算术指令目标枚举

/// 算术指令目标寄存器
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
