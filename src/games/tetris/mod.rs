//! 俄罗斯方块游戏模块
//! 
//! 基于GBA模拟器实现的Windows俄罗斯方块游戏

pub mod tetris_game;
pub mod tetris_gba;
pub mod windows_tetris;

// Re-export main types
pub use tetris_game::*;
pub use tetris_gba::*;
pub use windows_tetris::*;
