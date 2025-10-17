//! 概率分布优化器模块
//! 
//! 实现概率空间分布优化算法，确保随机数在概率空间上的均匀分布

use std::collections::HashMap;

/// 概率分布优化器
pub struct DistributionOptimizer {
    optimization_cycles: u32,
    distribution_quality: f64,
    entropy_density: f64,
    chi_square_threshold: f64,
}

impl DistributionOptimizer {
    pub fn new() -> Self {
        Self {
            optimization_cycles: 0,
            distribution_quality: 0.0,
            entropy_density: 0.0,
            chi_square_threshold: 0.05, // 5% 显著性水平
        }
    }
    
    /// 优化概率分布
    pub fn optimize_distribution(&mut self, entropy: &[u8]) -> Result<Vec<u8>, EntropyError> {
        self.optimization_cycles += 1;
        
        // 第一步：分析原始熵的分布
        let original_stats = self.analyze_distribution(entropy);
        
        // 第二步：应用多种优化技术
        let mut optimized = entropy.to_vec();
        
        // 1. 白化处理（去除相关性）
        optimized = self.apply_whitening(&optimized);
        
        // 2. 熵增强
        optimized = self.enhance_entropy(&optimized);
        
        // 3. 分布均衡
        optimized = self.balance_distribution(&optimized);
        
        // 4. 质量验证
        let final_stats = self.analyze_distribution(&optimized);
        self.distribution_quality = self.calculate_quality_score(&final_stats);
        self.entropy_density = self.calculate_entropy_density(&optimized);
        
        // 验证优化效果
        if self.distribution_quality < 0.5 {
            return Err(EntropyError::DistributionError(
                format!("分布质量不足: {:.3}", self.distribution_quality)
            ));
        }
        
        Ok(optimized)
    }
    
    /// 分析数据分布
    fn analyze_distribution(&self, data: &[u8]) -> DistributionStats {
        let mut histogram = [0u32; 256];
        let mut sum = 0u64;
        let mut sum_squares = 0u64;
        
        for &byte in data {
            histogram[byte as usize] += 1;
            sum += byte as u64;
            sum_squares += (byte as u64).pow(2);
        }
        
        let n = data.len() as f64;
        let mean = sum as f64 / n;
        let variance = (sum_squares as f64 / n) - mean.powi(2);
        let std_dev = variance.sqrt();
        
        // 计算卡方统计量
        let expected_freq = n / 256.0;
        let chi_square: f64 = histogram.iter()
            .map(|&freq| {
                let diff = freq as f64 - expected_freq;
                diff.powi(2) / expected_freq
            })
            .sum();
        
        DistributionStats {
            histogram,
            mean,
            variance,
            std_dev,
            chi_square,
            entropy: self.calculate_shannon_entropy(&histogram),
        }
    }
    
