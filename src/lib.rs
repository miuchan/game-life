//! Game Boy模拟器库

pub mod cpu;
pub mod memory;
pub mod instructions;
pub mod emulator;
pub mod rom;
pub mod gpu;
pub mod debug;
pub mod gba;
pub mod entropy;

pub use emulator::{GameBoy, AdvancedGameBoy};
pub use rom::RomGenerator;
pub use entropy::{EntropyManager, EntropyError, EntropyStats};
