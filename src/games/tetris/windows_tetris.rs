//! Windows俄罗斯方块游戏
//! 
//! 基于GBA模拟器实现的Windows俄罗斯方块游戏
//! 使用控制台界面，支持完整的俄罗斯方块游戏功能

use crate::games::tetris::tetris_game::{TetrisGame, GameState, Tetromino, Color};
use crate::gba::GBASystem;
use std::io::{self, Write, stdin};
use std::time::{Duration, Instant};
use std::thread;

/// Windows俄罗斯方块游戏
struct WindowsTetris {
    /// 俄罗斯方块游戏逻辑
    tetris: TetrisGame,
    /// GBA模拟器（用于底层支持）
    gba: GBASystem,
    /// 游戏循环控制
    running: bool,
    /// 上次渲染时间
    last_render_time: Instant,
    /// 渲染间隔
    render_interval: Duration,
    /// 输入缓冲区
    input_buffer: String,
}

impl WindowsTetris {
    /// 创建新的Windows俄罗斯方块游戏
    fn new() -> Self {
        let mut gba = GBASystem::new();
        
        // 创建简化的ROM数据
        let rom_data = Self::create_tetris_rom();
        if let Err(e) = gba.load_rom(rom_data) {
            eprintln!("警告: 无法加载GBA ROM: {}", e);
        } else {
            gba.start().unwrap();
        }
        
        Self {
            tetris: TetrisGame::new(),
            gba,
            running: true,
            last_render_time: Instant::now(),
            render_interval: Duration::from_millis(100),
            input_buffer: String::new(),
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
        let title = b"TETRIS WIN\0\0\0\0\0\0";
        rom.extend_from_slice(title);
        
        // 填充到最小ROM大小
        while rom.len() < 0x200 {
            rom.push(0x00);
        }
        
        rom
    }
    
    /// 运行游戏主循环
    fn run(&mut self) {
        self.clear_screen();
        self.show_welcome();
        
        while self.running {
            self.update();
            self.render();
            self.handle_input();
            
            thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }
        
        self.show_game_over();
    }
    
    /// 更新游戏状态
    fn update(&mut self) {
        // 更新俄罗斯方块游戏
        self.tetris.update();
        
        // 更新GBA模拟器（用于底层支持）
        if let Err(e) = self.gba.step() {
            eprintln!("GBA模拟器错误: {}", e);
        }
    }
    
    /// 渲染游戏画面
    fn render(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_render_time) < self.render_interval {
            return;
        }
        
        self.last_render_time = now;
        
        // 清屏
        self.clear_screen();
        
        // 渲染游戏板
        self.render_board();
        
        // 渲染当前方块
        self.render_current_piece();
        
        // 渲染幽灵方块
        self.render_ghost_piece();
        
        // 渲染UI
        self.render_ui();
        
        // 渲染控制说明
        self.render_controls();
        
        // 刷新屏幕
        io::stdout().flush().unwrap();
    }
    
    /// 渲染游戏板
    fn render_board(&mut self) {
        let board = self.tetris.get_board();
        let start_x = 2;
        let start_y = 3;
        
        print!("\x1B[{};{}H", start_y, start_x);
        print!("┌");
        for _ in 0..board.width {
            print!("─");
        }
        print!("┐");
        
        for (y, row) in board.grid.iter().enumerate() {
            print!("\x1B[{};{}H", start_y + y + 1, start_x);
            print!("│");
            
            for &color in row {
                let char = Self::color_to_char(color);
                print!("{}", char);
            }
            
            print!("│");
        }
        
        print!("\x1B[{};{}H", start_y + board.height + 1, start_x);
        print!("└");
        for _ in 0..board.width {
            print!("─");
        }
        print!("┘");
    }
    
    /// 渲染当前方块
    fn render_current_piece(&mut self) {
        if let Some(piece) = self.tetris.current_piece.clone() {
            self.render_piece(&piece, false);
        }
    }
    
