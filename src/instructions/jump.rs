//! 跳转指令相关枚举

/// 跳转目标
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JumpTarget {
    Immediate(u16),
    Relative(i8),
}
