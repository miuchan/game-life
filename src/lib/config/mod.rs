//! 通用配置管理
//! 
//! 提供统一的配置管理接口，支持从文件、环境变量和默认值加载配置

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 配置管理器
#[derive(Debug, Clone)]
pub struct Config {
    values: HashMap<String, String>,
}

impl Config {
    /// 创建新的配置实例
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// 从文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::FileRead(e))?;
        
        let mut config = Self::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                config.set(key.trim(), value.trim());
            }
        }
        
        Ok(config)
    }
    
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let mut config = Self::new();
        
        // 加载所有以 GAMEBOY_ 开头的环境变量
        for (key, value) in std::env::vars() {
            if key.starts_with("GAMEBOY_") {
                let config_key = key.strip_prefix("GAMEBOY_").unwrap().to_lowercase();
                config.set(&config_key, &value);
            }
        }
        
        config
    }
    
    /// 设置配置值
    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }
    
    /// 获取配置值
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
    
    /// 获取配置值，如果不存在则返回默认值
    pub fn get_or_default(&self, key: &str, default: &str) -> String {
        self.get(key).map(|s| s.clone()).unwrap_or_else(|| default.to_string())
    }
    
    /// 获取布尔值配置
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.parse().ok())
    }
    
    /// 获取整数配置
    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.get(key).and_then(|v| v.parse().ok())
    }
    
    /// 获取浮点数配置
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.parse().ok())
    }
    
    /// 合并另一个配置
    pub fn merge(&mut self, other: &Config) {
        for (key, value) in &other.values {
            self.values.insert(key.clone(), value.clone());
        }
    }
    
    /// 获取所有配置键
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }
    
    /// 检查是否包含某个键
    pub fn contains_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// 配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    FileRead(std::io::Error),
    ParseError(String),
    MissingKey(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileRead(e) => write!(f, "Failed to read config file: {}", e),
            ConfigError::ParseError(msg) => write!(f, "Config parse error: {}", msg),
            ConfigError::MissingKey(key) => write!(f, "Missing required config key: {}", key),
        }
    }
}

impl std::error::Error for ConfigError {}

/// 默认配置常量
pub mod defaults {
    pub const CPU_FREQUENCY: u32 = 4194304; // 4.194304 MHz
    pub const SCREEN_WIDTH: u32 = 160;
    pub const SCREEN_HEIGHT: u32 = 144;
    pub const MEMORY_SIZE: usize = 0x10000; // 64KB
    pub const ENTROPY_POOL_SIZE: usize = 1024;
    pub const QUANTUM_STATES_COUNT: usize = 256;
    pub const DISTRIBUTION_QUALITY_THRESHOLD: f64 = 0.5;
}

/// 配置键常量
pub mod keys {
    pub const CPU_FREQUENCY: &str = "cpu_frequency";
    pub const SCREEN_WIDTH: &str = "screen_width";
    pub const SCREEN_HEIGHT: &str = "screen_height";
    pub const MEMORY_SIZE: &str = "memory_size";
    pub const ENTROPY_POOL_SIZE: &str = "entropy_pool_size";
    pub const QUANTUM_STATES_COUNT: &str = "quantum_states_count";
    pub const DISTRIBUTION_QUALITY_THRESHOLD: &str = "distribution_quality_threshold";
    pub const DEBUG_MODE: &str = "debug_mode";
    pub const LOG_LEVEL: &str = "log_level";
    pub const ROM_PATH: &str = "rom_path";
    pub const SAVE_PATH: &str = "save_path";
}
