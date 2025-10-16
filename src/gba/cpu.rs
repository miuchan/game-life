//! GBA ARM7TDMI CPU核心实现
//! 
//! 这个模块实现了Game Boy Advance的ARM7TDMI CPU核心，
//! 包括ARM和Thumb指令集支持

use std::collections::HashMap;

/// ARM7TDMI CPU状态
#[derive(Debug, Clone)]
pub struct ARM7TDMI {
    /// 通用寄存器 (R0-R15)
    pub registers: [u32; 16],
    /// 程序计数器 (R15)
    pub pc: u32,
    /// 链接寄存器 (R14)
    pub lr: u32,
    /// 栈指针 (R13)
    pub sp: u32,
    /// 当前程序状态寄存器
    pub cpsr: u32,
    /// 保存的程序状态寄存器 (用于模式切换)
    pub spsr: u32,
    /// 当前执行模式
    pub mode: CPUMode,
    /// Thumb模式标志
    pub thumb_mode: bool,
    /// 指令缓存
    pub instruction_cache: HashMap<u32, u32>,
    /// 数据缓存
    pub data_cache: HashMap<u32, u32>,
    /// 性能统计
    pub stats: CPUStats,
}

/// CPU执行模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CPUMode {
    User,
    FIQ,
    IRQ,
    Supervisor,
    Abort,
    Undefined,
    System,
}

/// CPU性能统计
#[derive(Debug, Clone, Default)]
pub struct CPUStats {
    pub cycles: u64,
    pub instructions: u64,
    pub arm_instructions: u64,
    pub thumb_instructions: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub branch_taken: u64,
    pub branch_not_taken: u64,
}

/// ARM指令类型
#[derive(Debug, Clone, Copy)]
pub enum ARMInstruction {
    // 数据处理指令
    AND, EOR, SUB, RSB, ADD, ADC, SBC, RSC,
    TST, TEQ, CMP, CMN, ORR, MOV, BIC, MVN,
    
    // 分支指令
    B, BL, BX, BLX,
    
    // 加载/存储指令
    LDR, STR, LDM, STM, LDRB, STRB, LDRH, STRH,
    
    // 协处理器指令
    MRC, MCR, LDC, STC,
    
    // 软件中断
    SWI,
    
    // 未定义指令
    UNDEFINED,
}

/// Thumb指令类型
#[derive(Debug, Clone, Copy)]
pub enum ThumbInstruction {
    // 移动和比较指令
    MOV, CMP, ADD, SUB,
    
    // 逻辑指令
    AND, EOR, LSL, LSR, ASR, ADC, SBC, ROR,
    TST, NEG, CMP2, CMN, ORR, MUL, BIC, MVN,
    
    // 分支指令
    B, BL, BX, BLX,
    
    // 加载/存储指令
    LDR, STR, LDRB, STRB, LDRH, STRH, LDSB, LDSH, STRH2,
    
    // 栈操作
    PUSH, POP,
    
    // 未定义指令
    UNDEFINED,
}

