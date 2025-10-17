//! 熵池模块
//! 
//! 实现熵池管理，用于存储和混合来自不同源的熵数据

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// 熵池
pub struct EntropyPool {
    pool: Arc<Mutex<VecDeque<u8>>>,
    max_size: usize,
    min_size: usize,
    last_refill: u64,
    refill_count: u32,
}

impl EntropyPool {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::new())),
            max_size: 1024 * 1024, // 1MB
            min_size: 1024,        // 1KB
            last_refill: 0,
            refill_count: 0,
        }
    }
    
    /// 添加熵到池中
    pub fn add_entropy(&mut self, entropy: &[u8]) {
        let mut pool = self.pool.lock().unwrap();
        
        // 添加新熵
        for &byte in entropy {
            pool.push_back(byte);
        }
        
        // 保持池大小限制
        while pool.len() > self.max_size {
            pool.pop_front();
        }
        
        // 混合池中的熵
        self.mix_pool(&mut pool);
        
        self.refill_count += 1;
        self.last_refill = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
    
    /// 从池中提取熵
    pub fn extract_entropy(&mut self, size: usize) -> Vec<u8> {
        let mut pool = self.pool.lock().unwrap();
        
        // 检查池是否有足够的熵
        let current_size = pool.len();
        if current_size < size {
            // 如果熵不足，生成一些临时熵
            self.generate_temporary_entropy(&mut pool, size - current_size);
        }
        
        // 提取熵
        let mut extracted = Vec::with_capacity(size);
        for _ in 0..size {
            if let Some(byte) = pool.pop_front() {
                extracted.push(byte);
            }
        }
        
        // 重新混合剩余的池
        self.mix_pool(&mut pool);
        
        extracted
    }
    
    /// 混合池中的熵
    fn mix_pool(&self, pool: &mut VecDeque<u8>) {
        if pool.len() < 2 {
            return;
        }
        
        // 使用Fisher-Yates洗牌算法的变种
        let len = pool.len();
        for i in (1..len).rev() {
            let j = (i * 7 + 13) % len; // 伪随机选择
            pool.swap(i, j);
        }
        
        // 应用额外的混合
        self.apply_advanced_mixing(pool);
    }
    
    /// 应用高级混合算法
    fn apply_advanced_mixing(&self, pool: &mut VecDeque<u8>) {
        if pool.len() < 4 {
            return;
        }
        
        // 使用Feistel网络混合
        let mut mixed: Vec<u8> = pool.iter().cloned().collect();
        
        for round in 0..8 {
            for i in (0..mixed.len() - 1).step_by(2) {
                let left = mixed[i];
                let right = mixed[i + 1];
                
                let new_left = right;
                let new_right = left ^ self.feistel_function(right, round);
                
                mixed[i] = new_left;
                mixed[i + 1] = new_right;
            }
        }
        
        // 更新池
        pool.clear();
        pool.extend(mixed);
    }
    
    /// Feistel轮函数
    fn feistel_function(&self, input: u8, round: u8) -> u8 {
        let mut result = input;
        result = result.wrapping_add(round);
        result = result.wrapping_mul(3);
        result = result.rotate_left(2);
        result ^= 0x55;
        result
    }
    
    /// 生成临时熵
    fn generate_temporary_entropy(&self, pool: &mut VecDeque<u8>, size: usize) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        // 使用时间戳和计数器生成临时熵
        for i in 0..size {
            let temp_entropy = (now.wrapping_add(i as u64) ^ self.refill_count as u64) as u8;
            pool.push_back(temp_entropy);
        }
    }
    
    /// 获取池大小
    pub fn size(&self) -> usize {
        self.pool.lock().unwrap().len()
    }
    
    /// 检查池是否需要补充
    pub fn needs_refill(&self) -> bool {
        self.size() < self.min_size
    }
    
    /// 获取池统计信息
    pub fn get_stats(&self) -> PoolStats {
        PoolStats {
            current_size: self.size(),
            max_size: self.max_size,
            min_size: self.min_size,
            refill_count: self.refill_count,
            last_refill: self.last_refill,
        }
    }
}

/// 池化熵
pub struct PooledEntropy {
    pool: EntropyPool,
    extraction_buffer: Vec<u8>,
    buffer_size: usize,
}

