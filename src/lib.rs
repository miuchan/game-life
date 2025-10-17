//! Game Boy Emulator
//! 
//! A comprehensive Game Boy emulator written in Rust, featuring:
//! - Complete CPU emulation
//! - Memory management
//! - GPU rendering
//! - Instruction set implementation
//! - Debug capabilities
//! - ROM loading and execution
//! - GBA support
//! - Advanced entropy system with quantum resistance
//! - Multiple game implementations
//! - Unified configuration and error handling

// Core modules
pub mod core {
    pub mod cpu;
    pub mod memory;
    pub mod gpu;
    pub mod instructions;
}

// Game modules
pub mod games {
    pub mod life_game;
    pub mod tic_tac_toe;
    pub mod tetris;
    pub mod demos;
}

// Library modules
pub mod lib {
    pub mod common;
    pub mod config;
    pub mod error;
}

// Legacy modules (for backward compatibility)
pub mod cpu {
    pub use crate::core::cpu::*;
}
pub mod memory {
    pub use crate::core::memory::*;
}
pub mod gpu {
    pub use crate::core::gpu::*;
}
pub mod instructions {
    pub use crate::core::instructions::*;
}
pub mod emulator;
pub mod rom;
pub mod debug;
pub mod gba;
pub mod entropy;

// Re-export main types
pub use emulator::{GameBoy, AdvancedGameBoy};
pub use rom::RomGenerator;
pub use entropy::{EntropyManager, EntropyError, EntropyStats};

// Re-export library modules
pub use lib::common::*;
pub use lib::config::{Config, ConfigError};
pub use lib::error::{Error, Result};