    /// 计算香农熵
    fn calculate_shannon_entropy(&self, histogram: &[u32; 256]) -> f64 {
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
    
    /// 应用白化处理
    fn apply_whitening(&self, data: &[u8]) -> Vec<u8> {
        let mut whitened = Vec::with_capacity(data.len());
        
        // 使用简单的XOR白化
        let mut whitening_key = 0u8;
        for &byte in data {
            whitening_key = whitening_key.wrapping_add(byte);
            whitened.push(byte ^ whitening_key);
        }
        
        // 应用额外的白化变换
        for i in 1..whitened.len() {
            whitened[i] ^= whitened[i - 1];
        }
        
        whitened
    }
    
    /// 增强熵
    fn enhance_entropy(&self, data: &[u8]) -> Vec<u8> {
        let mut enhanced = Vec::with_capacity(data.len() * 2);
        
        // 使用Feistel网络结构增强熵
        for chunk in data.chunks(2) {
            let left = chunk[0];
            let right = if chunk.len() > 1 { chunk[1] } else { 0 };
            
            // Feistel轮函数
            let mut l = left;
            let mut r = right;
            
            for round in 0..4 {
                let temp = r;
                r = l ^ self.feistel_function(r, round);
                l = temp;
            }
            
            enhanced.push(l);
            enhanced.push(r);
        }
        
        enhanced
    }
    
    /// Feistel轮函数
    fn feistel_function(&self, input: u8, round: u8) -> u8 {
        // 使用简单的非线性变换
        let mut result = input;
        result = result.wrapping_add(round);
        result = result.wrapping_mul(3);
        result = result.rotate_left(2);
        result ^= 0xAA;
        result
    }
    
    /// 平衡分布
    fn balance_distribution(&self, data: &[u8]) -> Vec<u8> {
        let mut balanced = data.to_vec();
        
        // 计算当前分布
        let mut histogram = [0u32; 256];
        for &byte in &balanced {
            histogram[byte as usize] += 1;
        }
        
        // 找到最频繁和最不频繁的值
        let max_count = *histogram.iter().max().unwrap_or(&0);
        let min_count = *histogram.iter().filter(|&&x| x > 0).min().unwrap_or(&0);
        
        if max_count > min_count + 1 {
            // 重新分布高频值
            for i in 0..balanced.len() {
                let byte = balanced[i] as usize;
                if histogram[byte] > min_count + 1 {
                    // 将高频值替换为低频值
                    let target_byte = histogram.iter()
                        .position(|&count| count == min_count)
                        .unwrap_or(byte);
                    
                    balanced[i] = target_byte as u8;
                    histogram[byte] -= 1;
                    histogram[target_byte] += 1;
                }
            }
        }
        
        balanced
    }
    
    /// 计算质量评分
    fn calculate_quality_score(&self, stats: &DistributionStats) -> f64 {
        // 基于多个指标计算质量评分
        let entropy_score = stats.entropy / 8.0; // 归一化到0-1
        let chi_square_score = if stats.chi_square < 255.0 { 1.0 } else { 0.0 };
        let variance_score = if stats.std_dev > 50.0 && stats.std_dev < 100.0 { 1.0 } else { 0.5 };
        
        (entropy_score + chi_square_score + variance_score) / 3.0
    }
    
    /// 计算熵密度
    fn calculate_entropy_density(&self, data: &[u8]) -> f64 {
        let histogram = {
            let mut hist = [0u32; 256];
            for &byte in data {
                hist[byte as usize] += 1;
            }
            hist
        };
        
        self.calculate_shannon_entropy(&histogram)
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> OptimizerStats {
        OptimizerStats {
            distribution_quality: self.distribution_quality,
            optimization_cycles: self.optimization_cycles,
            entropy_density: self.entropy_density,
        }
    }
}

/// 分布统计信息
#[derive(Debug, Clone)]
struct DistributionStats {
    histogram: [u32; 256],
    mean: f64,
    variance: f64,
    std_dev: f64,
    chi_square: f64,
    entropy: f64,
}

/// 优化器统计信息
#[derive(Debug, Clone)]
pub struct OptimizerStats {
    pub distribution_quality: f64,
    pub optimization_cycles: u32,
    pub entropy_density: f64,
}

/// 概率空间表示
pub struct ProbabilitySpace {
    dimensions: usize,
    distribution_map: HashMap<Vec<u8>, f64>,
    entropy_field: Vec<f64>,
}

impl ProbabilitySpace {
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensions,
            distribution_map: HashMap::new(),
            entropy_field: vec![0.0; dimensions],
        }
    }
    
    /// 更新概率分布
    pub fn update_distribution(&mut self, sample: &[u8], probability: f64) {
        if sample.len() == self.dimensions {
            self.distribution_map.insert(sample.to_vec(), probability);
        }
    }
    
    /// 计算空间熵
    pub fn calculate_space_entropy(&self) -> f64 {
        let total_prob: f64 = self.distribution_map.values().sum();
        if total_prob == 0.0 {
            return 0.0;
        }
        
        self.distribution_map.values()
            .map(|&prob| {
                let normalized_prob = prob / total_prob;
                if normalized_prob > 0.0 {
                    -normalized_prob * normalized_prob.log2()
                } else {
                    0.0
                }
            })
            .sum()
    }
    
    /// 优化空间分布
    pub fn optimize_space_distribution(&mut self) {
        let space_entropy = self.calculate_space_entropy();
        
        // 更新熵场
        for i in 0..self.dimensions {
            self.entropy_field[i] = space_entropy / self.dimensions as f64;
        }
        
        // 重新平衡概率分布
        let mut new_distribution = HashMap::new();
        let total_samples = self.distribution_map.len() as f64;
        let uniform_prob = 1.0 / total_samples;
        
        for (sample, _) in &self.distribution_map {
            new_distribution.insert(sample.clone(), uniform_prob);
        }
        
        self.distribution_map = new_distribution;
    }
}

// 导入错误类型
use crate::entropy::EntropyError;
