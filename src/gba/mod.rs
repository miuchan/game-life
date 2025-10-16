//! GBA主模拟器实现
//! 
//! 这个模块实现了Game Boy Advance的主模拟器，
//! 协调CPU、GPU、内存等各个组件

mod cpu;
mod gpu;

use cpu::{ARM7TDMI, GBAMemory};
use gpu::GBAGPU;
use std::time::Instant;

/// GBA主模拟器
#[derive(Debug)]
pub struct GBASystem {
    /// ARM7TDMI CPU
    pub cpu: ARM7TDMI,
    /// 内存管理单元
    pub memory: GBAMemory,
    /// 图形处理单元
    pub gpu: GBAGPU,
    /// 模拟器状态
    pub state: GBAState,
    /// 性能统计
    pub stats: GBAStats,
    /// 启动时间
    pub start_time: Instant,
}

/// GBA模拟器状态
#[derive(Debug, Clone, PartialEq)]
pub enum GBAState {
    Stopped,
    Running,
    Paused,
    Error(String),
}

/// GBA性能统计
#[derive(Debug, Clone, Default)]
pub struct GBAStats {
    pub total_cycles: u64,
    pub total_frames: u32,
    pub fps: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub gpu_usage: f64,
    pub execution_time: f64,
}

impl GBASystem {
    /// 创建新的GBA模拟器实例
    pub fn new() -> Self {
        Self {
            cpu: ARM7TDMI::new(),
            memory: GBAMemory::new(),
            gpu: GBAGPU::new(),
            state: GBAState::Stopped,
            stats: GBAStats::default(),
            start_time: Instant::now(),
        }
    }
    
