//! GBA图形处理单元 (GPU) 实现
//! 
//! 这个模块实现了Game Boy Advance的图形处理单元，
//! 包括背景层、精灵、调色板等功能

use crate::gba::cpu::GBAMemory;

/// GBA GPU状态
#[derive(Debug, Clone)]
pub struct GBAGPU {
    /// 显示控制寄存器
    pub dispcnt: u16,
    /// 绿色交换寄存器
    pub green_swap: u16,
    /// 显示状态寄存器
    pub dispstat: u16,
    /// V计数寄存器
    pub vcount: u16,
    /// 背景控制寄存器
    pub bgcnt: [u16; 4],
    /// 背景滚动寄存器
    pub bgofs: [u16; 4],
    /// 精灵属性内存
    pub oam: [u16; 0x200],
    /// 调色板内存
    pub palette: [u16; 0x200],
    /// VRAM内存
    pub vram: [u8; 0x18000],
    /// 当前扫描线
    pub current_scanline: u16,
    /// 帧计数器
    pub frame_count: u32,
    /// 性能统计
    pub stats: GPUStats,
}

/// GPU性能统计
#[derive(Debug, Clone, Default)]
pub struct GPUStats {
    pub frames_rendered: u32,
    pub pixels_drawn: u64,
    pub sprites_rendered: u64,
    pub backgrounds_rendered: u64,
    pub vblank_count: u32,
    pub hblank_count: u32,
}

/// 显示模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Mode0, // 4个背景层
    Mode1, // 3个背景层
    Mode2, // 2个背景层
    Mode3, // 位图模式
    Mode4, // 位图模式 (调色板)
    Mode5, // 位图模式 (扩展)
}

/// 背景层类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackgroundType {
    Text,      // 文本背景
    Affine,    // 仿射变换背景
    Bitmap,    // 位图背景
    Disabled,  // 禁用
}

/// 精灵属性
#[derive(Debug, Clone, Copy)]
pub struct SpriteAttribute {
    pub attr0: u16, // 属性0
    pub attr1: u16, // 属性1
    pub attr2: u16, // 属性2
    pub attr3: u16, // 属性3
}

/// 调色板颜色
#[derive(Debug, Clone, Copy)]
pub struct PaletteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl GBAGPU {
    /// 创建新的GBA GPU实例
    pub fn new() -> Self {
        Self {
            dispcnt: 0x0000,
            green_swap: 0x0000,
            dispstat: 0x0000,
            vcount: 0x0000,
            bgcnt: [0; 4],
            bgofs: [0; 4],
            oam: [0; 0x200],
            palette: [0; 0x200],
            vram: [0; 0x18000],
            current_scanline: 0,
            frame_count: 0,
            stats: GPUStats::default(),
        }
    }
    
    /// 重置GPU到初始状态
    pub fn reset(&mut self) {
        self.dispcnt = 0x0000;
        self.green_swap = 0x0000;
        self.dispstat = 0x0000;
        self.vcount = 0x0000;
        self.bgcnt = [0; 4];
        self.bgofs = [0; 4];
        self.oam = [0; 0x200];
        self.palette = [0; 0x200];
        self.vram = [0; 0x18000];
        self.current_scanline = 0;
        self.frame_count = 0;
        self.stats = GPUStats::default();
    }
    
    /// 获取显示模式
    pub fn get_display_mode(&self) -> DisplayMode {
        match self.dispcnt & 0x7 {
            0 => DisplayMode::Mode0,
            1 => DisplayMode::Mode1,
            2 => DisplayMode::Mode2,
            3 => DisplayMode::Mode3,
            4 => DisplayMode::Mode4,
            5 => DisplayMode::Mode5,
            _ => DisplayMode::Mode0,
        }
    }
    
    /// 检查背景层是否启用
    pub fn is_background_enabled(&self, bg: usize) -> bool {
        if bg >= 4 {
            return false;
        }
        
        match self.get_display_mode() {
            DisplayMode::Mode0 => bg < 4,
            DisplayMode::Mode1 => bg < 3,
            DisplayMode::Mode2 => bg < 2,
            DisplayMode::Mode3 | DisplayMode::Mode4 | DisplayMode::Mode5 => false,
        }
    }
    
    /// 获取背景层类型
    pub fn get_background_type(&self, bg: usize) -> BackgroundType {
        if bg >= 4 {
            return BackgroundType::Disabled;
        }
        
        match self.get_display_mode() {
            DisplayMode::Mode0 | DisplayMode::Mode1 | DisplayMode::Mode2 => {
                BackgroundType::Text
            }
            DisplayMode::Mode3 | DisplayMode::Mode4 | DisplayMode::Mode5 => {
                BackgroundType::Bitmap
            }
        }
    }
    
