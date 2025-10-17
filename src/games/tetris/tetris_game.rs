//! 俄罗斯方块游戏核心实现
//! 
//! 实现经典的俄罗斯方块游戏逻辑，包括方块移动、旋转、消除等

use std::time::{Duration, Instant};
use std::collections::VecDeque;

/// 俄罗斯方块游戏状态
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
    Menu,
}

/// 方块类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TetrominoType {
    I, // 直线
    O, // 正方形
    T, // T形
    S, // S形
    Z, // Z形
    J, // J形
    L, // L形
}

/// 方块颜色
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Cyan,    // I - 青色
    Yellow,  // O - 黄色
    Purple,  // T - 紫色
    Green,   // S - 绿色
    Red,     // Z - 红色
    Blue,    // J - 蓝色
    Orange,  // L - 橙色
    Gray,    // 已放置的方块
    Black,   // 空白
}

/// 方块结构
#[derive(Debug, Clone)]
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub color: Color,
    pub shape: Vec<Vec<bool>>,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
}

/// 游戏板
#[derive(Debug, Clone)]
pub struct GameBoard {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Color>>,
}

/// 游戏统计
#[derive(Debug, Clone)]
pub struct GameStats {
    pub score: u32,
    pub lines_cleared: u32,
    pub level: u32,
    pub tetris_count: u32,
    pub total_pieces: u32,
    pub start_time: Instant,
    pub play_time: Duration,
}

/// 俄罗斯方块游戏
#[derive(Debug)]
pub struct TetrisGame {
    pub board: GameBoard,
    pub current_piece: Option<Tetromino>,
    pub next_piece: Option<Tetromino>,
    pub state: GameState,
    pub stats: GameStats,
    pub drop_timer: Instant,
    pub drop_interval: Duration,
    pub piece_bag: VecDeque<TetrominoType>,
    pub ghost_piece: Option<Tetromino>,
}

impl Tetromino {
    /// 创建新的方块
    pub fn new(tetromino_type: TetrominoType) -> Self {
        let (shape, color) = Self::get_shape_and_color(tetromino_type);
        
        Self {
            tetromino_type,
            color,
            shape,
            x: 3, // 起始位置
            y: 0,
            rotation: 0,
        }
    }
    
    /// 获取方块的形状和颜色
    fn get_shape_and_color(tetromino_type: TetrominoType) -> (Vec<Vec<bool>>, Color) {
        match tetromino_type {
            TetrominoType::I => (
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                Color::Cyan,
            ),
            TetrominoType::O => (
                vec![
                    vec![true, true],
                    vec![true, true],
                ],
                Color::Yellow,
            ),
            TetrominoType::T => (
                vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                Color::Purple,
            ),
            TetrominoType::S => (
                vec![
                    vec![false, true, true],
                    vec![true, true, false],
                    vec![false, false, false],
                ],
                Color::Green,
            ),
            TetrominoType::Z => (
                vec![
                    vec![true, true, false],
                    vec![false, true, true],
                    vec![false, false, false],
                ],
                Color::Red,
            ),
            TetrominoType::J => (
                vec![
                    vec![true, false, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                Color::Blue,
            ),
            TetrominoType::L => (
                vec![
                    vec![false, false, true],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                Color::Orange,
            ),
        }
    }
    
    /// 旋转方块
    pub fn rotate(&mut self) {
        let size = self.shape.len();
        let mut new_shape = vec![vec![false; size]; size];
        
        for i in 0..size {
            for j in 0..size {
                new_shape[j][size - 1 - i] = self.shape[i][j];
            }
        }
        
        self.shape = new_shape;
        self.rotation = (self.rotation + 1) % 4;
    }
    
    /// 获取旋转后的形状
    pub fn get_rotated_shape(&self) -> Vec<Vec<bool>> {
        let size = self.shape.len();
        let mut new_shape = vec![vec![false; size]; size];
        
        for i in 0..size {
            for j in 0..size {
                new_shape[j][size - 1 - i] = self.shape[i][j];
            }
        }
        
        new_shape
    }
    
    /// 获取方块的边界框
    pub fn get_bounds(&self) -> (usize, usize, usize, usize) {
        let mut min_x = self.shape.len();
        let mut max_x = 0;
        let mut min_y = self.shape.len();
        let mut max_y = 0;
        
        for (y, row) in self.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                }
            }
        }
        
        (min_x, min_y, max_x, max_y)
    }
}

