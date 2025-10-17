//! GBA俄罗斯方块实现
//! 
//! 将俄罗斯方块游戏适配到GBA模拟器上运行

use crate::gba::{GBASystem, GBAState};
use crate::games::tetris::tetris_game::{TetrisGame, GameState, Tetromino, Color};
use std::time::{Duration, Instant};

/// GBA俄罗斯方块游戏
#[derive(Debug)]
pub struct GBATetris {
    /// GBA模拟器
    pub gba: GBASystem,
    /// 俄罗斯方块游戏逻辑
    pub tetris: TetrisGame,
    /// 渲染缓冲区
    pub render_buffer: Vec<Vec<u16>>,
    /// 输入状态
    pub input_state: InputState,
    /// 上次输入时间
    pub last_input_time: Instant,
    /// 输入重复间隔
    pub input_repeat_interval: Duration,
}

/// 输入状态
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub rotate: bool,
    pub hard_drop: bool,
    pub pause: bool,
}

/// GBA俄罗斯方块实现
impl GBATetris {
    /// 创建新的GBA俄罗斯方块游戏
    pub fn new() -> Self {
        let mut gba = GBASystem::new();
        let tetris = TetrisGame::new();
        
        // 创建GBA ROM数据
        let rom_data = Self::create_tetris_rom();
        gba.load_rom(rom_data).unwrap();
        gba.start().unwrap();
        
        Self {
            gba,
            tetris,
            render_buffer: vec![vec![0; 240]; 160], // GBA屏幕分辨率
            input_state: InputState::default(),
            last_input_time: Instant::now(),
            input_repeat_interval: Duration::from_millis(150),
        }
    }
    