impl ARM7TDMI {
    /// 创建新的ARM7TDMI CPU实例
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            pc: 0x08000000, // GBA ROM起始地址
            lr: 0,
            sp: 0x03007F00, // GBA栈起始地址
            cpsr: 0x1F, // 用户模式，Thumb模式关闭
            spsr: 0,
            mode: CPUMode::User,
            thumb_mode: false,
            instruction_cache: HashMap::new(),
            data_cache: HashMap::new(),
            stats: CPUStats::default(),
        }
    }
    
    /// 重置CPU到初始状态
    pub fn reset(&mut self) {
        self.registers = [0; 16];
        self.pc = 0x08000000;
        self.lr = 0;
        self.sp = 0x03007F00;
        self.cpsr = 0x1F;
        self.spsr = 0;
        self.mode = CPUMode::User;
        self.thumb_mode = false;
        self.instruction_cache.clear();
        self.data_cache.clear();
        self.stats = CPUStats::default();
    }
    
    /// 获取当前程序计数器
    pub fn get_pc(&self) -> u32 {
        self.pc
    }
    
    /// 设置程序计数器
    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }
    
    /// 获取寄存器值
    pub fn get_register(&self, reg: usize) -> u32 {
        if reg < 16 {
            self.registers[reg]
        } else {
            0
        }
    }
    
    /// 设置寄存器值
    pub fn set_register(&mut self, reg: usize, value: u32) {
        if reg < 16 {
            self.registers[reg] = value;
        }
    }
    
    /// 获取CPSR标志位
    pub fn get_flag(&self, flag: CPSRFlag) -> bool {
        match flag {
            CPSRFlag::Negative => (self.cpsr & 0x80000000) != 0,
            CPSRFlag::Zero => (self.cpsr & 0x40000000) != 0,
            CPSRFlag::Carry => (self.cpsr & 0x20000000) != 0,
            CPSRFlag::Overflow => (self.cpsr & 0x10000000) != 0,
            CPSRFlag::Thumb => (self.cpsr & 0x20) != 0,
            CPSRFlag::FIQDisable => (self.cpsr & 0x40) != 0,
            CPSRFlag::IRQDisable => (self.cpsr & 0x80) != 0,
        }
    }
    
    /// 设置CPSR标志位
    pub fn set_flag(&mut self, flag: CPSRFlag, value: bool) {
        let mask = match flag {
            CPSRFlag::Negative => 0x80000000,
            CPSRFlag::Zero => 0x40000000,
            CPSRFlag::Carry => 0x20000000,
            CPSRFlag::Overflow => 0x10000000,
            CPSRFlag::Thumb => 0x20,
            CPSRFlag::FIQDisable => 0x40,
            CPSRFlag::IRQDisable => 0x80,
        };
        
        if value {
            self.cpsr |= mask;
        } else {
            self.cpsr &= !mask;
        }
    }
    
    /// 切换到Thumb模式
    pub fn enter_thumb_mode(&mut self) {
        self.thumb_mode = true;
        self.set_flag(CPSRFlag::Thumb, true);
    }
    
    /// 切换到ARM模式
    pub fn enter_arm_mode(&mut self) {
        self.thumb_mode = false;
        self.set_flag(CPSRFlag::Thumb, false);
    }
    
    /// 执行一条指令
    pub fn execute_instruction(&mut self, memory: &mut GBAMemory) -> Result<(), String> {
        if self.thumb_mode {
            self.execute_thumb_instruction(memory)
        } else {
            self.execute_arm_instruction(memory)
        }
    }
    
    /// 执行ARM指令
    fn execute_arm_instruction(&mut self, memory: &mut GBAMemory) -> Result<(), String> {
        // 获取指令
        let instruction = memory.read_32(self.pc)?;
        
        // 解码指令
        let decoded = self.decode_arm_instruction(instruction);
        
        // 执行指令
        match decoded {
            ARMInstruction::MOV => {
                // MOV指令实现
                let rd = ((instruction >> 12) & 0xF) as usize;
                let operand2 = self.get_operand2(instruction);
                self.set_register(rd, operand2);
                self.pc += 4;
            }
            ARMInstruction::ADD => {
                // ADD指令实现
                let rd = ((instruction >> 12) & 0xF) as usize;
                let rn = ((instruction >> 16) & 0xF) as usize;
                let operand2 = self.get_operand2(instruction);
                let result = self.get_register(rn).wrapping_add(operand2);
                self.set_register(rd, result);
                self.update_flags_add(self.get_register(rn), operand2, result);
                self.pc += 4;
            }
            ARMInstruction::SUB => {
                // SUB指令实现
                let rd = ((instruction >> 12) & 0xF) as usize;
                let rn = ((instruction >> 16) & 0xF) as usize;
                let operand2 = self.get_operand2(instruction);
                let result = self.get_register(rn).wrapping_sub(operand2);
                self.set_register(rd, result);
                self.update_flags_sub(self.get_register(rn), operand2, result);
                self.pc += 4;
            }
            ARMInstruction::B => {
                // 分支指令实现
                let offset = (instruction & 0xFFFFFF) as i32;
                let offset = if (offset & 0x800000) != 0 {
                    offset | 0xFF000000u32 as i32
                } else {
                    offset
                };
                self.pc = self.pc.wrapping_add((offset << 2) as u32);
            }
            ARMInstruction::LDR => {
                // 加载指令实现
                let rt = ((instruction >> 12) & 0xF) as usize;
                let rn = ((instruction >> 16) & 0xF) as usize;
                let offset = instruction & 0xFFF;
                let address = self.get_register(rn).wrapping_add(offset);
                let value = memory.read_32(address)?;
                self.set_register(rt, value);
                self.pc += 4;
            }
            ARMInstruction::STR => {
                // 存储指令实现
                let rt = ((instruction >> 12) & 0xF) as usize;
                let rn = ((instruction >> 16) & 0xF) as usize;
                let offset = instruction & 0xFFF;
                let address = self.get_register(rn).wrapping_add(offset);
                let value = self.get_register(rt);
                memory.write_32(address, value)?;
                self.pc += 4;
            }
            _ => {
                // 未实现指令
                self.pc += 4;
            }
        }
        
        self.stats.cycles += 1;
        self.stats.instructions += 1;
        self.stats.arm_instructions += 1;
        
        Ok(())
    }
    
    /// 执行Thumb指令
    fn execute_thumb_instruction(&mut self, memory: &mut GBAMemory) -> Result<(), String> {
        // 获取指令
        let instruction = memory.read_16(self.pc)?;
        
        // 解码指令
        let decoded = self.decode_thumb_instruction(instruction);
        
        // 执行指令
        match decoded {
            ThumbInstruction::MOV => {
                // Thumb MOV指令实现
                let rd = ((instruction >> 8) & 0x7) as usize;
                let rs = ((instruction >> 3) & 0x7) as usize;
                let value = self.get_register(rs);
                self.set_register(rd, value);
                self.pc += 2;
            }
            ThumbInstruction::ADD => {
                // Thumb ADD指令实现
                let rd = ((instruction >> 8) & 0x7) as usize;
                let rs = ((instruction >> 3) & 0x7) as usize;
                let value = self.get_register(rs);
                let result = self.get_register(rd).wrapping_add(value);
                self.set_register(rd, result);
                self.pc += 2;
            }
            ThumbInstruction::B => {
                // Thumb分支指令实现
                let offset = (instruction & 0xFF) as i32;
                let offset = if (offset & 0x80) != 0 {
                    offset | 0xFFFFFF00u32 as i32
                } else {
                    offset
                };
                self.pc = self.pc.wrapping_add((offset << 1) as u32);
            }
            _ => {
                // 未实现指令
                self.pc += 2;
            }
        }
        
        self.stats.cycles += 1;
        self.stats.instructions += 1;
        self.stats.thumb_instructions += 1;
        
        Ok(())
    }
    
    /// 解码ARM指令
    fn decode_arm_instruction(&self, instruction: u32) -> ARMInstruction {
        let opcode = (instruction >> 21) & 0xF;
        let condition = (instruction >> 28) & 0xF;
        
        match opcode {
            0x0 => ARMInstruction::AND,
            0x1 => ARMInstruction::EOR,
            0x2 => ARMInstruction::SUB,
            0x3 => ARMInstruction::RSB,
            0x4 => ARMInstruction::ADD,
            0x5 => ARMInstruction::ADC,
            0x6 => ARMInstruction::SBC,
            0x7 => ARMInstruction::RSC,
            0x8 => ARMInstruction::TST,
            0x9 => ARMInstruction::TEQ,
            0xA => ARMInstruction::CMP,
            0xB => ARMInstruction::CMN,
            0xC => ARMInstruction::ORR,
            0xD => ARMInstruction::MOV,
            0xE => ARMInstruction::BIC,
            0xF => ARMInstruction::MVN,
            _ => ARMInstruction::UNDEFINED,
        }
    }
    
    /// 解码Thumb指令
    fn decode_thumb_instruction(&self, instruction: u16) -> ThumbInstruction {
        let opcode = (instruction >> 10) & 0x3F;
        
        match opcode {
            0x00..=0x07 => ThumbInstruction::MOV,
            0x08..=0x0F => ThumbInstruction::CMP,
            0x10..=0x17 => ThumbInstruction::ADD,
            0x18..=0x1F => ThumbInstruction::SUB,
            0x20..=0x27 => ThumbInstruction::AND,
            0x28..=0x2F => ThumbInstruction::EOR,
            0x30..=0x37 => ThumbInstruction::LSL,
            0x38..=0x3F => ThumbInstruction::LSR,
            _ => ThumbInstruction::UNDEFINED,
        }
    }
    
    /// 获取操作数2
    fn get_operand2(&self, instruction: u32) -> u32 {
        let immediate = (instruction & 0xFF) as u32;
        let rotate = ((instruction >> 8) & 0xF) as u32;
        immediate.rotate_right(rotate * 2)
    }
    
    /// 更新加法标志位
    fn update_flags_add(&mut self, a: u32, b: u32, result: u32) {
        self.set_flag(CPSRFlag::Zero, result == 0);
        self.set_flag(CPSRFlag::Negative, (result & 0x80000000) != 0);
        self.set_flag(CPSRFlag::Carry, result < a);
        self.set_flag(CPSRFlag::Overflow, 
            ((a ^ result) & (b ^ result) & 0x80000000) != 0);
    }
    
    /// 更新减法标志位
    fn update_flags_sub(&mut self, a: u32, b: u32, result: u32) {
        self.set_flag(CPSRFlag::Zero, result == 0);
        self.set_flag(CPSRFlag::Negative, (result & 0x80000000) != 0);
        self.set_flag(CPSRFlag::Carry, a >= b);
        self.set_flag(CPSRFlag::Overflow, 
            ((a ^ b) & (a ^ result) & 0x80000000) != 0);
    }
    
    /// 获取性能统计
    pub fn get_stats(&self) -> &CPUStats {
        &self.stats
    }
}