impl GameBoard {
    /// 创建新的游戏板
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![Color::Black; width]; height],
        }
    }
    
    /// 检查位置是否有效
    pub fn is_valid_position(&self, piece: &Tetromino, dx: i32, dy: i32) -> bool {
        let new_x = piece.x + dx;
        let new_y = piece.y + dy;
        
        for (py, row) in piece.shape.iter().enumerate() {
            for (px, &cell) in row.iter().enumerate() {
                if cell {
                    let board_x = new_x + px as i32;
                    let board_y = new_y + py as i32;
                    
                    // 检查边界
                    if board_x < 0 || board_x >= self.width as i32 || 
                       board_y < 0 || board_y >= self.height as i32 {
                        return false;
                    }
                    
                    // 检查是否与已放置的方块冲突
                    if self.grid[board_y as usize][board_x as usize] != Color::Black {
                        return false;
                    }
                }
            }
        }
        
        true
    }
    
    /// 放置方块
    pub fn place_piece(&mut self, piece: &Tetromino) {
        for (py, row) in piece.shape.iter().enumerate() {
            for (px, &cell) in row.iter().enumerate() {
                if cell {
                    let board_x = (piece.x + px as i32) as usize;
                    let board_y = (piece.y + py as i32) as usize;
                    
                    if board_x < self.width && board_y < self.height {
                        self.grid[board_y][board_x] = piece.color;
                    }
                }
            }
        }
    }
    
    /// 检查并清除完整的行
    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut new_grid = Vec::new();
        
        for row in &self.grid {
            if row.iter().all(|&color| color != Color::Black) {
                lines_cleared += 1;
            } else {
                new_grid.push(row.clone());
            }
        }
        
        // 在顶部添加空行
        while new_grid.len() < self.height {
            new_grid.insert(0, vec![Color::Black; self.width]);
        }
        
        self.grid = new_grid;
        lines_cleared
    }
    
    /// 检查游戏是否结束
    pub fn is_game_over(&self) -> bool {
        // 检查顶部行是否有方块
        self.grid[0].iter().any(|&color| color != Color::Black)
    }
}

impl TetrisGame {
    /// 创建新的俄罗斯方块游戏
    pub fn new() -> Self {
        let mut game = Self {
            board: GameBoard::new(10, 20),
            current_piece: None,
            next_piece: None,
            state: GameState::Menu,
            stats: GameStats {
                score: 0,
                lines_cleared: 0,
                level: 1,
                tetris_count: 0,
                total_pieces: 0,
                start_time: Instant::now(),
                play_time: Duration::ZERO,
            },
            drop_timer: Instant::now(),
            drop_interval: Duration::from_millis(1000),
            piece_bag: VecDeque::new(),
            ghost_piece: None,
        };
        
        game.fill_piece_bag();
        game.spawn_next_piece();
        game.state = GameState::Playing;
        
        game
    }
    
    /// 填充方块袋
    fn fill_piece_bag(&mut self) {
        let mut pieces = vec![
            TetrominoType::I, TetrominoType::O, TetrominoType::T,
            TetrominoType::S, TetrominoType::Z, TetrominoType::J, TetrominoType::L
        ];
        
        // 随机打乱
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        for i in 0..pieces.len() {
            let mut hasher = DefaultHasher::new();
            i.hash(&mut hasher);
            let j = hasher.finish() as usize % pieces.len();
            pieces.swap(i, j);
        }
        
        for piece_type in pieces {
            self.piece_bag.push_back(piece_type);
        }
    }
    
    /// 生成下一个方块
    fn spawn_next_piece(&mut self) {
        if self.piece_bag.is_empty() {
            self.fill_piece_bag();
        }
        
        let piece_type = self.piece_bag.pop_front().unwrap();
        let piece = Tetromino::new(piece_type);
        
        // 检查是否可以放置
        if !self.board.is_valid_position(&piece, 0, 0) {
            self.state = GameState::GameOver;
            return;
        }
        
        self.current_piece = Some(piece);
        self.update_ghost_piece();
    }
    
    /// 更新幽灵方块（预览位置）
    fn update_ghost_piece(&mut self) {
        if let Some(ref mut piece) = self.current_piece {
            let mut ghost = piece.clone();
            
            // 将幽灵方块移动到最底部
            while self.board.is_valid_position(&ghost, 0, 1) {
                ghost.y += 1;
            }
            
            self.ghost_piece = Some(ghost);
        }
    }
    
