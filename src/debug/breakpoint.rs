//! 断点模块

use crate::cpu::CPU;

/// 断点
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub address: u16,
    pub condition: Option<String>,
    pub enabled: bool,
    pub hit_count: u64,
}

impl Breakpoint {
    /// 创建新的断点
    pub fn new(address: u16, condition: Option<String>) -> Self {
        Self {
            address,
            condition,
            enabled: true,
            hit_count: 0,
        }
    }

    /// 检查断点是否应该触发
    pub fn should_trigger(&mut self, cpu: &CPU) -> bool {
        if !self.enabled {
            return false;
        }

        if cpu.pc == self.address {
            self.hit_count += 1;
            true
        } else {
            false
        }
    }

    /// 简单检查断点是否应该触发
    pub fn should_trigger_simple(&mut self, pc: u16) -> bool {
        if !self.enabled {
            return false;
        }

        if pc == self.address {
            self.hit_count += 1;
            true
        } else {
            false
        }
    }

    /// 启用断点
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// 禁用断点
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
