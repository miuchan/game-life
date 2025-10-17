//! 内存总线模块 - 包含内存读写操作

/// 内存总线结构
#[derive(Debug, Clone)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    /// 创建新的内存总线实例
    pub fn new() -> Self {
        Self {
            memory: [0u8; 0xFFFF],
        }
    }

    /// 从指定地址读取一个字节
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    
    /// 向指定地址写入一个字节
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    
    /// 从指定地址读取一个字（16位，小端序）
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.memory[address as usize] as u16;
        let high = self.memory[(address + 1) as usize] as u16;
        (high << 8) | low
    }
    
    /// 向指定地址写入一个字（16位，小端序）
    pub fn write_word(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = (value & 0xFF) as u8;
        self.memory[(address + 1) as usize] = ((value >> 8) & 0xFF) as u8;
    }

    /// 加载程序到内存
    pub fn load_program(&mut self, start_address: u16, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            let address = start_address + i as u16;
            if address < 0xFFFF {
                self.memory[address as usize] = byte;
            }
        }
    }

    /// 获取内存的只读引用
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// 获取内存的可变引用
    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}
