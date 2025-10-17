//! 熵源实现模块
//! 
//! 实现了多种外部熵源，包括系统时间、硬件随机数、网络熵等

use std::time::{SystemTime, UNIX_EPOCH};
use std::process;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// 熵源特征
pub trait EntropySource: Send + Sync {
    /// 收集熵数据
    fn collect_entropy(&mut self) -> Result<Vec<u8>, EntropyError>;
    
    /// 获取熵源类型
    fn get_type(&self) -> EntropySourceType;
    
    /// 获取熵源质量评分 (0.0-1.0)
    fn get_quality(&self) -> f64;
    
    /// 检查熵源是否可用
    fn is_available(&self) -> bool;
}

/// 熵源类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum EntropySourceType {
    SystemTime,
    Hardware,
    Network,
    Process,
    Memory,
    Quantum,
    Custom(String),
}

/// 熵收集器
pub struct EntropyCollector {
    sources: Vec<Box<dyn EntropySource>>,
    entropy_buffer: Arc<Mutex<VecDeque<u8>>>,
}

impl EntropyCollector {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            entropy_buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    pub fn add_source(&mut self, source: Box<dyn EntropySource>) {
        self.sources.push(source);
    }
    
    pub fn collect_all(&mut self) -> Result<Vec<u8>, EntropyError> {
        let mut total_entropy = Vec::new();
        
        for source in &mut self.sources {
            if source.is_available() {
                match source.collect_entropy() {
                    Ok(entropy) => total_entropy.extend(entropy),
                    Err(e) => eprintln!("熵源 {} 收集失败: {}", 
                        match source.get_type() {
                            EntropySourceType::SystemTime => "系统时间",
                            EntropySourceType::Hardware => "硬件",
                            EntropySourceType::Network => "网络",
                            EntropySourceType::Process => "进程",
                            EntropySourceType::Memory => "内存",
                            EntropySourceType::Quantum => "量子",
                            EntropySourceType::Custom(ref name) => name,
                        }, e),
                }
            }
        }
        
        if total_entropy.is_empty() {
            return Err(EntropyError::InsufficientEntropy);
        }
        
        Ok(total_entropy)
    }
}

/// 系统时间熵源
pub struct SystemTimeEntropy {
    last_collection: u64,
    counter: u64,
}

impl SystemTimeEntropy {
    pub fn new() -> Self {
        Self {
            last_collection: 0,
            counter: 0,
        }
    }
}

impl EntropySource for SystemTimeEntropy {
    fn collect_entropy(&mut self) -> Result<Vec<u8>, EntropyError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| EntropyError::SourceUnavailable(format!("时间获取失败: {}", e)))?
            .as_nanos() as u64;
        
        self.counter += 1;
        
        // 使用时间差和计数器生成熵
        let time_diff = now.wrapping_sub(self.last_collection);
        self.last_collection = now;
        
        let mut entropy = Vec::new();
        entropy.extend_from_slice(&time_diff.to_le_bytes());
        entropy.extend_from_slice(&self.counter.to_le_bytes());
        entropy.extend_from_slice(&now.to_le_bytes());
        
        // 添加微秒级精度
        let micros = (now % 1_000_000) as u32;
        entropy.extend_from_slice(&micros.to_le_bytes());
        
        Ok(entropy)
    }
    
    fn get_type(&self) -> EntropySourceType {
        EntropySourceType::SystemTime
    }
    
    fn get_quality(&self) -> f64 {
        0.7 // 系统时间熵质量中等
    }
    
    fn is_available(&self) -> bool {
        true
    }
}

/// 硬件熵源（模拟）
pub struct HardwareEntropy {
    seed: u64,
    lfsr_state: u32,
}

impl HardwareEntropy {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Self {
            seed,
            lfsr_state: (seed & 0xFFFFFFFF) as u32,
        }
    }
    
    fn lfsr_next(&mut self) -> u8 {
        // 线性反馈移位寄存器
        let bit = ((self.lfsr_state >> 0) ^ (self.lfsr_state >> 2) ^ 
                   (self.lfsr_state >> 3) ^ (self.lfsr_state >> 5)) & 1;
        self.lfsr_state = (self.lfsr_state >> 1) | (bit << 31);
        bit as u8
    }
}

impl EntropySource for HardwareEntropy {
    fn collect_entropy(&mut self) -> Result<Vec<u8>, EntropyError> {
        let mut entropy = Vec::new();
        
        // 生成32字节的硬件熵
        for _ in 0..32 {
            let mut byte = 0u8;
            for bit_pos in 0..8 {
                byte |= self.lfsr_next() << bit_pos;
            }
            entropy.push(byte);
        }
        
        // 添加种子信息
        entropy.extend_from_slice(&self.seed.to_le_bytes());
        
        Ok(entropy)
    }
    
    fn get_type(&self) -> EntropySourceType {
        EntropySourceType::Hardware
    }
    
