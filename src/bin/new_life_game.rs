//! 全新的生命游戏 - 基于外部熵源的细胞自动机模拟
//! 
//! 使用多种外部熵源优化概率空间分布，生成更真实的生命模式

use gameboy_emulator::entropy::{
    EntropyManager, EntropyError,
    entropy_pool::PooledEntropy,
};

use std::time::{Duration, Instant};
use std::thread;
use std::io::{self, Write};

/// 生命游戏网格
#[derive(Clone)]
struct LifeGrid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
    generation: u32,
}

impl LifeGrid {
    /// 创建新的生命游戏网格
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![false; height]; width],
            generation: 0,
        }
    }
    
    /// 随机初始化网格
    fn random_init(&mut self, entropy_pool: &mut PooledEntropy, density: f64) {
        for x in 0..self.width {
            for y in 0..self.height {
                let random_value = entropy_pool.get_random_range(0, 1000);
                self.cells[x][y] = (random_value as f64 / 1000.0) < density;
            }
        }
    }
    
    /// 设置特定模式
    fn set_pattern(&mut self, pattern: &[&str], start_x: usize, start_y: usize) {
        for (dy, row) in pattern.iter().enumerate() {
            for (dx, ch) in row.chars().enumerate() {
                let x = start_x + dx;
                let y = start_y + dy;
                if x < self.width && y < self.height {
                    self.cells[x][y] = ch == 'X';
                }
            }
        }
    }
    
    /// 计算下一代
    fn next_generation(&mut self) {
        let mut new_cells = vec![vec![false; self.height]; self.width];
        
        for x in 0..self.width {
            for y in 0..self.height {
                let live_neighbors = self.count_live_neighbors(x, y);
                let current_cell = self.cells[x][y];
                
                // 生命游戏规则
                new_cells[x][y] = match (current_cell, live_neighbors) {
                    (true, 2) | (true, 3) => true,  // 存活
                    (false, 3) => true,             // 繁殖
                    _ => false,                     // 死亡
                };
            }
        }
        
        self.cells = new_cells;
        self.generation += 1;
    }
    
    /// 计算活邻居数量
    fn count_live_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                
                if nx < self.width && ny < self.height && self.cells[nx][ny] {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    /// 计算活细胞数量
    fn count_live_cells(&self) -> usize {
        self.cells.iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell)
            .count()
    }
    
    /// 计算熵值（用于分析模式复杂度）
    fn calculate_entropy(&self) -> f64 {
        let mut histogram = [0u32; 9]; // 0-8个邻居的分布
        
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.count_live_neighbors(x, y);
                histogram[neighbors as usize] += 1;
            }
        }
        
        let total = (self.width * self.height) as f64;
        histogram.iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / total;
                -p * p.log2()
            })
            .sum()
    }
    
    /// 显示网格
    fn display(&self, clear_screen: bool) {
        if clear_screen {
            print!("\x1B[2J\x1B[1;1H"); // 清屏
        }
        
        println!("🔄 第{}代生命游戏 | 活细胞: {} | 熵值: {:.3}", 
                 self.generation, self.count_live_cells(), self.calculate_entropy());
        println!("{}", "─".repeat(self.width + 2));
        
        for y in 0..self.height {
            print!("│");
            for x in 0..self.width {
                print!("{}", if self.cells[x][y] { "●" } else { " " });
            }
            println!("│");
        }
        
        println!("{}", "─".repeat(self.width + 2));
        io::stdout().flush().unwrap();
    }
    
    /// 检查是否稳定（连续几代无变化）
    fn is_stable(&self, prev_cells: &Vec<Vec<bool>>) -> bool {
        self.cells == *prev_cells
    }
}

/// 生命游戏模拟器
struct LifeGameSimulator {
    grid: LifeGrid,
    entropy_manager: EntropyManager,
    entropy_pool: PooledEntropy,
    patterns: Vec<Pattern>,
    stats: SimulationStats,
}