impl PooledEntropy {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            pool: EntropyPool::new(),
            extraction_buffer: Vec::new(),
            buffer_size,
        }
    }
    
    /// 添加熵源
    pub fn add_entropy_source(&mut self, entropy: &[u8]) {
        self.pool.add_entropy(entropy);
    }
    
    /// 获取随机字节
    pub fn get_random_bytes(&mut self, count: usize) -> Vec<u8> {
        // 如果缓冲区为空或不足，从池中提取
        if self.extraction_buffer.len() < count {
            let needed = count - self.extraction_buffer.len();
            let extracted = self.pool.extract_entropy(needed.max(self.buffer_size));
            self.extraction_buffer.extend(extracted);
        }
        
        // 从缓冲区提取请求的字节数
        let result: Vec<u8> = self.extraction_buffer.drain(0..count).collect();
        result
    }
    
    /// 获取单个随机字节
    pub fn get_random_byte(&mut self) -> u8 {
        let bytes = self.get_random_bytes(1);
        bytes[0]
    }
    
    /// 获取随机整数
    pub fn get_random_u32(&mut self) -> u32 {
        let bytes = self.get_random_bytes(4);
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    
    /// 获取随机整数（指定范围）
    pub fn get_random_range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        
        let range = max - min;
        let random_value = self.get_random_u32();
        min + (random_value % range)
    }
    
    /// 获取池统计信息
    pub fn get_pool_stats(&self) -> PoolStats {
        self.pool.get_stats()
    }
}

/// 池统计信息
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub current_size: usize,
    pub max_size: usize,
    pub min_size: usize,
    pub refill_count: u32,
    pub last_refill: u64,
}

/// 熵质量评估器
pub struct EntropyQualityAssessor {
    sample_count: u32,
    chi_square_sum: f64,
    entropy_sum: f64,
}

impl EntropyQualityAssessor {
    pub fn new() -> Self {
        Self {
            sample_count: 0,
            chi_square_sum: 0.0,
            entropy_sum: 0.0,
        }
    }
    
    /// 评估熵质量
    pub fn assess_quality(&mut self, entropy: &[u8]) -> f64 {
        self.sample_count += 1;
        
        // 计算卡方统计量
        let chi_square = self.calculate_chi_square(entropy);
        self.chi_square_sum += chi_square;
        
        // 计算香农熵
        let entropy_value = self.calculate_shannon_entropy(entropy);
        self.entropy_sum += entropy_value;
        
        // 综合质量评分
        let chi_square_score = if chi_square < 255.0 { 1.0 } else { 0.0 };
        let entropy_score = entropy_value / 8.0; // 归一化到0-1
        
        (chi_square_score + entropy_score) / 2.0
    }
    
    /// 计算卡方统计量
    fn calculate_chi_square(&self, data: &[u8]) -> f64 {
        let mut histogram = [0u32; 256];
        for &byte in data {
            histogram[byte as usize] += 1;
        }
        
        let n = data.len() as f64;
        let expected_freq = n / 256.0;
        
        histogram.iter()
            .map(|&freq| {
                let diff = freq as f64 - expected_freq;
                diff.powi(2) / expected_freq
            })
            .sum()
    }
    
    /// 计算香农熵
    fn calculate_shannon_entropy(&self, data: &[u8]) -> f64 {
        let mut histogram = [0u32; 256];
        for &byte in data {
            histogram[byte as usize] += 1;
        }
        
        let total: u32 = histogram.iter().sum();
        if total == 0 {
            return 0.0;
        }
        
        histogram.iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / total as f64;
                -p * p.log2()
            })
            .sum()
    }
    
    /// 获取平均质量评分
    pub fn get_average_quality(&self) -> f64 {
        if self.sample_count == 0 {
            0.0
        } else {
            let avg_chi_square = self.chi_square_sum / self.sample_count as f64;
            let avg_entropy = self.entropy_sum / self.sample_count as f64;
            
            let chi_square_score = if avg_chi_square < 255.0 { 1.0 } else { 0.0 };
            let entropy_score = avg_entropy / 8.0;
            
            (chi_square_score + entropy_score) / 2.0
        }
    }
}
