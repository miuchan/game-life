//! ROM生成器模块 - 生成Game Boy兼容的ROM文件

use std::fs::File;
use std::io::Write;

/// Game Boy ROM头部结构
#[derive(Debug, Clone)]
pub struct RomHeader {
    /// Nintendo Logo (0x104-0x133)
    pub nintendo_logo: [u8; 48],
    /// 游戏标题 (0x134-0x143)
    pub title: [u8; 16],
    /// 制造商代码 (0x144-0x145)
    pub manufacturer_code: [u8; 2],
    /// CGB标志 (0x146)
    pub cgb_flag: u8,
    /// 新许可证代码 (0x144-0x145)
    pub new_licensee_code: [u8; 2],
    /// SGB标志 (0x146)
    pub sgb_flag: u8,
    /// 卡带类型 (0x147)
    pub cartridge_type: u8,
    /// ROM大小 (0x148)
    pub rom_size: u8,
    /// RAM大小 (0x149)
    pub ram_size: u8,
    /// 目标市场 (0x14A)
    pub destination_code: u8,
    /// 旧许可证代码 (0x14B)
    pub old_licensee_code: u8,
    /// ROM版本号 (0x14C)
    pub rom_version: u8,
    /// 头部校验和 (0x14D)
    pub header_checksum: u8,
    /// 全局校验和 (0x14E-0x14F)
    pub global_checksum: u16,
}

impl Default for RomHeader {
    fn default() -> Self {
        Self {
            // Nintendo Logo - 必须完全匹配
            nintendo_logo: [
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
            ],
            title: [0; 16],
            manufacturer_code: [0; 2],
            cgb_flag: 0x00, // 非CGB游戏
            new_licensee_code: [0; 2],
            sgb_flag: 0x00, // 非SGB游戏
            cartridge_type: 0x00, // ROM only
            rom_size: 0x00, // 32KB ROM
            ram_size: 0x00, // 无RAM
            destination_code: 0x00, // 日本
            old_licensee_code: 0x00,
            rom_version: 0x00,
            header_checksum: 0x00,
            global_checksum: 0x0000,
        }
    }
}

impl RomHeader {
    /// 创建新的ROM头部
    pub fn new(title: &str) -> Self {
        let mut header = Self::default();
        
        // 设置游戏标题
        let title_bytes = title.as_bytes();
        let title_len = title_bytes.len().min(16);
        header.title[..title_len].copy_from_slice(&title_bytes[..title_len]);
        
        header
    }

    /// 计算头部校验和
    pub fn calculate_header_checksum(&self) -> u8 {
        let mut checksum = 0u8;
        
        // 计算0x134-0x14C的校验和
        for &byte in &self.title {
            checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
        }
        for &byte in &self.manufacturer_code {
            checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
        }
        checksum = checksum.wrapping_sub(self.cgb_flag).wrapping_sub(1);
        for &byte in &self.new_licensee_code {
            checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
        }
        checksum = checksum.wrapping_sub(self.sgb_flag).wrapping_sub(1);
        checksum = checksum.wrapping_sub(self.cartridge_type).wrapping_sub(1);
        checksum = checksum.wrapping_sub(self.rom_size).wrapping_sub(1);
        checksum = checksum.wrapping_sub(self.ram_size).wrapping_sub(1);
        checksum = checksum.wrapping_sub(self.destination_code).wrapping_sub(1);
        checksum = checksum.wrapping_sub(self.old_licensee_code).wrapping_sub(1);
        checksum = checksum.wrapping_sub(self.rom_version).wrapping_sub(1);
        
        checksum
    }

    /// 计算全局校验和
    pub fn calculate_global_checksum(&self, rom_data: &[u8]) -> u16 {
        let mut checksum = 0u16;
        
        for &byte in rom_data {
            checksum = checksum.wrapping_add(byte as u16);
        }
        
        checksum
    }

