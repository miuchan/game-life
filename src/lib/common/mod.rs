//! 通用工具和常量
//! 
//! 提供项目中通用的工具函数、常量和类型定义

use std::time::{Duration, Instant};

/// 通用常量
pub mod constants {
    /// CPU频率 (Hz)
    pub const CPU_FREQUENCY: u32 = 4_194_304;
    
    /// 屏幕尺寸
    pub const SCREEN_WIDTH: u32 = 160;
    pub const SCREEN_HEIGHT: u32 = 144;
    
    /// 内存大小
    pub const MEMORY_SIZE: usize = 0x10000; // 64KB
    
    /// 寄存器数量
    pub const REGISTER_COUNT: usize = 8;
    
    /// 最大指令长度
    pub const MAX_INSTRUCTION_LENGTH: usize = 3;
    
    /// 熵源系统常量
    pub const ENTROPY_POOL_SIZE: usize = 1024;
    pub const QUANTUM_STATES_COUNT: usize = 256;
    pub const DISTRIBUTION_QUALITY_THRESHOLD: f64 = 0.5;
}

/// 性能监控工具
pub struct PerformanceMonitor {
    start_time: Instant,
    measurements: Vec<(String, Duration)>,
}

impl PerformanceMonitor {
    /// 创建新的性能监控器
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            measurements: Vec::new(),
        }
    }
    
    /// 记录测量点
    pub fn mark(&mut self, name: &str) {
        let elapsed = self.start_time.elapsed();
        self.measurements.push((name.to_string(), elapsed));
    }
    
    /// 获取所有测量结果
    pub fn get_measurements(&self) -> &[(String, Duration)] {
        &self.measurements
    }
    
    /// 获取总运行时间
    pub fn total_time(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// 打印性能报告
    pub fn print_report(&self) {
        println!("Performance Report:");
        println!("Total time: {:?}", self.total_time());
        
        for (name, duration) in &self.measurements {
            println!("  {}: {:?}", name, duration);
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// 位操作工具
pub mod bit_ops {
    /// 检查位是否设置
    pub fn is_bit_set(value: u8, bit: u8) -> bool {
        (value & (1 << bit)) != 0
    }
    
    /// 设置位
    pub fn set_bit(value: u8, bit: u8) -> u8 {
        value | (1 << bit)
    }
    
    /// 清除位
    pub fn clear_bit(value: u8, bit: u8) -> u8 {
        value & !(1 << bit)
    }
    
    /// 切换位
    pub fn toggle_bit(value: u8, bit: u8) -> u8 {
        value ^ (1 << bit)
    }
    
    /// 获取位的值
    pub fn get_bit(value: u8, bit: u8) -> u8 {
        (value >> bit) & 1
    }
    
    /// 设置多个位
    pub fn set_bits(value: u8, mask: u8) -> u8 {
        value | mask
    }
    
    /// 清除多个位
    pub fn clear_bits(value: u8, mask: u8) -> u8 {
        value & !mask
    }
    
    /// 检查是否有任何位设置
    pub fn has_any_bit(value: u8, mask: u8) -> bool {
        (value & mask) != 0
    }
    
    /// 检查是否所有位都设置
    pub fn has_all_bits(value: u8, mask: u8) -> bool {
        (value & mask) == mask
    }
}

/// 数学工具
pub mod math {
    /// 计算两个数的最大公约数
    pub fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    
    /// 计算两个数的最小公倍数
    pub fn lcm(a: u32, b: u32) -> u32 {
        a * b / gcd(a, b)
    }
    
    /// 将值限制在指定范围内
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    
    /// 线性插值
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
    
    /// 将角度转换为弧度
    pub fn deg_to_rad(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }
    
    /// 将弧度转换为角度
    pub fn rad_to_deg(radians: f32) -> f32 {
        radians * 180.0 / std::f32::consts::PI
    }
}

/// 字符串工具
pub mod string {
    /// 移除字符串前后的空白字符
    pub fn trim(s: &str) -> &str {
        s.trim()
    }
    
    /// 检查字符串是否为空或只包含空白字符
    pub fn is_empty_or_whitespace(s: &str) -> bool {
        s.trim().is_empty()
    }
    
    /// 将字符串转换为标题格式
    pub fn to_title_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize = true;
        
        for c in s.chars() {
            if c.is_whitespace() {
                capitalize = true;
                result.push(c);
            } else if capitalize {
                result.push(c.to_uppercase().next().unwrap());
                capitalize = false;
            } else {
                result.push(c.to_lowercase().next().unwrap());
            }
        }
        
        result
    }
    
    /// 检查字符串是否以指定前缀开始
    pub fn starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }
    
    /// 检查字符串是否以指定后缀结束
    pub fn ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }
}

/// 集合工具
pub mod collections {
    use std::collections::HashMap;
    
    /// 创建频率映射
    pub fn frequency_map<T: std::hash::Hash + Eq>(items: &[T]) -> HashMap<&T, usize> {
        let mut map = HashMap::new();
        for item in items {
            *map.entry(item).or_insert(0) += 1;
        }
        map
    }
    
    /// 获取最常见的元素
    pub fn most_common<T: std::hash::Hash + Eq + Clone>(items: &[T]) -> Option<T> {
        let freq_map = frequency_map(items);
        freq_map.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(item, _)| (*item).clone())
    }
    
    /// 检查两个集合是否有交集
    pub fn has_intersection<T: std::hash::Hash + Eq>(a: &[T], b: &[T]) -> bool {
        let set_a: std::collections::HashSet<_> = a.iter().collect();
        let set_b: std::collections::HashSet<_> = b.iter().collect();
        !set_a.is_disjoint(&set_b)
    }
}

/// 类型别名
pub type Byte = u8;
pub type Word = u16;
pub type DWord = u32;
pub type Address = u16;
pub type Register = u8;

/// 通用结果类型
pub type GameResult<T> = Result<T, crate::lib::error::Error>;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// 获取日志级别的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
    
    /// 从字符串创建日志级别
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            _ => None,
        }
    }
}

/// 简单的日志记录器
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    /// 创建新的日志记录器
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }
    
    /// 记录日志
    pub fn log(&self, level: LogLevel, message: &str) {
        if level >= self.level {
            println!("[{}] {}", level.as_str(), message);
        }
    }
    
    /// 记录跟踪日志
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    
    /// 记录调试日志
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    
    /// 记录信息日志
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    
    /// 记录警告日志
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    
    /// 记录错误日志
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(LogLevel::Info)
    }
}
