//! 外部熵源随机数发生器演示程序
//! 
//! 展示如何使用多种外部熵源优化概率空间分布

use crate::entropy::{
    EntropyManager, EntropyError,
    entropy_source::{EntropySource, SystemTimeEntropy, HardwareEntropy, NetworkEntropy, ProcessEntropy, MemoryEntropy},
    entropy_pool::{PooledEntropy, EntropyQualityAssessor},
};

fn main() -> Result<(), EntropyError> {
    println!("🎲 外部熵源随机数发生器演示");
    println!("==================================================");
    
    // 创建熵管理器
    let mut entropy_manager = EntropyManager::new();
    
    // 显示初始统计信息
    let initial_stats = entropy_manager.get_entropy_stats();
    println!("📊 初始熵源统计:");
    println!("  熵源数量: {}", initial_stats.source_count);
    println!("  池大小: {} 字节", initial_stats.pool_size);
    println!("  分布质量: {:.3}", initial_stats.optimizer_stats.distribution_quality);
    println!("  量子强度: {:.3}", initial_stats.quantum_stats.post_quantum_strength);
    println!();
    
    // 演示熵收集和优化
    println!("🔄 收集和优化熵...");
    let optimized_entropy = entropy_manager.collect_and_optimize()?;
    println!("✅ 成功收集 {} 字节的优化熵", optimized_entropy.len());
    
    // 分析熵质量
    let mut quality_assessor = EntropyQualityAssessor::new();
    let quality_score = quality_assessor.assess_quality(&optimized_entropy);
    println!("📈 熵质量评分: {:.3}", quality_score);
    println!();
    
    // 生成高质量随机数
    println!("🎯 生成高质量随机数...");
    let random_data = entropy_manager.generate_random(64)?;
    println!("✅ 成功生成 {} 字节的随机数据", random_data.len());
    
    // 显示随机数据样本
    println!("📋 随机数据样本 (前16字节):");
    for (i, &byte) in random_data.iter().take(16).enumerate() {
        if i % 8 == 0 {
            print!("  ");
        }
        print!("{:02X} ", byte);
        if i % 8 == 7 {
            println!();
        }
    }
    if random_data.len() > 16 {
        println!("  ... (还有 {} 字节)", random_data.len() - 16);
    }
    println!();
    
    // 演示池化熵
    println!("🏊 演示池化熵系统...");
    let mut pooled_entropy = PooledEntropy::new(1024);
    
    // 添加多种熵源
    let mut system_entropy = SystemTimeEntropy::new();
    let mut hardware_entropy = HardwareEntropy::new();
    let mut network_entropy = NetworkEntropy::new();
    let mut process_entropy = ProcessEntropy::new();
    let mut memory_entropy = MemoryEntropy::new();
    
    // 收集各源熵
    if let Ok(entropy) = system_entropy.collect_entropy() {
        pooled_entropy.add_entropy_source(&entropy);
        println!("✅ 添加系统时间熵: {} 字节", entropy.len());
    }
    
    if let Ok(entropy) = hardware_entropy.collect_entropy() {
        pooled_entropy.add_entropy_source(&entropy);
        println!("✅ 添加硬件熵: {} 字节", entropy.len());
    }
    
    if let Ok(entropy) = network_entropy.collect_entropy() {
        pooled_entropy.add_entropy_source(&entropy);
        println!("✅ 添加网络熵: {} 字节", entropy.len());
    }
    
    if let Ok(entropy) = process_entropy.collect_entropy() {
        pooled_entropy.add_entropy_source(&entropy);
        println!("✅ 添加进程熵: {} 字节", entropy.len());
    }
    
    if let Ok(entropy) = memory_entropy.collect_entropy() {
        pooled_entropy.add_entropy_source(&entropy);
        println!("✅ 添加内存熵: {} 字节", entropy.len());
    }
    
    // 显示池统计信息
    let pool_stats = pooled_entropy.get_pool_stats();
    println!("📊 池统计信息:");
    println!("  当前大小: {} 字节", pool_stats.current_size);
    println!("  最大大小: {} 字节", pool_stats.max_size);
    println!("  最小大小: {} 字节", pool_stats.min_size);
    println!("  补充次数: {}", pool_stats.refill_count);
    println!();
    
    // 生成各种类型的随机数
    println!("🎲 生成各种类型的随机数:");
    
    // 随机字节
    let random_byte = pooled_entropy.get_random_byte();
    println!("  随机字节: 0x{:02X}", random_byte);
    
    // 随机32位整数
    let random_u32 = pooled_entropy.get_random_u32();
    println!("  随机32位整数: {}", random_u32);
    
    // 随机范围整数
    let random_range = pooled_entropy.get_random_range(1, 100);
    println!("  随机范围整数 (1-100): {}", random_range);
    
    // 随机字节数组
    let random_bytes = pooled_entropy.get_random_bytes(8);
    println!("  随机字节数组: {:?}", random_bytes);
    println!();
    
    // 概率分布分析
    println!("📊 概率分布分析:");
    analyze_distribution(&random_data);
    println!();
    
    // 最终统计信息
    let final_stats = entropy_manager.get_entropy_stats();
    println!("📈 最终统计信息:");
    println!("  优化周期: {}", final_stats.optimizer_stats.optimization_cycles);
    println!("  分布质量: {:.3}", final_stats.optimizer_stats.distribution_quality);
    println!("  熵密度: {:.3}", final_stats.optimizer_stats.entropy_density);
    println!("  量子强度: {:.3}", final_stats.quantum_stats.post_quantum_strength);
    println!("  处理时间: {} 纳秒", final_stats.quantum_stats.processing_time_ns);
    println!("  熵放大: {:.3}", final_stats.quantum_stats.entropy_amplification);
    println!();
    
    println!("🎉 外部熵源随机数发生器演示完成！");
    println!("✨ 系统已成功优化概率空间分布，提供高质量的随机数生成能力");
    
    Ok(())
}