    /// 将头部写入字节数组
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; 0x150];
        
        // 0x100-0x103: 跳转到0x150
        bytes[0x100] = 0x00; // NOP
        bytes[0x101] = 0xC3; // JP
        bytes[0x102] = 0x50; // 0x0150的低字节
        bytes[0x103] = 0x01; // 0x0150的高字节
        
        // 0x104-0x133: Nintendo Logo
        bytes[0x104..0x134].copy_from_slice(&self.nintendo_logo);
        
        // 0x134-0x143: 游戏标题
        bytes[0x134..0x144].copy_from_slice(&self.title);
        
        // 0x144-0x145: 制造商代码
        bytes[0x144..0x146].copy_from_slice(&self.manufacturer_code);
        
        // 0x146: CGB标志
        bytes[0x146] = self.cgb_flag;
        
        // 0x147-0x148: 新许可证代码
        bytes[0x147..0x149].copy_from_slice(&self.new_licensee_code);
        
        // 0x149: SGB标志
        bytes[0x149] = self.sgb_flag;
        
        // 0x14A: 卡带类型
        bytes[0x14A] = self.cartridge_type;
        
        // 0x14B: ROM大小
        bytes[0x14B] = self.rom_size;
        
        // 0x14C: RAM大小
        bytes[0x14C] = self.ram_size;
        
        // 0x14D: 目标市场
        bytes[0x14D] = self.destination_code;
        
        // 0x14E: 旧许可证代码
        bytes[0x14E] = self.old_licensee_code;
        
        // 0x14F: ROM版本号
        bytes[0x14F] = self.rom_version;
        
        // 0x150: 头部校验和
        if bytes.len() > 0x150 {
            bytes[0x150] = self.header_checksum;
        }
        
        // 0x151-0x152: 全局校验和
        if bytes.len() > 0x152 {
            bytes[0x151] = (self.global_checksum & 0xFF) as u8;
            bytes[0x152] = ((self.global_checksum >> 8) & 0xFF) as u8;
        }
        
        bytes
    }
}

/// ROM生成器
pub struct RomGenerator {
    header: RomHeader,
    program_data: Vec<u8>,
}

impl RomGenerator {
    /// 创建新的ROM生成器
    pub fn new(title: &str) -> Self {
        Self {
            header: RomHeader::new(title),
            program_data: Vec::new(),
        }
    }

    /// 添加程序数据
    pub fn add_program(&mut self, start_address: u16, program: &[u8]) {
        // 确保有足够的空间
        let end_address = start_address + program.len() as u16;
        if end_address as usize > self.program_data.len() {
            self.program_data.resize(end_address as usize, 0x00);
        }
        
        // 复制程序数据
        self.program_data[start_address as usize..end_address as usize]
            .copy_from_slice(program);
    }

    /// 生成ROM文件
    pub fn generate_rom(&mut self) -> Vec<u8> {
        // 计算校验和
        self.header.header_checksum = self.header.calculate_header_checksum();
        
        // 生成头部
        let mut rom_data = self.header.to_bytes();
        
        // 添加程序数据
        rom_data.extend_from_slice(&self.program_data);
        
        // 确保ROM大小是32KB的倍数
        let target_size = 32 * 1024; // 32KB
        if rom_data.len() < target_size {
            rom_data.resize(target_size, 0xFF); // 用0xFF填充
        }
        
        // 计算全局校验和
        self.header.global_checksum = self.header.calculate_global_checksum(&rom_data);
        
        // 更新头部中的全局校验和
        let checksum_bytes = self.header.global_checksum.to_le_bytes();
        rom_data[0x151] = checksum_bytes[0];
        rom_data[0x152] = checksum_bytes[1];
        
        rom_data
    }

    /// 保存ROM文件
    pub fn save_rom(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let rom_data = self.generate_rom();
        let mut file = File::create(filename)?;
        file.write_all(&rom_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom_header_creation() {
        let header = RomHeader::new("TEST GAME");
        assert_eq!(header.title[0..9], b"TEST GAME"[..]);
    }

    #[test]
    fn test_rom_generator() {
        let mut generator = RomGenerator::new("TEST ROM");
        generator.add_program(0x150, &[0x00, 0x01, 0x02]);
        
        let rom_data = generator.generate_rom();
        assert_eq!(rom_data.len(), 32 * 1024); // 32KB
        assert_eq!(rom_data[0x150], 0x00);
        assert_eq!(rom_data[0x151], 0x01);
        assert_eq!(rom_data[0x152], 0x02);
    }
}
