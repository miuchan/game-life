//! 瓦片图模块

/// 瓦片图
#[derive(Debug, Clone)]
pub struct TileMap {
    pub tiles: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl TileMap {
    /// 创建新的瓦片图
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            tiles: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    /// 获取瓦片
    pub fn get_tile(&self, x: u16, y: u16) -> u8 {
        if x < self.width && y < self.height {
            self.tiles[(y * self.width + x) as usize]
        } else {
            0
        }
    }

    /// 设置瓦片
    pub fn set_tile(&mut self, x: u16, y: u16, tile: u8) {
        if x < self.width && y < self.height {
            self.tiles[(y * self.width + x) as usize] = tile;
        }
    }
}