    /// 移动当前方块
    pub fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            if self.board.is_valid_position(piece, dx, dy) {
                piece.x += dx;
                piece.y += dy;
                self.update_ghost_piece();
                return true;
            }
        }
        false
    }
    
    /// 旋转当前方块
    pub fn rotate_piece(&mut self) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            let original_shape = piece.shape.clone();
            let original_rotation = piece.rotation;
            
            piece.rotate();
            
            // 检查旋转后是否有效
            if !self.board.is_valid_position(piece, 0, 0) {
                // 尝试踢墙（wall kick）
                let kicks = vec![(-1, 0), (1, 0), (0, -1), (-1, -1), (1, -1)];
                
                for (kx, ky) in kicks {
                    if self.board.is_valid_position(piece, kx, ky) {
                        piece.x += kx;
                        piece.y += ky;
                        self.update_ghost_piece();
                        return true;
                    }
                }
                
                // 恢复原始状态
                piece.shape = original_shape;
                piece.rotation = original_rotation;
                return false;
            }
            
            self.update_ghost_piece();
            return true;
        }
        false
    }
    
    /// 硬降落（直接到底部）
    pub fn hard_drop(&mut self) {
        if let Some(ref mut piece) = self.current_piece {
            while self.board.is_valid_position(piece, 0, 1) {
                piece.y += 1;
            }
            self.place_current_piece();
        }
    }
    
    /// 放置当前方块
    fn place_current_piece(&mut self) {
        if let Some(piece) = self.current_piece.take() {
            self.board.place_piece(&piece);
            self.stats.total_pieces += 1;
            
            // 检查并清除行
            let lines_cleared = self.board.clear_lines();
            if lines_cleared > 0 {
                self.stats.lines_cleared += lines_cleared;
                
                // 计算得分
                let base_score = match lines_cleared {
                    1 => 100,
                    2 => 300,
                    3 => 500,
                    4 => 800,
                    _ => 0,
                };
                
                self.stats.score += base_score * self.stats.level;
                
                // 检查是否是Tetris（一次清除4行）
                if lines_cleared == 4 {
                    self.stats.tetris_count += 1;
                }
                
                // 升级
                self.stats.level = (self.stats.lines_cleared / 10) + 1;
                self.drop_interval = Duration::from_millis(
                    (1000.0 / (1.0 + self.stats.level as f64 * 0.1)).max(50.0) as u64
                );
            }
            
            // 检查游戏结束
            if self.board.is_game_over() {
                self.state = GameState::GameOver;
            } else {
                self.spawn_next_piece();
            }
        }
    }
    
    /// 更新游戏状态
    pub fn update(&mut self) {
        if self.state != GameState::Playing {
            return;
        }
        
        // 检查自动降落
        if self.drop_timer.elapsed() >= self.drop_interval {
            if !self.move_piece(0, 1) {
                self.place_current_piece();
            }
            self.drop_timer = Instant::now();
        }
        
        // 更新游戏时间
        self.stats.play_time = self.stats.start_time.elapsed();
    }
    
    /// 暂停/恢复游戏
    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
            _ => {}
        }
    }
    
    /// 重置游戏
    pub fn reset(&mut self) {
        self.board = GameBoard::new(10, 20);
        self.current_piece = None;
        self.next_piece = None;
        self.state = GameState::Playing;
        self.stats = GameStats {
            score: 0,
            lines_cleared: 0,
            level: 1,
            tetris_count: 0,
            total_pieces: 0,
            start_time: Instant::now(),
            play_time: Duration::ZERO,
        };
        self.drop_timer = Instant::now();
        self.drop_interval = Duration::from_millis(1000);
        self.piece_bag.clear();
        self.ghost_piece = None;
        
        self.fill_piece_bag();
        self.spawn_next_piece();
    }
    
    /// 获取游戏统计信息
    pub fn get_stats(&self) -> &GameStats {
        &self.stats
    }
    
    /// 获取游戏板
    pub fn get_board(&self) -> &GameBoard {
        &self.board
    }
    
    /// 获取当前方块
    pub fn get_current_piece(&self) -> &Option<Tetromino> {
        &self.current_piece
    }
    
    /// 获取幽灵方块
    pub fn get_ghost_piece(&self) -> &Option<Tetromino> {
        &self.ghost_piece
    }
    
    /// 获取游戏状态
    pub fn get_state(&self) -> &GameState {
        &self.state
    }
}

impl Default for TetrisGame {
    fn default() -> Self {
        Self::new()
    }
}
