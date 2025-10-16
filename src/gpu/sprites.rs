//! 精灵模块

/// 精灵
#[derive(Debug, Clone)]
pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub tile_index: u8,
    pub palette: u8,
    pub x_flip: bool,
    pub y_flip: bool,
    pub priority: bool,
}

impl Sprite {
    /// 创建新的精灵
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            tile_index: 0,
            palette: 0,
            x_flip: false,
            y_flip: false,
            priority: false,
        }
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self::new()
    }
}