    fn get_quality(&self) -> f64 {
        0.9 // 硬件熵质量较高
    }
    
    fn is_available(&self) -> bool {
        true
    }
}

/// 网络熵源（模拟网络延迟和包计数）
pub struct NetworkEntropy {
    packet_counter: u64,
    last_packet_time: u64,
}

impl NetworkEntropy {
    pub fn new() -> Self {
        Self {
            packet_counter: 0,
            last_packet_time: 0,
        }
    }
}

impl EntropySource for NetworkEntropy {
    fn collect_entropy(&mut self) -> Result<Vec<u8>, EntropyError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| EntropyError::SourceUnavailable(format!("网络时间获取失败: {}", e)))?
            .as_nanos() as u64;
        
        self.packet_counter += 1;
        
        // 模拟网络延迟变化
        let delay = if self.last_packet_time > 0 {
            now - self.last_packet_time
        } else {
            0
        };
        
        self.last_packet_time = now;
        
        let mut entropy = Vec::new();
        entropy.extend_from_slice(&self.packet_counter.to_le_bytes());
        entropy.extend_from_slice(&delay.to_le_bytes());
        entropy.extend_from_slice(&now.to_le_bytes());
        
        // 模拟网络接口统计
        let interface_stats = [
            (self.packet_counter % 1000) as u16,
            (delay % 10000) as u16,
            (now % 65536) as u16,
        ];
        
        for stat in &interface_stats {
            entropy.extend_from_slice(&stat.to_le_bytes());
        }
        
        Ok(entropy)
    }
    
    fn get_type(&self) -> EntropySourceType {
        EntropySourceType::Network
    }
    
    fn get_quality(&self) -> f64 {
        0.8 // 网络熵质量较高
    }
    
    fn is_available(&self) -> bool {
        true
    }
}

/// 进程熵源
pub struct ProcessEntropy {
    pid: u32,
    thread_id: u64,
    memory_usage: u64,
}

impl ProcessEntropy {
    pub fn new() -> Self {
        Self {
            pid: process::id(),
            thread_id: 0, // 简化实现，避免使用不稳定的API
            memory_usage: 0,
        }
    }
}

impl EntropySource for ProcessEntropy {
    fn collect_entropy(&mut self) -> Result<Vec<u8>, EntropyError> {
        // 更新内存使用情况（模拟）
        self.memory_usage = self.memory_usage.wrapping_add(1);
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| EntropyError::SourceUnavailable(format!("进程时间获取失败: {}", e)))?
            .as_nanos() as u64;
        
        let mut entropy = Vec::new();
        entropy.extend_from_slice(&self.pid.to_le_bytes());
        entropy.extend_from_slice(&self.thread_id.to_le_bytes());
        entropy.extend_from_slice(&self.memory_usage.to_le_bytes());
        entropy.extend_from_slice(&now.to_le_bytes());
        
        // 添加进程状态信息
        let process_stats = [
            self.pid as u16,
            (self.thread_id % 65536) as u16,
            (self.memory_usage % 65536) as u16,
        ];
        
        for stat in &process_stats {
            entropy.extend_from_slice(&stat.to_le_bytes());
        }
        
        Ok(entropy)
    }
    
    fn get_type(&self) -> EntropySourceType {
        EntropySourceType::Process
    }
    
    fn get_quality(&self) -> f64 {
        0.6 // 进程熵质量中等
    }
    
    fn is_available(&self) -> bool {
        true
    }
}

/// 内存熵源
pub struct MemoryEntropy {
    memory_addresses: Vec<usize>,
    access_pattern: Vec<u8>,
}

impl MemoryEntropy {
    pub fn new() -> Self {
        Self {
            memory_addresses: vec![0x1000, 0x2000, 0x3000, 0x4000],
            access_pattern: Vec::new(),
        }
    }
}

impl EntropySource for MemoryEntropy {
    fn collect_entropy(&mut self) -> Result<Vec<u8>, EntropyError> {
        let mut entropy = Vec::new();
        
        // 模拟内存访问模式
        for addr in &self.memory_addresses {
            let value = *addr as u64;
            entropy.extend_from_slice(&value.to_le_bytes());
            
            // 模拟内存内容变化
            let content = (value ^ 0xDEADBEEF) as u32;
            entropy.extend_from_slice(&content.to_le_bytes());
        }
        
        // 添加访问模式
        self.access_pattern.push((entropy.len() % 256) as u8);
        if self.access_pattern.len() > 16 {
            self.access_pattern.remove(0);
        }
        
        entropy.extend(&self.access_pattern);
        
        Ok(entropy)
    }
    
    fn get_type(&self) -> EntropySourceType {
        EntropySourceType::Memory
    }
    
    fn get_quality(&self) -> f64 {
        0.5 // 内存熵质量较低
    }
    
    fn is_available(&self) -> bool {
        true
    }
}

// 导入错误类型
use crate::entropy::EntropyError;