    /// 渲染幽灵方块
    fn render_ghost_piece(&mut self) {
        if let Some(ghost) = self.tetris.ghost_piece.clone() {
            self.render_piece(&ghost, true);
        }
    }
    
    /// 渲染方块
    fn render_piece(&mut self, piece: &Tetromino, is_ghost: bool) {
        let board = self.tetris.get_board();
        let start_x = 2;
        let start_y = 3;
        
        for (py, row) in piece.shape.iter().enumerate() {
            for (px, &cell) in row.iter().enumerate() {
                if cell {
                    let board_x = piece.x + px as i32;
                    let board_y = piece.y + py as i32;
                    
                    if board_x >= 0 && board_x < board.width as i32 &&
                       board_y >= 0 && board_y < board.height as i32 {
                        
                        let screen_x = start_x + board_x as usize + 1;
                        let screen_y = start_y + board_y as usize + 1;
                        
                        print!("\x1B[{};{}H", screen_y, screen_x);
                        
                        let char = if is_ghost {
                            Self::color_to_char(Color::Gray)
                        } else {
                            Self::color_to_char(piece.color)
                        };
                        
                        print!("{}", char);
                    }
                }
            }
        }
    }
    
    /// 渲染UI
    fn render_ui(&mut self) {
        let stats = self.tetris.get_stats();
        let gba_stats = self.gba.get_stats();
        
        // 渲染统计信息
        print!("\x1B[3;25H");
        print!("┌─ 游戏统计 ─┐");
        
        print!("\x1B[4;25H");
        print!("│ 分数: {:>8} │", stats.score);
        
        print!("\x1B[5;25H");
        print!("│ 等级: {:>8} │", stats.level);
        
        print!("\x1B[6;25H");
        print!("│ 行数: {:>8} │", stats.lines_cleared);
        
        print!("\x1B[7;25H");
        print!("│ Tetris: {:>6} │", stats.tetris_count);
        
        print!("\x1B[8;25H");
        print!("│ 方块数: {:>6} │", stats.total_pieces);
        
        print!("\x1B[9;25H");
        print!("└──────────────┘");
        
        // 渲染GBA统计信息
        print!("\x1B[11;25H");
        print!("┌─ GBA统计 ─┐");
        
        print!("\x1B[12;25H");
        print!("│ FPS: {:>8.1} │", gba_stats.fps);
        
        print!("\x1B[13;25H");
        print!("│ CPU: {:>7.1}% │", gba_stats.cpu_usage * 100.0);
        
        print!("\x1B[14;25H");
        print!("│ 帧数: {:>7} │", gba_stats.total_frames);
        
        print!("\x1B[15;25H");
        print!("└────────────┘");
        
        // 渲染游戏状态
        print!("\x1B[17;25H");
        match self.tetris.get_state() {
            GameState::Playing => {
                print!("┌─ 游戏状态 ─┐");
                print!("\x1B[18;25H");
                print!("│   游戏中    │");
                print!("\x1B[19;25H");
                print!("└────────────┘");
            },
            GameState::Paused => {
                print!("┌─ 游戏状态 ─┐");
                print!("\x1B[18;25H");
                print!("│   暂停中    │");
                print!("\x1B[19;25H");
                print!("└────────────┘");
            },
            GameState::GameOver => {
                print!("┌─ 游戏状态 ─┐");
                print!("\x1B[18;25H");
                print!("│   游戏结束  │");
                print!("\x1B[19;25H");
                print!("└────────────┘");
            },
            GameState::Menu => {
                print!("┌─ 游戏状态 ─┐");
                print!("\x1B[18;25H");
                print!("│   主菜单    │");
                print!("\x1B[19;25H");
                print!("└────────────┘");
            },
        }
    }
    