    /// 创建俄罗斯方块ROM数据
    fn create_tetris_rom() -> Vec<u8> {
        let mut rom = Vec::new();
        
        // ROM头
        rom.extend_from_slice(&[
            0x24, 0xFF, 0xAE, 0x51, 0x69, 0x9A, 0xA2, 0x21, 0x3D, 0x84, 0x82, 0x8A, 0x84, 0x24, 0x04, 0x51,
            0x11, 0x40, 0x9C, 0x00, 0x21, 0x13, 0x82, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        
        // 游戏标题
        let title = b"TETRIS GBA\0\0\0\0\0\0";
        rom.extend_from_slice(title);
        
        // 填充到最小ROM大小
        while rom.len() < 0x200 {
            rom.push(0x00);
        }
        
        // 添加游戏代码
        rom.extend_from_slice(&Self::create_game_code());
        
        rom
    }
    
    /// 创建游戏代码
    fn create_game_code() -> Vec<u8> {
        // 简化的ARM汇编代码，用于初始化GBA
        vec![
            // 初始化代码
            0x00, 0x00, 0x9F, 0xE5, // ldr r0, =0x04000000 (DISPCNT)
            0x01, 0x10, 0x80, 0xE3, // mov r1, #1
            0x00, 0x10, 0x80, 0xE5, // str r1, [r0]
            
            // 主循环
            0xFE, 0xFF, 0xFF, 0xEA, // b main_loop
            
            // 游戏逻辑占位符
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    }
    
    /// 更新游戏状态
    pub fn update(&mut self) {
        // 更新俄罗斯方块游戏逻辑
        self.tetris.update();
        
        // 更新GBA模拟器
        if let Err(e) = self.gba.step() {
            eprintln!("GBA模拟器错误: {}", e);
        }
        
        // 处理输入
        self.handle_input();
        
        // 渲染到缓冲区
        self.render();
    }
    
    /// 处理输入
    fn handle_input(&mut self) {
        let now = Instant::now();
        
        if now.duration_since(self.last_input_time) >= self.input_repeat_interval {
            if self.input_state.left {
                self.tetris.move_piece(-1, 0);
            }
            if self.input_state.right {
                self.tetris.move_piece(1, 0);
            }
            if self.input_state.down {
                self.tetris.move_piece(0, 1);
            }
            if self.input_state.rotate {
                self.tetris.rotate_piece();
            }
            if self.input_state.hard_drop {
                self.tetris.hard_drop();
            }
            if self.input_state.pause {
                self.tetris.toggle_pause();
            }
            
            self.last_input_time = now;
        }
    }
    
    /// 渲染游戏
    fn render(&mut self) {
        // 清空缓冲区
        for row in &mut self.render_buffer {
            for pixel in row {
                *pixel = Self::color_to_gba(Color::Black);
            }
        }
        
        // 渲染游戏板
        self.render_board();
        
        // 渲染当前方块
        if let Some(piece) = self.tetris.current_piece.clone() {
            self.render_piece(&piece, false);
        }
        
        // 渲染幽灵方块
        if let Some(ghost) = self.tetris.ghost_piece.clone() {
            self.render_piece(&ghost, true);
        }
        
        // 渲染UI
        self.render_ui();
    }
    
    /// 渲染游戏板
    fn render_board(&mut self) {
        let board = self.tetris.get_board();
        let start_x = 50;
        let start_y = 20;
        let cell_size = 8;
        
        for (y, row) in board.grid.iter().enumerate() {
            for (x, &color) in row.iter().enumerate() {
                if color != Color::Black {
                    let pixel_x = start_x + x * cell_size;
                    let pixel_y = start_y + y * cell_size;
                    
                    for dy in 0..cell_size {
                        for dx in 0..cell_size {
                            let screen_x = pixel_x + dx;
                            let screen_y = pixel_y + dy;
                            
                            if screen_x < 240 && screen_y < 160 {
                                self.render_buffer[screen_y][screen_x] = Self::color_to_gba(color);
                            }
                        }
                    }
                }
            }
        }
        
        // 绘制边框
        self.draw_border(start_x, start_y, board.width * cell_size, board.height * cell_size);
    }
    
    /// 渲染方块
    fn render_piece(&mut self, piece: &Tetromino, is_ghost: bool) {
        let start_x = 50 + piece.x as usize * 8;
        let start_y = 20 + piece.y as usize * 8;
        
        for (py, row) in piece.shape.iter().enumerate() {
            for (px, &cell) in row.iter().enumerate() {
                if cell {
                    let pixel_x = start_x + px * 8;
                    let pixel_y = start_y + py * 8;
                    
                    let color = if is_ghost {
                        Self::make_ghost_color(piece.color)
                    } else {
                        piece.color
                    };
                    
                    for dy in 0..8 {
                        for dx in 0..8 {
                            let screen_x = pixel_x + dx;
                            let screen_y = pixel_y + dy;
                            
                            if screen_x < 240 && screen_y < 160 {
                                self.render_buffer[screen_y][screen_x] = Self::color_to_gba(color);
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// 渲染UI
    fn render_ui(&mut self) {
        let stats = self.tetris.get_stats().clone();
        
        // 渲染分数
        self.draw_text(&format!("SCORE: {}", stats.score), 10, 10);
        self.draw_text(&format!("LEVEL: {}", stats.level), 10, 20);
        self.draw_text(&format!("LINES: {}", stats.lines_cleared), 10, 30);
        
        // 渲染游戏状态
        match self.tetris.get_state() {
            GameState::Playing => {},
            GameState::Paused => {
                self.draw_text("PAUSED", 100, 80);
            },
            GameState::GameOver => {
                self.draw_text("GAME OVER", 90, 80);
                self.draw_text("PRESS R TO RESTART", 70, 90);
            },
            GameState::Menu => {
                self.draw_text("TETRIS GBA", 80, 80);
                self.draw_text("PRESS START", 85, 90);
            },
        }
    }
    
    /// 绘制边框
    fn draw_border(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let border_color = Self::color_to_gba(Color::Gray);
        
        // 上边框
        for dx in 0..width {
            if x + dx < 240 && y < 160 {
                self.render_buffer[y][x + dx] = border_color;
            }
        }
        
        // 下边框
        for dx in 0..width {
            if x + dx < 240 && y + height < 160 {
                self.render_buffer[y + height][x + dx] = border_color;
            }
        }
        
        // 左边框
        for dy in 0..height {
            if x < 240 && y + dy < 160 {
                self.render_buffer[y + dy][x] = border_color;
            }
        }
        
        // 右边框
        for dy in 0..height {
            if x + width < 240 && y + dy < 160 {
                self.render_buffer[y + dy][x + width] = border_color;
            }
        }
    }
    
    /// 绘制文本（简化版）
    fn draw_text(&mut self, text: &str, x: usize, y: usize) {
        // 简化的文本渲染，使用8x8像素字体
        for (i, ch) in text.chars().enumerate() {
            let char_x = x + i * 8;
            if char_x < 240 && y < 160 {
                self.draw_char(ch, char_x, y);
            }
        }
    }
    
    /// 绘制单个字符
    fn draw_char(&mut self, ch: char, x: usize, y: usize) {
        // 简化的字符渲染
        let color = Self::color_to_gba(Color::Cyan);
        
        match ch {
            'A'..='Z' | '0'..='9' | ' ' => {
                // 绘制简单的8x8字符
                for dy in 0..8 {
                    for dx in 0..8 {
                        let screen_x = x + dx;
                        let screen_y = y + dy;
                        
                        if screen_x < 240 && screen_y < 160 {
                            // 简单的字符模式
                            if (dx == 0 || dx == 7) || (dy == 0 || dy == 7) {
                                self.render_buffer[screen_y][screen_x] = color;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
    
    /// 将颜色转换为GBA格式
    fn color_to_gba(color: Color) -> u16 {
        match color {
            Color::Black => 0x0000,
            Color::Cyan => 0x07FF,
            Color::Yellow => 0xFFE0,
            Color::Purple => 0xF81F,
            Color::Green => 0x07E0,
            Color::Red => 0xF800,
            Color::Blue => 0x001F,
            Color::Orange => 0xFC00,
            Color::Gray => 0x7BEF,
        }
    }
    
    /// 创建幽灵方块颜色
    fn make_ghost_color(color: Color) -> Color {
        // 将颜色变暗作为幽灵方块
        match color {
            Color::Cyan => Color::Gray,
            Color::Yellow => Color::Gray,
            Color::Purple => Color::Gray,
            Color::Green => Color::Gray,
            Color::Red => Color::Gray,
            Color::Blue => Color::Gray,
            Color::Orange => Color::Gray,
            _ => color,
        }
    }
    
    /// 处理键盘输入
    pub fn handle_key_input(&mut self, key: char) {
        match key {
            'a' | 'A' => self.input_state.left = true,
            'd' | 'D' => self.input_state.right = true,
            's' | 'S' => self.input_state.down = true,
            'w' | 'W' => self.input_state.rotate = true,
            ' ' => self.input_state.hard_drop = true,
            'p' | 'P' => self.input_state.pause = true,
            'r' | 'R' => {
                if *self.tetris.get_state() == GameState::GameOver {
                    self.tetris.reset();
                }
            },
            _ => {}
        }
    }
    
    /// 释放键盘输入
    pub fn release_key_input(&mut self, key: char) {
        match key {
            'a' | 'A' => self.input_state.left = false,
            'd' | 'D' => self.input_state.right = false,
            's' | 'S' => self.input_state.down = false,
            'w' | 'W' => self.input_state.rotate = false,
            ' ' => self.input_state.hard_drop = false,
            'p' | 'P' => self.input_state.pause = false,
            _ => {}
        }
    }
    
    /// 获取渲染缓冲区
    pub fn get_render_buffer(&self) -> &Vec<Vec<u16>> {
        &self.render_buffer
    }
    
    /// 获取游戏统计信息
    pub fn get_game_stats(&self) -> &crate::games::tetris::tetris_game::GameStats {
        self.tetris.get_stats()
    }
    
    /// 获取GBA统计信息
    pub fn get_gba_stats(&self) -> &crate::gba::GBAStats {
        self.gba.get_stats()
    }
    
    /// 重置游戏
    pub fn reset(&mut self) {
        self.tetris.reset();
        self.input_state = InputState::default();
        self.last_input_time = Instant::now();
    }
}

impl Default for GBATetris {
    fn default() -> Self {
        Self::new()
    }
}