#[derive(Debug)]
struct Pattern {
    name: String,
    pattern: Vec<&'static str>,
    description: String,
}

#[derive(Debug)]
struct SimulationStats {
    total_generations: u32,
    max_population: usize,
    min_population: usize,
    avg_entropy: f64,
    stability_count: u32,
    start_time: Instant,
}

impl LifeGameSimulator {
    /// 创建新的生命游戏模拟器
    fn new(width: usize, height: usize) -> Result<Self, EntropyError> {
        let mut entropy_manager = EntropyManager::new();
        let mut entropy_pool = PooledEntropy::new(2048);
        
        // 收集初始熵
        let _ = entropy_manager.collect_and_optimize();
        let random_data = entropy_manager.generate_random(512)?;
        entropy_pool.add_entropy_source(&random_data);
        
        let mut grid = LifeGrid::new(width, height);
        
        // 初始化网格
        grid.random_init(&mut entropy_pool, 0.3);
        
        let patterns = Self::create_patterns();
        
        Ok(Self {
            grid,
            entropy_manager,
            entropy_pool,
            patterns,
            stats: SimulationStats {
                total_generations: 0,
                max_population: 0,
                min_population: usize::MAX,
                avg_entropy: 0.0,
                stability_count: 0,
                start_time: Instant::now(),
            },
        })
    }
    
    /// 创建经典模式
    fn create_patterns() -> Vec<Pattern> {
        vec![
            Pattern {
                name: "滑翔机".to_string(),
                pattern: vec![
                    " X ",
                    "X X",
                    " XX",
                ],
                description: "会移动的简单模式".to_string(),
            },
            Pattern {
                name: "脉冲星".to_string(),
                pattern: vec![
                    "  XXX   XXX  ",
                    " X   X X   X ",
                    "X     X     X",
                    "X     X     X",
                    "X     X     X",
                    " X   X X   X ",
                    "  XXX   XXX  ",
                ],
                description: "周期性振荡模式".to_string(),
            },
            Pattern {
                name: "信标".to_string(),
                pattern: vec![
                    "XX  ",
                    "XX  ",
                    "  XX",
                    "  XX",
                ],
                description: "周期性闪烁模式".to_string(),
            },
            Pattern {
                name: "蟾蜍".to_string(),
                pattern: vec![
                    " XXX",
                    "XXX ",
                ],
                description: "周期性振荡模式".to_string(),
            },
        ]
    }
    
    /// 添加随机模式
    fn add_random_pattern(&mut self) -> Result<(), EntropyError> {
        let pattern_idx = self.entropy_pool.get_random_range(0, self.patterns.len() as u32) as usize;
        let pattern = &self.patterns[pattern_idx];
        
        let x = self.entropy_pool.get_random_range(0, (self.grid.width - pattern.pattern[0].len()) as u32) as usize;
        let y = self.entropy_pool.get_random_range(0, (self.grid.height - pattern.pattern.len()) as u32) as usize;
        
        self.grid.set_pattern(&pattern.pattern, x, y);
        
        println!("🎯 添加模式: {} 在位置 ({}, {})", pattern.name, x, y);
        Ok(())
    }
    
