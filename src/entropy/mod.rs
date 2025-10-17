//! 外部熵源随机数发生器模块
//! 
//! 本模块实现了多种外部熵源的集成，用于优化整个系统的概率空间分布
//! 通过结合多种熵源，确保随机数的高质量和不可预测性

pub mod entropy_source;
pub mod distribution_optimizer;
pub mod quantum_resistant;
pub mod entropy_pool;

pub use entropy_source::{EntropySource, EntropySourceType, EntropyCollector};
pub use distribution_optimizer::{DistributionOptimizer, ProbabilitySpace};
pub use quantum_resistant::{QuantumResistantRNG, PostQuantumEntropy};
pub use entropy_pool::{EntropyPool, PooledEntropy};

/// 主熵源管理器
pub struct EntropyManager {
    sources: Vec<Box<dyn EntropySource>>,
    optimizer: DistributionOptimizer,
    pool: EntropyPool,
    quantum_rng: QuantumResistantRNG,
}

impl EntropyManager {
    /// 创建新的熵源管理器
    pub fn new() -> Self {
        let mut sources: Vec<Box<dyn EntropySource>> = Vec::new();
        
        // 添加多种熵源
        sources.push(Box::new(SystemTimeEntropy::new()));
        sources.push(Box::new(HardwareEntropy::new()));
        sources.push(Box::new(NetworkEntropy::new()));
        sources.push(Box::new(ProcessEntropy::new()));
        sources.push(Box::new(MemoryEntropy::new()));
        
        Self {
            sources,
            optimizer: DistributionOptimizer::new(),
            pool: EntropyPool::new(),
            quantum_rng: QuantumResistantRNG::new(),
        }
    }
    
    /// 收集熵并优化分布
    pub fn collect_and_optimize(&mut self) -> Result<Vec<u8>, EntropyError> {
        let mut collected_entropy = Vec::new();
        
        // 从所有源收集熵
        for source in &mut self.sources {
            match source.collect_entropy() {
                Ok(entropy) => collected_entropy.extend(entropy),
                Err(e) => eprintln!("熵源错误: {}", e),
            }
        }
        
        // 将熵添加到池中
        self.pool.add_entropy(&collected_entropy);
        
        // 优化概率分布
        let optimized = self.optimizer.optimize_distribution(&collected_entropy)?;
        
        Ok(optimized)
    }
    
    /// 生成高质量随机数
    pub fn generate_random(&mut self, size: usize) -> Result<Vec<u8>, EntropyError> {
        // 首先收集和优化熵
        let _ = self.collect_and_optimize();
        
        // 从池中提取熵
        let pool_entropy = self.pool.extract_entropy(size);
        
        // 使用量子抗性RNG进一步处理
        let quantum_processed = self.quantum_rng.process_entropy(&pool_entropy)?;
        
        Ok(quantum_processed)
    }
    
    /// 获取熵源统计信息
    pub fn get_entropy_stats(&self) -> EntropyStats {
        EntropyStats {
            source_count: self.sources.len(),
            pool_size: self.pool.size(),
            optimizer_stats: self.optimizer.get_stats(),
            quantum_stats: self.quantum_rng.get_stats(),
        }
    }
}

/// 熵源错误类型
#[derive(Debug)]
pub enum EntropyError {
    SourceUnavailable(String),
    InsufficientEntropy,
    DistributionError(String),
    QuantumProcessingError(String),
}

impl std::fmt::Display for EntropyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EntropyError::SourceUnavailable(msg) => write!(f, "熵源不可用: {}", msg),
            EntropyError::InsufficientEntropy => write!(f, "熵不足"),
            EntropyError::DistributionError(msg) => write!(f, "分布错误: {}", msg),
            EntropyError::QuantumProcessingError(msg) => write!(f, "量子处理错误: {}", msg),
        }
    }
}

impl std::error::Error for EntropyError {}

/// 熵源统计信息
#[derive(Debug, Clone)]
pub struct EntropyStats {
    pub source_count: usize,
    pub pool_size: usize,
    pub optimizer_stats: distribution_optimizer::OptimizerStats,
    pub quantum_stats: quantum_resistant::QuantumStats,
}

// 导入具体的熵源实现
use entropy_source::{SystemTimeEntropy, HardwareEntropy, NetworkEntropy, ProcessEntropy, MemoryEntropy};