    /// 渲染当前扫描线
    pub fn render_scanline(&mut self, memory: &mut GBAMemory) -> Result<(), String> {
        let mut scanline_buffer = [0u16; 240]; // 240像素宽
        
        // 渲染背景层
        for bg in 0..4 {
            if self.is_background_enabled(bg) {
                self.render_background_scanline(bg, &mut scanline_buffer, memory)?;
            }
        }
        
        // 渲染精灵
        self.render_sprites_scanline(&mut scanline_buffer, memory)?;
        
        // 更新统计
        self.stats.pixels_drawn += 240;
        self.stats.backgrounds_rendered += 1;
        
        Ok(())
    }
    
    /// 渲染背景层扫描线
    fn render_background_scanline(&self, bg: usize, buffer: &mut [u16; 240], memory: &mut GBAMemory) -> Result<(), String> {
        let bg_type = self.get_background_type(bg);
        
        match bg_type {
            BackgroundType::Text => {
                self.render_text_background_scanline(bg, buffer, memory)?;
            }
            BackgroundType::Bitmap => {
                self.render_bitmap_background_scanline(buffer, memory)?;
            }
            BackgroundType::Affine => {
                self.render_affine_background_scanline(bg, buffer, memory)?;
            }
            BackgroundType::Disabled => {
                // 背景层禁用，不渲染
            }
        }
        
        Ok(())
    }
    
    /// 渲染文本背景层扫描线
    fn render_text_background_scanline(&self, bg: usize, buffer: &mut [u16; 240], memory: &mut GBAMemory) -> Result<(), String> {
        let bgcnt = self.bgcnt[bg];
        let bgofs_x = self.bgofs[bg] & 0x1FF;
        let bgofs_y = (self.bgofs[bg] >> 8) & 0x1FF;
        
        // 获取背景层参数
        let screen_size = (bgcnt >> 14) & 0x3;
        let char_base = (bgcnt >> 2) & 0xF;
        let screen_base = (bgcnt >> 8) & 0x1F;
        let palette_mode = (bgcnt >> 13) & 0x1;
        
        // 计算屏幕尺寸
        let screen_width = match screen_size {
            0 => 32,
            1 => 64,
            2 => 32,
            3 => 64,
            _ => 32,
        };
        
        let screen_height = match screen_size {
            0 => 32,
            1 => 32,
            2 => 64,
            3 => 64,
            _ => 32,
        };
        
        // 渲染扫描线
        for x in 0..240 {
            let screen_x = (x + bgofs_x as usize) % (screen_width * 8);
            let screen_y = (self.current_scanline as usize + bgofs_y as usize) % (screen_height * 8);
            
            let tile_x = screen_x / 8;
            let tile_y = screen_y / 8;
            let pixel_x = screen_x % 8;
            let pixel_y = screen_y % 8;
            
            // 计算屏幕基址
            let screen_addr = 0x06000000 + (screen_base as u32 * 0x800) + ((tile_y * screen_width + tile_x) as u32 * 2);
            
            // 读取瓦片数据
            let tile_data = memory.read_16(screen_addr)?;
            let tile_index = tile_data & 0x3FF;
            let h_flip = (tile_data >> 10) & 0x1;
            let v_flip = (tile_data >> 11) & 0x1;
            let palette_index = (tile_data >> 12) & 0xF;
            
            // 计算瓦片基址
            let tile_addr = 0x06000000 + (char_base as u32 * 0x4000) + (tile_index as u32 * 32);
            
            // 计算像素坐标
            let final_pixel_x = if h_flip != 0 { 7 - pixel_x } else { pixel_x };
            let final_pixel_y = if v_flip != 0 { 7 - pixel_y } else { pixel_y };
            
            // 读取像素数据
            let pixel_addr = tile_addr + (final_pixel_y * 4 + final_pixel_x / 2) as u32;
            let pixel_data = memory.read_8(pixel_addr)?;
            
            let color_index = if pixel_x % 2 == 0 {
                pixel_data & 0xF
            } else {
                (pixel_data >> 4) & 0xF
            };
            
            if color_index != 0 {
                // 计算调色板地址
                let palette_addr = 0x05000000 + (palette_index as u32 * 32) + (color_index as u32 * 2);
                let color = memory.read_16(palette_addr)?;
                
                buffer[x] = color;
            }
        }
        
        Ok(())
    }
    
    /// 渲染位图背景层扫描线
    fn render_bitmap_background_scanline(&self, buffer: &mut [u16; 240], memory: &mut GBAMemory) -> Result<(), String> {
        let mode = self.get_display_mode();
        
        match mode {
            DisplayMode::Mode3 => {
                // Mode 3: 16位直接颜色
                for x in 0..240 {
                    let pixel_addr = 0x06000000 + (self.current_scanline as u32 * 240 + x as u32) * 2;
                    let color = memory.read_16(pixel_addr)?;
                    buffer[x] = color;
                }
            }
            DisplayMode::Mode4 => {
                // Mode 4: 8位调色板模式
                for x in 0..240 {
                    let pixel_addr = 0x06000000 + (self.current_scanline as u32 * 240 + x as u32);
                    let color_index = memory.read_8(pixel_addr)?;
                    
                    if color_index != 0 {
                        let palette_addr = 0x05000000 + (color_index as u32 * 2);
                        let color = memory.read_16(palette_addr)?;
                        buffer[x] = color;
                    }
                }
            }
            DisplayMode::Mode5 => {
                // Mode 5: 16位直接颜色 (160x128)
                for x in 0..160 {
                    let pixel_addr = 0x06000000 + (self.current_scanline as u32 * 160 + x as u32) * 2;
                    let color = memory.read_16(pixel_addr)?;
                    buffer[x] = color;
                }
            }
            _ => {
                // 其他模式不支持位图背景
            }
        }
        
        Ok(())
    }
    
