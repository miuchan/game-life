//! 统一错误处理系统
//! 
//! 提供统一的错误类型和处理机制，支持错误链和上下文信息

use std::fmt;
use std::error::Error as StdError;

/// 主错误类型
#[derive(Debug)]
pub enum Error {
    /// CPU相关错误
    Cpu(CpuError),
    /// 内存相关错误
    Memory(MemoryError),
    /// GPU相关错误
    Gpu(GpuError),
    /// 指令相关错误
    Instruction(InstructionError),
    /// 熵源系统错误
    Entropy(EntropyError),
    /// 游戏相关错误
    Game(GameError),
    /// 配置错误
    Config(ConfigError),
    /// IO错误
    Io(std::io::Error),
    /// 通用错误
    Generic(String),
}

/// CPU错误类型
#[derive(Debug)]
pub enum CpuError {
    InvalidRegister,
    InvalidInstruction,
    StackOverflow,
    StackUnderflow,
    DivisionByZero,
    InvalidAddress(u16),
}

/// 内存错误类型
#[derive(Debug)]
pub enum MemoryError {
    InvalidAddress(u16),
    ReadOnlyMemory(u16),
    OutOfBounds(u16),
    InvalidBank(u8),
}

/// GPU错误类型
#[derive(Debug)]
pub enum GpuError {
    InvalidMode(u8),
    InvalidTile(u8),
    InvalidPalette(u8),
    ScanlineError(u8),
}

/// 指令错误类型
#[derive(Debug)]
pub enum InstructionError {
    UnknownOpcode(u8),
    InvalidOperand,
    UnimplementedInstruction(String),
}

/// 熵源系统错误类型
#[derive(Debug)]
pub enum EntropyError {
    InsufficientEntropy,
    InvalidSource,
    PoolOverflow,
    DistributionError(String),
    QuantumError(String),
}

/// 游戏错误类型
#[derive(Debug)]
pub enum GameError {
    InvalidMove,
    GameOver,
    InvalidState,
    LoadError(String),
    SaveError(String),
}

/// 配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    FileRead(std::io::Error),
    ParseError(String),
    MissingKey(String),
}

// 实现 Display trait
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Cpu(e) => write!(f, "CPU Error: {}", e),
            Error::Memory(e) => write!(f, "Memory Error: {}", e),
            Error::Gpu(e) => write!(f, "GPU Error: {}", e),
            Error::Instruction(e) => write!(f, "Instruction Error: {}", e),
            Error::Entropy(e) => write!(f, "Entropy Error: {}", e),
            Error::Game(e) => write!(f, "Game Error: {}", e),
            Error::Config(e) => write!(f, "Config Error: {}", e),
            Error::Io(e) => write!(f, "IO Error: {}", e),
            Error::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuError::InvalidRegister => write!(f, "Invalid register"),
            CpuError::InvalidInstruction => write!(f, "Invalid instruction"),
            CpuError::StackOverflow => write!(f, "Stack overflow"),
            CpuError::StackUnderflow => write!(f, "Stack underflow"),
            CpuError::DivisionByZero => write!(f, "Division by zero"),
            CpuError::InvalidAddress(addr) => write!(f, "Invalid address: 0x{:04X}", addr),
        }
    }
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::InvalidAddress(addr) => write!(f, "Invalid address: 0x{:04X}", addr),
            MemoryError::ReadOnlyMemory(addr) => write!(f, "Read-only memory at: 0x{:04X}", addr),
            MemoryError::OutOfBounds(addr) => write!(f, "Address out of bounds: 0x{:04X}", addr),
            MemoryError::InvalidBank(bank) => write!(f, "Invalid memory bank: {}", bank),
        }
    }
}

impl fmt::Display for GpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuError::InvalidMode(mode) => write!(f, "Invalid GPU mode: {}", mode),
            GpuError::InvalidTile(tile) => write!(f, "Invalid tile: {}", tile),
            GpuError::InvalidPalette(palette) => write!(f, "Invalid palette: {}", palette),
            GpuError::ScanlineError(line) => write!(f, "Scanline error at line: {}", line),
        }
    }
}

impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionError::UnknownOpcode(opcode) => write!(f, "Unknown opcode: 0x{:02X}", opcode),
            InstructionError::InvalidOperand => write!(f, "Invalid operand"),
            InstructionError::UnimplementedInstruction(inst) => write!(f, "Unimplemented instruction: {}", inst),
        }
    }
}

impl fmt::Display for EntropyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntropyError::InsufficientEntropy => write!(f, "Insufficient entropy"),
            EntropyError::InvalidSource => write!(f, "Invalid entropy source"),
            EntropyError::PoolOverflow => write!(f, "Entropy pool overflow"),
            EntropyError::DistributionError(msg) => write!(f, "Distribution error: {}", msg),
            EntropyError::QuantumError(msg) => write!(f, "Quantum error: {}", msg),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidMove => write!(f, "Invalid move"),
            GameError::GameOver => write!(f, "Game over"),
            GameError::InvalidState => write!(f, "Invalid game state"),
            GameError::LoadError(msg) => write!(f, "Load error: {}", msg),
            GameError::SaveError(msg) => write!(f, "Save error: {}", msg),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileRead(e) => write!(f, "Failed to read config file: {}", e),
            ConfigError::ParseError(msg) => write!(f, "Config parse error: {}", msg),
            ConfigError::MissingKey(key) => write!(f, "Missing required config key: {}", key),
        }
    }
}

// 实现 Error trait
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Config(ConfigError::FileRead(e)) => Some(e),
            _ => None,
        }
    }
}

impl StdError for CpuError {}
impl StdError for MemoryError {}
impl StdError for GpuError {}
impl StdError for InstructionError {}
impl StdError for EntropyError {}
impl StdError for GameError {}
impl StdError for ConfigError {}

// 实现 From trait 用于错误转换
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<CpuError> for Error {
    fn from(e: CpuError) -> Self {
        Error::Cpu(e)
    }
}

impl From<MemoryError> for Error {
    fn from(e: MemoryError) -> Self {
        Error::Memory(e)
    }
}

impl From<GpuError> for Error {
    fn from(e: GpuError) -> Self {
        Error::Gpu(e)
    }
}

impl From<InstructionError> for Error {
    fn from(e: InstructionError) -> Self {
        Error::Instruction(e)
    }
}

impl From<EntropyError> for Error {
    fn from(e: EntropyError) -> Self {
        Error::Entropy(e)
    }
}

impl From<GameError> for Error {
    fn from(e: GameError) -> Self {
        Error::Game(e)
    }
}

impl From<ConfigError> for Error {
    fn from(e: ConfigError) -> Self {
        Error::Config(e)
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

/// 错误上下文宏
#[macro_export]
macro_rules! with_context {
    ($result:expr, $context:expr) => {
        $result.map_err(|e| Error::Generic(format!("{}: {}", $context, e)))
    };
}

/// 错误处理工具函数
pub mod utils {
    use super::*;
    
    /// 将错误转换为字符串
    pub fn error_to_string(error: &Error) -> String {
        format!("{}", error)
    }
    
    /// 获取错误链
    pub fn error_chain(error: &Error) -> Vec<String> {
        let mut chain = vec![error.to_string()];
        let mut source = error.source();
        
        while let Some(err) = source {
            chain.push(err.to_string());
            source = err.source();
        }
        
        chain
    }
    
    /// 检查是否为致命错误
    pub fn is_fatal(error: &Error) -> bool {
        matches!(error, Error::Memory(_) | Error::Cpu(_))
    }
}
