//! 指令模块 - 包含所有指令定义和解码

pub mod instruction;
pub mod arithmetic;
pub mod load;
pub mod jump;

pub use instruction::Instruction;
pub use arithmetic::ArithmeticTarget;
pub use load::{LoadTarget, LoadSource, LoadTarget16, LoadSource16};
pub use jump::JumpTarget;