/// CPSR标志位
#[derive(Debug, Clone, Copy)]
pub enum CPSRFlag {
    Negative,
    Zero,
    Carry,
    Overflow,
    Thumb,
    FIQDisable,
    IRQDisable,
}

/// GBA内存管理单元
#[derive(Debug, Clone)]
pub struct GBAMemory {
    /// 内部工作RAM (32KB)
    pub iwram: [u8; 0x8000],
    /// 外部工作RAM (256KB)
    pub ewram: [u8; 0x40000],
    /// 调色板RAM (1KB)
    pub palette_ram: [u8; 0x400],
    /// VRAM (96KB)
    pub vram: [u8; 0x18000],
    /// OAM RAM (1KB)
    pub oam_ram: [u8; 0x400],
    /// ROM数据
    pub rom: Vec<u8>,
    /// 性能统计
    pub stats: MemoryStats,
}

/// 内存性能统计
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub reads: u64,
    pub writes: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl GBAMemory {
    /// 创建新的GBA内存实例
    pub fn new() -> Self {
        Self {
            iwram: [0; 0x8000],
            ewram: [0; 0x40000],
            palette_ram: [0; 0x400],
            vram: [0; 0x18000],
            oam_ram: [0; 0x400],
            rom: Vec::new(),
            stats: MemoryStats::default(),
        }
    }
    
    /// 加载ROM数据
    pub fn load_rom(&mut self, rom_data: Vec<u8>) {
        self.rom = rom_data;
    }
    
    /// 读取8位数据
    pub fn read_8(&mut self, address: u32) -> Result<u8, String> {
        self.stats.reads += 1;
        
        match address {
            0x00000000..=0x00FFFFFF => {
                // ROM区域
                let rom_addr = (address - 0x08000000) as usize;
                if rom_addr < self.rom.len() {
                    Ok(self.rom[rom_addr])
                } else {
                    Ok(0)
                }
            }
            0x02000000..=0x0203FFFF => {
                // 外部工作RAM
                let ram_addr = (address - 0x02000000) as usize;
                if ram_addr < self.ewram.len() {
                    Ok(self.ewram[ram_addr])
                } else {
                    Ok(0)
                }
            }
            0x03000000..=0x03007FFF => {
                // 内部工作RAM
                let ram_addr = (address - 0x03000000) as usize;
                if ram_addr < self.iwram.len() {
                    Ok(self.iwram[ram_addr])
                } else {
                    Ok(0)
                }
            }
            0x05000000..=0x050003FF => {
                // 调色板RAM
                let pal_addr = (address - 0x05000000) as usize;
                if pal_addr < self.palette_ram.len() {
                    Ok(self.palette_ram[pal_addr])
                } else {
                    Ok(0)
                }
            }
            0x06000000..=0x06017FFF => {
                // VRAM
                let vram_addr = (address - 0x06000000) as usize;
                if vram_addr < self.vram.len() {
                    Ok(self.vram[vram_addr])
                } else {
                    Ok(0)
                }
            }
            0x07000000..=0x070003FF => {
                // OAM RAM
                let oam_addr = (address - 0x07000000) as usize;
                if oam_addr < self.oam_ram.len() {
                    Ok(self.oam_ram[oam_addr])
                } else {
                    Ok(0)
                }
            }
            _ => Ok(0),
        }
    }
    
    /// 读取16位数据
    pub fn read_16(&mut self, address: u32) -> Result<u16, String> {
        let low = self.read_8(address)?;
        let high = self.read_8(address + 1)?;
        Ok((high as u16) << 8 | low as u16)
    }
    
    /// 读取32位数据
    pub fn read_32(&mut self, address: u32) -> Result<u32, String> {
        let low = self.read_16(address)?;
        let high = self.read_16(address + 2)?;
        Ok((high as u32) << 16 | low as u32)
    }
    
    /// 写入8位数据
    pub fn write_8(&mut self, address: u32, value: u8) -> Result<(), String> {
        self.stats.writes += 1;
        
        match address {
            0x02000000..=0x0203FFFF => {
                // 外部工作RAM
                let ram_addr = (address - 0x02000000) as usize;
                if ram_addr < self.ewram.len() {
                    self.ewram[ram_addr] = value;
                }
            }
            0x03000000..=0x03007FFF => {
                // 内部工作RAM
                let ram_addr = (address - 0x03000000) as usize;
                if ram_addr < self.iwram.len() {
                    self.iwram[ram_addr] = value;
                }
            }
            0x05000000..=0x050003FF => {
                // 调色板RAM
                let pal_addr = (address - 0x05000000) as usize;
                if pal_addr < self.palette_ram.len() {
                    self.palette_ram[pal_addr] = value;
                }
            }
            0x06000000..=0x06017FFF => {
                // VRAM
                let vram_addr = (address - 0x06000000) as usize;
                if vram_addr < self.vram.len() {
                    self.vram[vram_addr] = value;
                }
            }
            0x07000000..=0x070003FF => {
                // OAM RAM
                let oam_addr = (address - 0x07000000) as usize;
                if oam_addr < self.oam_ram.len() {
                    self.oam_ram[oam_addr] = value;
                }
            }
            _ => {
                // 只读区域，忽略写入
            }
        }
        
        Ok(())
    }
    
    /// 写入16位数据
    pub fn write_16(&mut self, address: u32, value: u16) -> Result<(), String> {
        self.write_8(address, value as u8)?;
        self.write_8(address + 1, (value >> 8) as u8)?;
        Ok(())
    }
    
    /// 写入32位数据
    pub fn write_32(&mut self, address: u32, value: u32) -> Result<(), String> {
        self.write_16(address, value as u16)?;
        self.write_16(address + 2, (value >> 16) as u16)?;
        Ok(())
    }
    
    /// 获取内存统计
    pub fn get_stats(&self) -> &MemoryStats {
        &self.stats
    }
}

impl Default for ARM7TDMI {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GBAMemory {
    fn default() -> Self {
        Self::new()
    }
}