    /// 渲染仿射变换背景层扫描线
    fn render_affine_background_scanline(&self, bg: usize, buffer: &mut [u16; 240], memory: &mut GBAMemory) -> Result<(), String> {
        // 仿射变换背景层实现
        // 这里简化实现，实际需要矩阵变换
        for x in 0..240 {
            // 简化的仿射变换
            let transformed_x = x as f32 * 1.0;
            let transformed_y = self.current_scanline as f32 * 1.0;
            
            // 读取像素数据
            let pixel_addr = 0x06000000 + (transformed_y as u32 * 240 + transformed_x as u32) * 2;
            let color = memory.read_16(pixel_addr)?;
            buffer[x] = color;
        }
        
        Ok(())
    }
    
    /// 渲染精灵扫描线
    fn render_sprites_scanline(&self, buffer: &mut [u16; 240], memory: &mut GBAMemory) -> Result<(), String> {
        // 遍历所有精灵
        for sprite_index in 0..128 {
            let sprite = self.get_sprite(sprite_index);
            
            // 检查精灵是否在当前扫描线
            if self.is_sprite_on_scanline(sprite) {
                self.render_sprite_scanline(sprite, buffer, memory)?;
            }
        }
        
        Ok(())
    }
    
    /// 获取精灵属性
    fn get_sprite(&self, index: usize) -> SpriteAttribute {
        let base_addr = index * 4;
        SpriteAttribute {
            attr0: self.oam[base_addr],
            attr1: self.oam[base_addr + 1],
            attr2: self.oam[base_addr + 2],
            attr3: self.oam[base_addr + 3],
        }
    }
    
    /// 检查精灵是否在当前扫描线
    fn is_sprite_on_scanline(&self, sprite: SpriteAttribute) -> bool {
        let y = sprite.attr0 & 0xFF;
        let height = self.get_sprite_height(sprite);
        
        y <= self.current_scanline && self.current_scanline < y + height
    }
    
    /// 获取精灵高度
    fn get_sprite_height(&self, sprite: SpriteAttribute) -> u16 {
        let size = (sprite.attr1 >> 14) & 0x3;
        match size {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 64,
            _ => 8,
        }
    }
    
    /// 渲染精灵扫描线
    fn render_sprite_scanline(&self, sprite: SpriteAttribute, buffer: &mut [u16; 240], memory: &mut GBAMemory) -> Result<(), String> {
        let x = sprite.attr1 & 0x1FF;
        let y = sprite.attr0 & 0xFF;
        let tile_index = sprite.attr2 & 0x3FF;
        let palette_index = (sprite.attr2 >> 12) & 0xF;
        
        // 计算精灵在扫描线中的位置
        let scanline_y = self.current_scanline - y;
        
        // 计算瓦片地址
        let tile_addr = 0x06010000 + (tile_index as u32 * 32) + (scanline_y as u32 * 4);
        
        // 渲染精灵像素
        for pixel_x in 0..8 {
            let pixel_addr = tile_addr + (pixel_x / 2) as u32;
            let pixel_data = memory.read_8(pixel_addr)?;
            
            let color_index = if pixel_x % 2 == 0 {
                pixel_data & 0xF
            } else {
                (pixel_data >> 4) & 0xF
            };
            
            if color_index != 0 {
                let buffer_x = x as usize + pixel_x as usize;
                if buffer_x < 240 {
                    let palette_addr = 0x05000200 + (palette_index as u32 * 32) + (color_index as u32 * 2);
                    let color = memory.read_16(palette_addr)?;
                    buffer[buffer_x] = color;
                }
            }
        }
        
        Ok(())
    }
    
    /// 更新GPU状态
    pub fn update(&mut self) {
        self.current_scanline += 1;
        
        if self.current_scanline >= 160 {
            // VBlank开始
            self.current_scanline = 0;
            self.frame_count += 1;
            self.stats.frames_rendered += 1;
            self.stats.vblank_count += 1;
        } else if self.current_scanline >= 144 {
            // VBlank期间
            self.stats.vblank_count += 1;
        } else {
            // 可见扫描线
            self.stats.hblank_count += 1;
        }
        
        self.vcount = self.current_scanline;
    }
    
    /// 获取GPU统计
    pub fn get_stats(&self) -> &GPUStats {
        &self.stats
    }
}

impl Default for GBAGPU {
    fn default() -> Self {
        Self::new()
    }
}
