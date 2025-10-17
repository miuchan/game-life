//! 加载指令相关枚举

/// 8位加载目标寄存器
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadTarget {
    A, B, C, D, E, H, L,
}

/// 8位加载源
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadSource {
    A, B, C, D, E, H, L,
    Immediate(u8),
}

/// 16位加载目标寄存器对
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadTarget16 {
    BC, DE, HL, SP,
}

/// 16位加载源
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadSource16 {
    BC, DE, HL, SP,
    Immediate(u16),
}
