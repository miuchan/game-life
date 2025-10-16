//! 调试器实现

use std::collections::HashMap;
use crate::cpu::{CPU, Registers, FlagsRegister};
use crate::memory::MemoryBus;
use crate::instructions::Instruction;
use super::breakpoint::Breakpoint;
use super::disassembler::Disassembler;

/// 调试器状态
#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerState {
    Running,
    Paused,
    Stepping,
    BreakpointHit,
}

/// 调试器
#[derive(Debug)]
pub struct Debugger {
    pub state: DebuggerState,
    pub breakpoints: HashMap<u16, Breakpoint>,
    pub disassembler: Disassembler,
    pub step_count: u64,
    pub max_steps: Option<u64>,
    pub log_level: LogLevel,
    pub instruction_history: Vec<InstructionRecord>,
    pub max_history: usize,
}

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    None,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

/// 指令记录
#[derive(Debug, Clone)]
pub struct InstructionRecord {
    pub pc: u16,
    pub instruction: Instruction,
    pub registers: Registers,
    pub flags: FlagsRegister,
    pub cycle: u64,
}

impl Debugger {
    /// 创建新的调试器
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Running,
            breakpoints: HashMap::new(),
            disassembler: Disassembler::new(),
            step_count: 0,
            max_steps: None,
            log_level: LogLevel::Info,
            instruction_history: Vec::new(),
            max_history: 1000,
        }
    }

    /// 设置断点
    pub fn set_breakpoint(&mut self, address: u16, condition: Option<String>) {
        let breakpoint = Breakpoint::new(address, condition);
        self.breakpoints.insert(address, breakpoint);
        self.log(LogLevel::Info, &format!("断点设置在地址 0x{:04X}", address));
    }

    /// 移除断点
    pub fn remove_breakpoint(&mut self, address: u16) {
        self.breakpoints.remove(&address);
        self.log(LogLevel::Info, &format!("断点从地址 0x{:04X} 移除", address));
    }

    /// 清除所有断点
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
        self.log(LogLevel::Info, "所有断点已清除");
    }

    /// 暂停执行
    pub fn pause(&mut self) {
        self.state = DebuggerState::Paused;
        self.log(LogLevel::Info, "执行已暂停");
    }

    /// 恢复执行
    pub fn resume(&mut self) {
        self.state = DebuggerState::Running;
        self.log(LogLevel::Info, "执行已恢复");
    }

    /// 单步执行
    pub fn step(&mut self) {
        self.state = DebuggerState::Stepping;
        self.log(LogLevel::Debug, "单步执行");
    }

    /// 检查断点
    pub fn check_breakpoint(&mut self, pc: u16, _registers: &Registers, _flags: &FlagsRegister) -> bool {
        if let Some(breakpoint) = self.breakpoints.get_mut(&pc) {
            if breakpoint.should_trigger_simple(pc) {
                self.state = DebuggerState::BreakpointHit;
                self.log(LogLevel::Info, &format!("断点触发在地址 0x{:04X}", pc));
                return true;
            }
        }
        false
    }

    /// 记录指令执行
    pub fn record_instruction(&mut self, pc: u16, instruction: Instruction, cpu: &CPU, cycle: u64) {
        let record = InstructionRecord {
            pc,
            instruction: instruction.clone(),
            registers: cpu.registers.clone(),
            flags: cpu.flags.clone(),
            cycle,
        };

        self.instruction_history.push(record);
        
        // 保持历史记录在限制范围内
        if self.instruction_history.len() > self.max_history {
            self.instruction_history.remove(0);
        }
    }

    /// 获取CPU状态
    pub fn get_cpu_state(&self, cpu: &CPU) -> CPUState {
        CPUState {
            pc: cpu.pc,
            sp: cpu.sp,
            registers: cpu.registers.clone(),
            flags: cpu.flags.clone(),
            cycle_count: self.step_count,
        }
    }

    /// 反汇编指令
    pub fn disassemble_instruction(&self, pc: u16, memory: &MemoryBus) -> String {
        self.disassembler.disassemble_at(pc, memory)
    }

    /// 反汇编内存区域
    pub fn disassemble_range(&self, start: u16, end: u16, memory: &MemoryBus) -> Vec<String> {
        self.disassembler.disassemble_range(start, end, memory)
    }

    /// 设置日志级别
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    /// 记录日志
    pub fn log(&self, level: LogLevel, message: &str) {
        if self.should_log(level) {
            let level_str = self.get_level_string(level);
            println!("[DEBUG] {}: {}", level_str, message);
        }
    }

    /// 检查是否应该记录日志
    fn should_log(&self, level: LogLevel) -> bool {
        match (&self.log_level, level) {
            (LogLevel::None, _) => false,
            (LogLevel::Error, LogLevel::Error) => true,
            (LogLevel::Warning, LogLevel::Error | LogLevel::Warning) => true,
            (LogLevel::Info, LogLevel::Error | LogLevel::Warning | LogLevel::Info) => true,
            (LogLevel::Debug, LogLevel::Error | LogLevel::Warning | LogLevel::Info | LogLevel::Debug) => true,
            (LogLevel::Trace, _) => true,
            _ => false,
        }
    }

    /// 获取日志级别字符串
    fn get_level_string(&self, level: LogLevel) -> &'static str {
        match level {
            LogLevel::None => "NONE",
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }

    /// 获取指令历史
    pub fn get_instruction_history(&self) -> &[InstructionRecord] {
        &self.instruction_history
    }

    /// 设置最大步数
    pub fn set_max_steps(&mut self, max_steps: Option<u64>) {
        self.max_steps = max_steps;
    }

    /// 检查是否达到最大步数
    pub fn check_max_steps(&self) -> bool {
        if let Some(max) = self.max_steps {
            self.step_count >= max
        } else {
            false
        }
    }

    /// 增加步数计数
    pub fn increment_step_count(&mut self) {
        self.step_count += 1;
    }
}

/// CPU状态快照
#[derive(Debug, Clone)]
pub struct CPUState {
    pub pc: u16,
    pub sp: u16,
    pub registers: Registers,
    pub flags: FlagsRegister,
    pub cycle_count: u64,
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_creation() {
        let debugger = Debugger::new();
        assert_eq!(debugger.state, DebuggerState::Running);
        assert_eq!(debugger.breakpoints.len(), 0);
    }

    #[test]
    fn test_breakpoint_management() {
        let mut debugger = Debugger::new();
        debugger.set_breakpoint(0x100, None);
        assert_eq!(debugger.breakpoints.len(), 1);
        
        debugger.remove_breakpoint(0x100);
        assert_eq!(debugger.breakpoints.len(), 0);
    }

    #[test]
    fn test_log_levels() {
        let mut debugger = Debugger::new();
        debugger.set_log_level(LogLevel::Debug);
        assert_eq!(debugger.log_level, LogLevel::Debug);
    }
}