    /// 渲染控制说明
    fn render_controls(&mut self) {
        print!("\x1B[21;25H");
        print!("┌─ 控制说明 ─┐");
        
        print!("\x1B[22;25H");
        print!("│ A/D: 左右移动 │");
        
        print!("\x1B[23;25H");
        print!("│ S: 快速下降   │");
        
        print!("\x1B[24;25H");
        print!("│ W: 旋转方块   │");
        
        print!("\x1B[25;25H");
        print!("│ 空格: 硬降落  │");
        
        print!("\x1B[26;25H");
        print!("│ P: 暂停游戏   │");
        
        print!("\x1B[27;25H");
        print!("│ R: 重新开始   │");
        
        print!("\x1B[28;25H");
        print!("│ Q: 退出游戏   │");
        
        print!("\x1B[29;25H");
        print!("└──────────────┘");
    }
    
    /// 处理输入
    fn handle_input(&mut self) {
        // 检查是否有输入
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
        
        if !input.is_empty() {
            let ch = input.chars().next().unwrap().to_ascii_lowercase();
            
            match ch {
                'a' => { self.tetris.move_piece(-1, 0); },
                'd' => { self.tetris.move_piece(1, 0); },
                's' => { self.tetris.move_piece(0, 1); },
                'w' => { self.tetris.rotate_piece(); },
                ' ' => { self.tetris.hard_drop(); },
                'p' => { self.tetris.toggle_pause(); },
                'r' => {
                    if *self.tetris.get_state() == GameState::GameOver {
                        self.tetris.reset();
                    }
                },
                'q' => self.running = false,
                _ => {}
            }
        }
    }
    
    /// 将颜色转换为字符
    fn color_to_char(color: Color) -> char {
        match color {
            Color::Black => ' ',
            Color::Cyan => '█',
            Color::Yellow => '█',
            Color::Purple => '█',
            Color::Green => '█',
            Color::Red => '█',
            Color::Blue => '█',
            Color::Orange => '█',
            Color::Gray => '░',
        }
    }
    
    /// 清屏
    fn clear_screen(&mut self) {
        print!("\x1B[2J\x1B[H");
    }
    
    /// 显示欢迎信息
    fn show_welcome(&mut self) {
        self.clear_screen();
        println!("🎮 Windows俄罗斯方块 - 基于GBA模拟器");
        println!("==================================================");
        println!("");
        println!("欢迎来到俄罗斯方块游戏！");
        println!("本游戏基于我们开发的GBA模拟器实现");
        println!("");
        println!("游戏特色：");
        println!("  ✅ 完整的俄罗斯方块游戏逻辑");
        println!("  ✅ 基于GBA模拟器底层支持");
        println!("  ✅ 实时性能统计");
        println!("  ✅ 幽灵方块预览");
        println!("  ✅ 完整的UI界面");
        println!("");
        println!("按任意键开始游戏...");
        
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
    }
    
    /// 显示游戏结束信息
    fn show_game_over(&mut self) {
        self.clear_screen();
        let stats = self.tetris.get_stats();
        
        println!("🎮 游戏结束！");
        println!("==================================================");
        println!("");
        println!("最终统计：");
        println!("  分数: {}", stats.score);
        println!("  等级: {}", stats.level);
        println!("  清除行数: {}", stats.lines_cleared);
        println!("  Tetris次数: {}", stats.tetris_count);
        println!("  总方块数: {}", stats.total_pieces);
        println!("  游戏时间: {:.1}秒", stats.play_time.as_secs_f64());
        println!("");
        println!("感谢游玩！");
        println!("");
    }
}

/// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 启动Windows俄罗斯方块游戏...");
    println!("基于GBA模拟器实现");
    println!("");
    
    // 检查系统支持
    if cfg!(windows) {
        println!("✅ Windows系统检测通过");
    } else {
        println!("⚠️  非Windows系统，但游戏仍可运行");
    }
    
    // 创建并运行游戏
    let mut game = WindowsTetris::new();
    game.run();
    
    println!("游戏已退出，感谢游玩！");
    Ok(())
}