    /// 重置模拟器到初始状态
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory = GBAMemory::new();
        self.gpu.reset();
        self.state = GBAState::Stopped;
        self.stats = GBAStats::default();
        self.start_time = Instant::now();
    }
    
    /// 加载ROM文件
    pub fn load_rom(&mut self, rom_data: Vec<u8>) -> Result<(), String> {
        if rom_data.len() < 0x200 {
            return Err("ROM文件太小".to_string());
        }
        
        // 验证ROM头
        if !self.validate_rom_header(&rom_data) {
            return Err("无效的ROM文件".to_string());
        }
        
        self.memory.load_rom(rom_data);
        self.state = GBAState::Stopped;
        
        Ok(())
    }
    
    /// 验证ROM头
    fn validate_rom_header(&self, rom_data: &[u8]) -> bool {
        if rom_data.len() < 0xC0 {
            return false;
        }
        
        // 简化验证：检查ROM大小和基本结构
        rom_data.len() >= 0x200
    }
    
    /// 获取Nintendo标志数据
    fn get_nintendo_logo(&self) -> &[u8] {
        // GBA Nintendo标志数据
        &[
            0x24, 0xFF, 0xAE, 0x51, 0x69, 0x9A, 0xA2, 0x21, 0x3D, 0x84, 0x82, 0x8A, 0x84, 0x24, 0x04, 0x51,
            0x11, 0x40, 0x9C, 0x00, 0x21, 0x13, 0x82, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    }
    
    /// 启动模拟器
    pub fn start(&mut self) -> Result<(), String> {
        if self.memory.rom.is_empty() {
            return Err("没有加载ROM文件".to_string());
        }
        
        self.state = GBAState::Running;
        self.start_time = Instant::now();
        
        Ok(())
    }
    
    /// 停止模拟器
    pub fn stop(&mut self) {
        self.state = GBAState::Stopped;
    }
    
    /// 暂停模拟器
    pub fn pause(&mut self) {
        if self.state == GBAState::Running {
            self.state = GBAState::Paused;
        }
    }
    
    /// 恢复模拟器
    pub fn resume(&mut self) {
        if self.state == GBAState::Paused {
            self.state = GBAState::Running;
        }
    }
    
    /// 执行一个CPU周期
    pub fn step(&mut self) -> Result<(), String> {
        if self.state != GBAState::Running {
            return Ok(());
        }
        
        // 执行CPU指令
        self.cpu.execute_instruction(&mut self.memory)?;
        
        // 更新GPU
        self.gpu.update();
        
        // 更新统计
        self.update_stats();
        
        Ok(())
    }
    
    /// 运行指定数量的周期
    pub fn run_cycles(&mut self, cycles: u64) -> Result<(), String> {
        for _ in 0..cycles {
            self.step()?;
        }
        Ok(())
    }
    
    /// 运行到下一帧
    pub fn run_frame(&mut self) -> Result<(), String> {
        let start_frame = self.gpu.frame_count;
        
        while self.gpu.frame_count == start_frame && self.state == GBAState::Running {
            self.step()?;
        }
        
        Ok(())
    }
    
    /// 更新性能统计
    fn update_stats(&mut self) {
        let elapsed = self.start_time.elapsed();
        self.stats.execution_time = elapsed.as_secs_f64();
        
        // 更新CPU统计
        let cpu_stats = self.cpu.get_stats();
        self.stats.total_cycles = cpu_stats.cycles;
        
        // 更新GPU统计
        let gpu_stats = self.gpu.get_stats();
        self.stats.total_frames = gpu_stats.frames_rendered;
        
        // 计算FPS
        if elapsed.as_secs_f64() > 0.0 {
            self.stats.fps = gpu_stats.frames_rendered as f64 / elapsed.as_secs_f64();
        }
        
        // 计算CPU使用率
        self.stats.cpu_usage = (cpu_stats.cycles as f64 / elapsed.as_secs_f64()) / 16_777_216.0; // 16.78MHz
        
        // 计算内存使用率
        let memory_stats = self.memory.get_stats();
        self.stats.memory_usage = (memory_stats.reads + memory_stats.writes) as f64 / elapsed.as_secs_f64();
        
        // 计算GPU使用率
        self.stats.gpu_usage = gpu_stats.pixels_drawn as f64 / elapsed.as_secs_f64();
    }
    
    /// 获取模拟器状态
    pub fn get_state(&self) -> &GBAState {
        &self.state
    }
    
    /// 获取性能统计
    pub fn get_stats(&self) -> &GBAStats {
        &self.stats
    }
    
    /// 获取CPU状态
    pub fn get_cpu_state(&self) -> &ARM7TDMI {
        &self.cpu
    }
    
    /// 获取内存状态
    pub fn get_memory_state(&self) -> &GBAMemory {
        &self.memory
    }
    
    /// 获取GPU状态
    pub fn get_gpu_state(&self) -> &GBAGPU {
        &self.gpu
    }
    
    /// 获取调试信息
    pub fn get_debug_info(&self) -> String {
        let cpu_stats = self.cpu.get_stats();
        let memory_stats = self.memory.get_stats();
        let gpu_stats = self.gpu.get_stats();
        
        format!(
            "=== GBA模拟器调试信息 ===\n\
            状态: {:?}\n\
            PC: 0x{:08X}\n\
            Thumb模式: {}\n\
            寄存器: R0=0x{:08X}, R1=0x{:08X}, R2=0x{:08X}, R3=0x{:08X}\n\
            CPSR: 0x{:08X}\n\
            CPU周期: {}\n\
            CPU指令: {} (ARM: {}, Thumb: {})\n\
            内存读取: {}, 写入: {}\n\
            GPU帧数: {}\n\
            GPU像素: {}\n\
            FPS: {:.2}\n\
            CPU使用率: {:.2}%\n\
            =========================",
            self.state,
            self.cpu.get_pc(),
            self.cpu.thumb_mode,
            self.cpu.get_register(0),
            self.cpu.get_register(1),
            self.cpu.get_register(2),
            self.cpu.get_register(3),
            self.cpu.cpsr,
            cpu_stats.cycles,
            cpu_stats.instructions,
            cpu_stats.arm_instructions,
            cpu_stats.thumb_instructions,
            memory_stats.reads,
            memory_stats.writes,
            gpu_stats.frames_rendered,
            gpu_stats.pixels_drawn,
            self.stats.fps,
            self.stats.cpu_usage * 100.0
        )
    }
}

impl Default for GBASystem {
    fn default() -> Self {
        Self::new()
    }
}