    /// 运行模拟
    fn run(&mut self, max_generations: u32, display_interval: u32) -> Result<(), EntropyError> {
        println!("🌱 全新的生命游戏开始！");
        println!("使用外部熵源优化概率空间分布");
        println!("网格大小: {}x{}", self.grid.width, self.grid.height);
        println!("最大代数: {}", max_generations);
        println!();
        
        let mut prev_cells = self.grid.cells.clone();
        let mut entropy_sum = 0.0;
        
        for generation in 0..max_generations {
            // 更新统计信息
            let population = self.grid.count_live_cells();
            self.stats.max_population = self.stats.max_population.max(population);
            self.stats.min_population = self.stats.min_population.min(population);
            
            let entropy = self.grid.calculate_entropy();
            entropy_sum += entropy;
            
            // 检查稳定性
            if self.grid.is_stable(&prev_cells) {
                self.stats.stability_count += 1;
                if self.stats.stability_count >= 5 {
                    println!("🔒 系统达到稳定状态，在第{}代", generation);
                    break;
                }
            } else {
                self.stats.stability_count = 0;
            }
            
            // 显示网格
            if generation % display_interval == 0 {
                self.grid.display(true);
                
                // 显示统计信息
                println!("📊 统计信息:");
                println!("  总代数: {}", generation);
                println!("  当前种群: {}", population);
                println!("  最大种群: {}", self.stats.max_population);
                println!("  最小种群: {}", self.stats.min_population);
                println!("  平均熵值: {:.3}", entropy_sum / (generation + 1) as f64);
                println!("  运行时间: {:.2}秒", self.stats.start_time.elapsed().as_secs_f64());
                
                // 显示熵源统计
                let entropy_stats = self.entropy_manager.get_entropy_stats();
                println!("🔬 熵源统计:");
                println!("  熵源数量: {}", entropy_stats.source_count);
                println!("  池大小: {} 字节", entropy_stats.pool_size);
                println!("  分布质量: {:.3}", entropy_stats.optimizer_stats.distribution_quality);
                println!("  量子强度: {:.3}", entropy_stats.quantum_stats.post_quantum_strength);
                
                println!();
                println!("按回车键继续，或输入 'q' 退出，'p' 添加模式...");
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                match input.trim() {
                    "q" => {
                        println!("👋 游戏结束！");
                        break;
                    }
                    "p" => {
                        self.add_random_pattern()?;
                    }
                    _ => {}
                }
            }
            
            // 更新到下一代
            prev_cells = self.grid.cells.clone();
            self.grid.next_generation();
            
            // 控制速度
            thread::sleep(Duration::from_millis(100));
        }
        
        self.stats.total_generations = self.grid.generation;
        self.stats.avg_entropy = entropy_sum / self.grid.generation as f64;
        
        Ok(())
    }
    
    /// 显示最终统计
    fn display_final_stats(&self) {
        println!("\n🎉 生命游戏模拟完成！");
        println!("==================================================");
        println!("📈 最终统计:");
        println!("  总代数: {}", self.stats.total_generations);
        println!("  最大种群: {}", self.stats.max_population);
        println!("  最小种群: {}", self.stats.min_population);
        println!("  平均熵值: {:.3}", self.stats.avg_entropy);
        println!("  总运行时间: {:.2}秒", self.stats.start_time.elapsed().as_secs_f64());
        println!("  平均每代时间: {:.3}毫秒", 
                 self.stats.start_time.elapsed().as_millis() as f64 / self.stats.total_generations as f64);
        
        // 显示熵源统计
        let entropy_stats = self.entropy_manager.get_entropy_stats();
        println!("\n🔬 熵源系统统计:");
        println!("  熵源数量: {}", entropy_stats.source_count);
        println!("  池大小: {} 字节", entropy_stats.pool_size);
        println!("  分布质量: {:.3}", entropy_stats.optimizer_stats.distribution_quality);
        println!("  量子强度: {:.3}", entropy_stats.quantum_stats.post_quantum_strength);
        println!("  处理时间: {} 纳秒", entropy_stats.quantum_stats.processing_time_ns);
        println!("  熵放大: {:.3}", entropy_stats.quantum_stats.entropy_amplification);
        
        println!("\n🎯 模式库:");
        for (i, pattern) in self.patterns.iter().enumerate() {
            println!("  {}. {} - {}", i + 1, pattern.name, pattern.description);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 全新的生命游戏");
    println!("基于外部熵源的细胞自动机模拟");
    println!("优化概率空间分布，生成更真实的生命模式");
    println!();
    
    // 创建模拟器
    let mut simulator = LifeGameSimulator::new(40, 20)?;
    
    // 运行模拟
    simulator.run(1000, 5)?;
    
    // 显示最终统计
    simulator.display_final_stats();
    
    Ok(())
}
