//! 生命游戏模块
//! 
//! 包含各种生命游戏实现，包括经典版本和优化版本

pub mod new_life_game;
pub mod sweet_life_game;
pub mod sweet_life_optimized;

// Re-export main types
pub use new_life_game::*;
pub use sweet_life_game::*;
pub use sweet_life_optimized::*;