/// 分析数据分布
fn analyze_distribution(data: &[u8]) {
    let mut histogram = [0u32; 256];
    for &byte in data {
        histogram[byte as usize] += 1;
    }
    
    let n = data.len() as f64;
    let expected_freq = n / 256.0;
    
    // 计算卡方统计量
    let chi_square: f64 = histogram.iter()
        .map(|&freq| {
            let diff = freq as f64 - expected_freq;
            diff.powi(2) / expected_freq
        })
        .sum();
    
    // 计算香农熵
    let entropy: f64 = histogram.iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / n;
            -p * p.log2()
        })
        .sum();
    
    // 计算方差
    let mean = data.iter().map(|&x| x as f64).sum::<f64>() / n;
    let variance = data.iter()
        .map(|&x| ((x as f64) - mean).powi(2))
        .sum::<f64>() / n;
    let std_dev = variance.sqrt();
    
    println!("  数据长度: {} 字节", data.len());
    println!("  平均值: {:.2}", mean);
    println!("  标准差: {:.2}", std_dev);
    println!("  香农熵: {:.3} bits", entropy);
    println!("  卡方统计量: {:.2}", chi_square);
    
    // 分布质量评估
    let chi_square_score = if chi_square < 255.0 { "优秀" } else { "需要改进" };
    let entropy_score = if entropy > 7.5 { "优秀" } else if entropy > 7.0 { "良好" } else { "需要改进" };
    
    println!("  卡方测试: {}", chi_square_score);
    println!("  熵质量: {}", entropy_score);
    
    // 显示最频繁和最不频繁的值
    let max_count = histogram.iter().max().unwrap();
    let min_count = histogram.iter().filter(|&&x| x > 0).min().unwrap();
    let max_byte = histogram.iter().position(|&x| x == *max_count).unwrap();
    let min_byte = histogram.iter().position(|&x| x == *min_count).unwrap();
    
    println!("  最频繁值: 0x{:02X} (出现 {} 次)", max_byte, max_count);
    println!("  最不频繁值: 0x{:02X} (出现 {} 次)", min_byte, min_count);
}
