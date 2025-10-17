//! 演示程序模块
//! 
//! 包含各种演示程序，展示系统的不同功能

pub mod advanced_demo;
pub mod entropy_demo;
pub mod gba_demo;
// 这些模块暂时注释掉，因为文件还没有移动
// pub mod nintendo_fixed_point;
// pub mod ping_pong_automaton;
pub mod quantum_resistant_demo;
pub mod rom_generator;
// pub mod spacetime_entanglement;

// Re-export main types
pub use advanced_demo::*;
pub use entropy_demo::*;
pub use gba_demo::*;
// pub use nintendo_fixed_point::*;
// pub use ping_pong_automaton::*;
pub use quantum_resistant_demo::*;
pub use rom_generator::*;
// pub use spacetime_entanglement::*;
