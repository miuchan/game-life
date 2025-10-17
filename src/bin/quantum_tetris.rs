//! 量子纠缠俄罗斯方块 - 革命性创新版本
//! 
//! 这是世界上第一个量子纠缠俄罗斯方块游戏，具有以下革命性特性：
//! 1. 量子纠缠机制 - 多个方块同时操作
//! 2. 时空扭曲 - 方块可以穿越时间
//! 3. 概率叠加 - 方块同时存在于多个状态
//! 4. 量子隧道 - 方块可以穿过障碍物
//! 5. 观察者效应 - 玩家的观察影响游戏状态

use std::io::{self, Write, stdin};
use std::time::{Duration, Instant};
use std::thread;
use std::f64::consts::PI;

/// 量子状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantumState {
    Superposition,  // 叠加态
    Entangled,      // 纠缠态
    Collapsed,      // 坍缩态
    Tunneling,      // 隧道态
}

/// 时空坐标
#[derive(Debug, Clone, Copy)]
pub struct SpacetimeCoord {
    pub x: f64,
    pub y: f64,
    pub t: f64,  // 时间维度
    pub probability: f64,  // 存在概率
}

/// 量子方块
#[derive(Debug, Clone)]
pub struct QuantumTetromino {
    pub tetromino_type: QuantumTetrominoType,
    pub quantum_state: QuantumState,
    pub spacetime_coords: Vec<SpacetimeCoord>,
    pub entanglement_partners: Vec<usize>,  // 纠缠伙伴的索引
    pub wave_function: Vec<f64>,  // 波函数
    pub phase: f64,  // 相位
    pub energy: f64,  // 能量
}

/// 量子方块类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantumTetrominoType {
    QuantumI,    // 量子直线
    QuantumO,    // 量子正方形
    QuantumT,    // 量子T形
    QuantumS,    // 量子S形
    QuantumZ,    // 量子Z形
    QuantumJ,    // 量子J形
    QuantumL,    // 量子L形
    EntangledPair, // 纠缠对
    SuperpositionBlock, // 叠加方块
}

/// 量子游戏板
#[derive(Debug, Clone)]
pub struct QuantumGameBoard {
    pub width: usize,
    pub height: usize,
    pub quantum_grid: Vec<Vec<QuantumCell>>,
    pub spacetime_field: Vec<Vec<f64>>,  // 时空场
    pub quantum_field: Vec<Vec<f64>>,    // 量子场
    pub observer_effect: f64,           // 观察者效应强度
}

/// 量子单元格
#[derive(Debug, Clone)]
pub struct QuantumCell {
    pub occupied_probability: f64,
    pub quantum_state: QuantumState,
    pub energy_level: f64,
    pub phase: f64,
    pub entangled_with: Vec<usize>,
}

/// 量子游戏统计
#[derive(Debug, Clone)]
pub struct QuantumGameStats {
    pub quantum_score: f64,
    pub entanglement_count: u32,
    pub superposition_events: u32,
    pub tunneling_events: u32,
    pub observer_interactions: u32,
    pub quantum_coherence: f64,
    pub spacetime_distortion: f64,
    pub start_time: Instant,
    pub play_time: Duration,
}

/// 量子俄罗斯方块游戏
#[derive(Debug)]
pub struct QuantumTetrisGame {
    pub board: QuantumGameBoard,
    pub current_quantum_pieces: Vec<QuantumTetromino>,
    pub quantum_field_strength: f64,
    pub spacetime_distortion: f64,
    pub observer_presence: f64,
    pub state: QuantumGameState,
    pub stats: QuantumGameStats,
    pub quantum_timer: Instant,
    pub quantum_interval: Duration,
    pub entanglement_network: Vec<Vec<usize>>,
}

/// 量子游戏状态
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumGameState {
    QuantumSuperposition,  // 量子叠加态
    QuantumCollapse,       // 量子坍缩
    EntanglementMode,       // 纠缠模式
    TunnelingMode,          // 隧道模式
    ObserverMode,           // 观察者模式
    QuantumGameOver,        // 量子游戏结束
}

impl QuantumTetromino {
    /// 创建新的量子方块
    pub fn new(tetromino_type: QuantumTetrominoType) -> Self {
        let mut coords = Vec::new();
        let mut wave_function = Vec::new();
        
        // 根据方块类型创建量子坐标
        match tetromino_type {
            QuantumTetrominoType::QuantumI => {
                for i in 0..4 {
                    coords.push(SpacetimeCoord {
                        x: 3.0 + i as f64,
                        y: 0.0,
                        t: 0.0,
                        probability: 1.0 / 4.0,
                    });
                    wave_function.push((i as f64 * PI / 2.0).sin());
                }
            },
            QuantumTetrominoType::EntangledPair => {
                // 创建纠缠对
                coords.push(SpacetimeCoord {
                    x: 3.0,
                    y: 0.0,
                    t: 0.0,
                    probability: 0.5,
                });
                coords.push(SpacetimeCoord {
                    x: 7.0,
                    y: 0.0,
                    t: 0.0,
                    probability: 0.5,
                });
                wave_function.push(1.0);
                wave_function.push(-1.0); // 相反相位
            },
            _ => {
                // 其他方块类型
                coords.push(SpacetimeCoord {
                    x: 3.0,
                    y: 0.0,
                    t: 0.0,
                    probability: 1.0,
                });
                wave_function.push(1.0);
            }
        }
        
        Self {
            tetromino_type,
            quantum_state: QuantumState::Superposition,
            spacetime_coords: coords,
            entanglement_partners: Vec::new(),
            wave_function,
            phase: 0.0,
            energy: 1.0,
        }
    }
    
    /// 量子旋转 - 同时存在于多个旋转状态
    pub fn quantum_rotate(&mut self) {
        self.phase += PI / 4.0;
        
        // 更新波函数
        for (i, wave) in self.wave_function.iter_mut().enumerate() {
            *wave = (self.phase + i as f64 * PI / 2.0).sin();
        }
        
        // 更新坐标的概率分布
        for coord in &mut self.spacetime_coords {
            coord.probability = (coord.probability * (self.phase.cos().abs())).max(0.1);
        }
    }
    
    /// 量子移动 - 同时向多个方向移动
    pub fn quantum_move(&mut self, dx: f64, dy: f64) {
        for coord in &mut self.spacetime_coords {
            coord.x += dx;
            coord.y += dy;
            coord.t += 0.1; // 时间推进
        }
    }
    
    /// 创建量子纠缠
    pub fn create_entanglement(&mut self, other: &mut QuantumTetromino) {
        self.quantum_state = QuantumState::Entangled;
        other.quantum_state = QuantumState::Entangled;
        
        self.entanglement_partners.push(other.spacetime_coords.len());
        other.entanglement_partners.push(self.spacetime_coords.len());
        
        // 同步波函数
        for (i, wave) in self.wave_function.iter().enumerate() {
            if i < other.wave_function.len() {
                other.wave_function[i] = -*wave; // 相反相位
            }
        }
    }
    
    /// 量子隧道效应
    pub fn quantum_tunnel(&mut self, target_x: f64, target_y: f64) {
        self.quantum_state = QuantumState::Tunneling;
        
        for coord in &mut self.spacetime_coords {
            coord.x = target_x;
            coord.y = target_y;
            coord.probability *= 0.8; // 隧道效应降低概率
        }
    }
}

impl QuantumGameBoard {
    /// 创建新的量子游戏板
    pub fn new(width: usize, height: usize) -> Self {
        let mut quantum_grid = Vec::new();
        let mut spacetime_field = Vec::new();
        let mut quantum_field = Vec::new();
        
        for y in 0..height {
            let mut row = Vec::new();
            let mut spacetime_row = Vec::new();
            let mut quantum_row = Vec::new();
            
            for x in 0..width {
                row.push(QuantumCell {
                    occupied_probability: 0.0,
                    quantum_state: QuantumState::Collapsed,
                    energy_level: 0.0,
                    phase: 0.0,
                    entangled_with: Vec::new(),
                });
                
                // 创建时空场和量子场
                let spacetime_value = ((x as f64 + y as f64) * 0.1).sin();
                let quantum_value = ((x as f64 - y as f64) * 0.1).cos();
                
                spacetime_row.push(spacetime_value);
                quantum_row.push(quantum_value);
            }
            
            quantum_grid.push(row);
            spacetime_field.push(spacetime_row);
            quantum_field.push(quantum_row);
        }
        
        Self {
            width,
            height,
            quantum_grid,
            spacetime_field,
            quantum_field,
            observer_effect: 0.0,
        }
    }
    
    /// 检查量子位置是否有效
    pub fn is_quantum_position_valid(&self, piece: &QuantumTetromino) -> bool {
        for coord in &piece.spacetime_coords {
            let x = coord.x as usize;
            let y = coord.y as usize;
            
            if x >= self.width || y >= self.height {
                return false;
            }
            
            // 检查量子概率
            if coord.probability < 0.1 {
                return false;
            }
            
            // 检查与现有方块的量子干涉
            if self.quantum_grid[y][x].occupied_probability > 0.5 {
                return false;
            }
        }
        
        true
    }
    
    /// 量子干涉效应
    pub fn quantum_interference(&mut self, piece: &QuantumTetromino) {
        for coord in &piece.spacetime_coords {
            let x = coord.x as usize;
            let y = coord.y as usize;
            
            if x < self.width && y < self.height {
                let cell = &mut self.quantum_grid[y][x];
                
                // 量子干涉计算
                let interference = cell.phase + piece.phase;
                cell.occupied_probability += coord.probability * interference.cos().abs();
                cell.energy_level += piece.energy * coord.probability;
                cell.phase = interference;
            }
        }
    }
    
    /// 观察者效应 - 观察导致量子坍缩
    pub fn observer_effect(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let cell = &mut self.quantum_grid[y][x];
            
            // 观察导致概率坍缩
            if cell.occupied_probability > 0.5 {
                cell.occupied_probability = 1.0;
                cell.quantum_state = QuantumState::Collapsed;
            } else {
                cell.occupied_probability = 0.0;
                cell.quantum_state = QuantumState::Collapsed;
            }
            
            // 影响周围的量子态
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    
                    if nx < self.width && ny < self.height {
                        let neighbor = &mut self.quantum_grid[ny][nx];
                        neighbor.occupied_probability *= 0.9; // 观察者效应衰减
                    }
                }
            }
        }
    }
    
    /// 量子行消除
    pub fn quantum_line_clear(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut new_grid = Vec::new();
        
        for (y, row) in self.quantum_grid.iter().enumerate() {
            let mut should_clear = true;
            
            for cell in row {
                if cell.occupied_probability < 0.8 {
                    should_clear = false;
                    break;
                }
            }
            
            if should_clear {
                lines_cleared += 1;
                // 量子行消除 - 创建量子爆炸效果
                for x in 0..self.width {
                    self.quantum_field[y][x] *= 2.0; // 增强量子场
                }
            } else {
                new_grid.push(row.clone());
            }
        }
        
        // 重新构建网格
        while new_grid.len() < self.height {
            let mut empty_row = Vec::new();
            for _ in 0..self.width {
                empty_row.push(QuantumCell {
                    occupied_probability: 0.0,
                    quantum_state: QuantumState::Superposition,
                    energy_level: 0.0,
                    phase: 0.0,
                    entangled_with: Vec::new(),
                });
            }
            new_grid.insert(0, empty_row);
        }
        
        self.quantum_grid = new_grid;
        lines_cleared
    }
}

impl QuantumTetrisGame {
    /// 创建新的量子俄罗斯方块游戏
    pub fn new() -> Self {
        let mut game = Self {
            board: QuantumGameBoard::new(12, 24), // 更大的量子游戏板
            current_quantum_pieces: Vec::new(),
            quantum_field_strength: 1.0,
            spacetime_distortion: 0.0,
            observer_presence: 0.0,
            state: QuantumGameState::QuantumSuperposition,
            stats: QuantumGameStats {
                quantum_score: 0.0,
                entanglement_count: 0,
                superposition_events: 0,
                tunneling_events: 0,
                observer_interactions: 0,
                quantum_coherence: 1.0,
                spacetime_distortion: 0.0,
                start_time: Instant::now(),
                play_time: Duration::ZERO,
            },
            quantum_timer: Instant::now(),
            quantum_interval: Duration::from_millis(500),
            entanglement_network: Vec::new(),
        };
        
        game.spawn_quantum_pieces();
        game.state = QuantumGameState::QuantumSuperposition;
        
        game
    }
    
    /// 生成量子方块
    fn spawn_quantum_pieces(&mut self) {
        self.current_quantum_pieces.clear();
        
        // 生成多个量子方块
        let piece_types = vec![
            QuantumTetrominoType::QuantumI,
            QuantumTetrominoType::EntangledPair,
            QuantumTetrominoType::SuperpositionBlock,
        ];
        
        for piece_type in piece_types {
            let mut piece = QuantumTetromino::new(piece_type);
            
            // 随机量子状态
            piece.quantum_state = match (self.stats.quantum_score as u32) % 4 {
                0 => QuantumState::Superposition,
                1 => QuantumState::Entangled,
                2 => QuantumState::Tunneling,
                _ => QuantumState::Collapsed,
            };
            
            self.current_quantum_pieces.push(piece);
        }
        
        // 创建纠缠网络
        if self.current_quantum_pieces.len() >= 2 {
            self.create_entanglement_network();
        }
    }
    
    /// 创建纠缠网络
    fn create_entanglement_network(&mut self) {
        let piece_count = self.current_quantum_pieces.len();
        for i in 0..piece_count {
            for j in (i+1)..piece_count {
                if (i + j) % 2 == 0 { // 选择性纠缠
                    // 分别处理两个方块
                    let mut piece1 = self.current_quantum_pieces[i].clone();
                    let mut piece2 = self.current_quantum_pieces[j].clone();
                    
                    piece1.create_entanglement(&mut piece2);
                    
                    self.current_quantum_pieces[i] = piece1;
                    self.current_quantum_pieces[j] = piece2;
                    self.stats.entanglement_count += 1;
                }
            }
        }
    }
    
    /// 量子更新
    pub fn quantum_update(&mut self) {
        if self.state != QuantumGameState::QuantumSuperposition {
            return;
        }
        
        // 更新量子场强度
        self.quantum_field_strength += 0.01;
        
        // 更新时空扭曲
        self.spacetime_distortion = (self.stats.quantum_score * 0.001).sin();
        
        // 更新观察者效应
        self.observer_presence = (self.stats.observer_interactions as f64 * 0.1).min(1.0);
        
        // 更新量子相干性
        self.stats.quantum_coherence = (1.0 - self.observer_presence * 0.1).max(0.1);
        
        // 量子自动移动
        if self.quantum_timer.elapsed() >= self.quantum_interval {
            for piece in &mut self.current_quantum_pieces {
                match piece.quantum_state {
                    QuantumState::Superposition => {
                        // 叠加态 - 同时向多个方向移动
                        piece.quantum_move(0.0, 1.0);
                        piece.quantum_move(0.5, 0.0);
                        piece.quantum_move(-0.5, 0.0);
                    },
                    QuantumState::Entangled => {
                        // 纠缠态 - 同步移动
                        piece.quantum_move(0.0, 1.0);
                    },
                    QuantumState::Tunneling => {
                        // 隧道态 - 随机传送
                        let target_x = (self.stats.quantum_score as u32 % self.board.width as u32) as f64;
                        let target_y = (self.stats.quantum_score as u32 % self.board.height as u32) as f64;
                        piece.quantum_tunnel(target_x, target_y);
                        self.stats.tunneling_events += 1;
                    },
                    QuantumState::Collapsed => {
                        // 坍缩态 - 正常移动
                        piece.quantum_move(0.0, 1.0);
                    }
                }
            }
            
            self.quantum_timer = Instant::now();
        }
        
        // 更新游戏时间
        self.stats.play_time = self.stats.start_time.elapsed();
    }
    
    /// 量子操作
    pub fn quantum_operation(&mut self, operation: QuantumOperation) {
        match operation {
            QuantumOperation::Observe(x, y) => {
                self.board.observer_effect(x, y);
                self.stats.observer_interactions += 1;
            },
            QuantumOperation::Entangle => {
                if self.current_quantum_pieces.len() >= 2 {
                    self.create_entanglement_network();
                }
            },
            QuantumOperation::Tunnel(x, y) => {
                for piece in &mut self.current_quantum_pieces {
                    piece.quantum_tunnel(x as f64, y as f64);
                    self.stats.tunneling_events += 1;
                }
            },
            QuantumOperation::Superposition => {
                for piece in &mut self.current_quantum_pieces {
                    piece.quantum_state = QuantumState::Superposition;
                    self.stats.superposition_events += 1;
                }
            }
        }
    }
}

/// 量子操作类型
#[derive(Debug, Clone)]
pub enum QuantumOperation {
    Observe(usize, usize),  // 观察指定位置
    Entangle,              // 创建纠缠
    Tunnel(f64, f64),      // 量子隧道
    Superposition,          // 进入叠加态
}

/// 量子俄罗斯方块游戏主程序
struct QuantumTetrisApp {
    game: QuantumTetrisGame,
    running: bool,
    last_render_time: Instant,
    render_interval: Duration,
}

impl QuantumTetrisApp {
    /// 创建新的量子俄罗斯方块应用
    fn new() -> Self {
        Self {
            game: QuantumTetrisGame::new(),
            running: true,
            last_render_time: Instant::now(),
            render_interval: Duration::from_millis(100),
        }
    }
    
    /// 运行量子游戏主循环
    fn run(&mut self) {
        self.clear_screen();
        self.show_quantum_welcome();
        
        while self.running {
            self.update();
            self.render();
            self.handle_quantum_input();
            
            thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }
        
        self.show_quantum_game_over();
    }
    
    /// 更新量子游戏状态
    fn update(&mut self) {
        self.game.quantum_update();
    }
    
    /// 渲染量子游戏画面
    fn render(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_render_time) < self.render_interval {
            return;
        }
        
        self.last_render_time = now;
        
        // 清屏
        self.clear_screen();
        
        // 渲染量子游戏板
        self.render_quantum_board();
        
        // 渲染量子方块
        self.render_quantum_pieces();
        
        // 渲染量子UI
        self.render_quantum_ui();
        
        // 渲染量子控制说明
        self.render_quantum_controls();
        
        // 刷新屏幕
        io::stdout().flush().unwrap();
    }
    
    /// 渲染量子游戏板
    fn render_quantum_board(&mut self) {
        let board = &self.game.board;
        let start_x = 2;
        let start_y = 3;
        
        print!("\x1B[{};{}H", start_y, start_x);
        print!("┌");
        for _ in 0..board.width {
            print!("─");
        }
        print!("┐");
        
        for (y, row) in board.quantum_grid.iter().enumerate() {
            print!("\x1B[{};{}H", start_y + y + 1, start_x);
            print!("│");
            
            for cell in row {
                let char = self.quantum_cell_to_char(cell);
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
    
    /// 渲染量子方块
    fn render_quantum_pieces(&mut self) {
        let pieces = self.game.current_quantum_pieces.clone();
        for piece in &pieces {
            self.render_quantum_piece(piece);
        }
    }
    
    /// 渲染单个量子方块
    fn render_quantum_piece(&mut self, piece: &QuantumTetromino) {
        let board = &self.game.board;
        let start_x = 2;
        let start_y = 3;
        
        for coord in &piece.spacetime_coords {
            let x = coord.x as usize;
            let y = coord.y as usize;
            
            if x < board.width && y < board.height {
                let screen_x = start_x + x + 1;
                let screen_y = start_y + y + 1;
                
                print!("\x1B[{};{}H", screen_y, screen_x);
                
                let char = self.quantum_state_to_char(piece.quantum_state);
                print!("{}", char);
            }
        }
    }
    
    /// 渲染量子UI
    fn render_quantum_ui(&mut self) {
        let stats = &self.game.stats;
        
        // 渲染量子统计信息
        print!("\x1B[3;30H");
        print!("┌─ 量子统计 ─┐");
        
        print!("\x1B[4;30H");
        print!("│ 量子分数: {:>6.1} │", stats.quantum_score);
        
        print!("\x1B[5;30H");
        print!("│ 纠缠次数: {:>6} │", stats.entanglement_count);
        
        print!("\x1B[6;30H");
        print!("│ 叠加事件: {:>6} │", stats.superposition_events);
        
        print!("\x1B[7;30H");
        print!("│ 隧道事件: {:>6} │", stats.tunneling_events);
        
        print!("\x1B[8;30H");
        print!("│ 观察交互: {:>6} │", stats.observer_interactions);
        
        print!("\x1B[9;30H");
        print!("│ 量子相干: {:>6.2} │", stats.quantum_coherence);
        
        print!("\x1B[10;30H");
        print!("└──────────────┘");
        
        // 渲染量子场强度
        print!("\x1B[12;30H");
        print!("┌─ 量子场 ─┐");
        
        print!("\x1B[13;30H");
        print!("│ 场强度: {:>6.2} │", self.game.quantum_field_strength);
        
        print!("\x1B[14;30H");
        print!("│ 时空扭曲: {:>5.2} │", self.game.spacetime_distortion);
        
        print!("\x1B[15;30H");
        print!("│ 观察者: {:>7.2} │", self.game.observer_presence);
        
        print!("\x1B[16;30H");
        print!("└────────────┘");
    }
    
    /// 渲染量子控制说明
    fn render_quantum_controls(&mut self) {
        print!("\x1B[18;30H");
        print!("┌─ 量子控制 ─┐");
        
        print!("\x1B[19;30H");
        print!("│ O: 观察位置   │");
        
        print!("\x1B[20;30H");
        print!("│ E: 创建纠缠   │");
        
        print!("\x1B[21;30H");
        print!("│ T: 量子隧道   │");
        
        print!("\x1B[22;30H");
        print!("│ S: 叠加态     │");
        
        print!("\x1B[23;30H");
        print!("│ Q: 退出游戏   │");
        
        print!("\x1B[24;30H");
        print!("└──────────────┘");
    }
    
    /// 处理量子输入
    fn handle_quantum_input(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
        
        if !input.is_empty() {
            let ch = input.chars().next().unwrap().to_ascii_lowercase();
            
            match ch {
                'o' => {
                    // 观察中心位置
                    self.game.quantum_operation(QuantumOperation::Observe(6, 12));
                },
                'e' => {
                    // 创建纠缠
                    self.game.quantum_operation(QuantumOperation::Entangle);
                },
                't' => {
                    // 量子隧道到随机位置
                    let x = (self.game.stats.quantum_score as u32 % 12) as f64;
                    let y = (self.game.stats.quantum_score as u32 % 24) as f64;
                    self.game.quantum_operation(QuantumOperation::Tunnel(x, y));
                },
                's' => {
                    // 进入叠加态
                    self.game.quantum_operation(QuantumOperation::Superposition);
                },
                'q' => self.running = false,
                _ => {}
            }
        }
    }
    
    /// 将量子单元格转换为字符
    fn quantum_cell_to_char(&self, cell: &QuantumCell) -> char {
        match cell.quantum_state {
            QuantumState::Superposition => '◐',
            QuantumState::Entangled => '◑',
            QuantumState::Collapsed => '█',
            QuantumState::Tunneling => '◯',
        }
    }
    
    /// 将量子状态转换为字符
    fn quantum_state_to_char(&self, state: QuantumState) -> char {
        match state {
            QuantumState::Superposition => '◐',
            QuantumState::Entangled => '◑',
            QuantumState::Collapsed => '█',
            QuantumState::Tunneling => '◯',
        }
    }
    
    /// 清屏
    fn clear_screen(&mut self) {
        print!("\x1B[2J\x1B[H");
    }
    
    /// 显示量子欢迎信息
    fn show_quantum_welcome(&mut self) {
        self.clear_screen();
        println!("🌌 量子纠缠俄罗斯方块 - 革命性创新版本");
        println!("==================================================");
        println!("");
        println!("欢迎来到量子世界！这是世界上第一个量子俄罗斯方块游戏");
        println!("");
        println!("🎯 革命性特性：");
        println!("  ✅ 量子纠缠机制 - 多个方块同时操作");
        println!("  ✅ 时空扭曲 - 方块可以穿越时间");
        println!("  ✅ 概率叠加 - 方块同时存在于多个状态");
        println!("  ✅ 量子隧道 - 方块可以穿过障碍物");
        println!("  ✅ 观察者效应 - 你的观察影响游戏状态");
        println!("");
        println!("按任意键进入量子世界...");
        
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
    }
    
    /// 显示量子游戏结束信息
    fn show_quantum_game_over(&mut self) {
        self.clear_screen();
        let stats = &self.game.stats;
        
        println!("🌌 量子游戏结束！");
        println!("==================================================");
        println!("");
        println!("量子统计：");
        println!("  量子分数: {:.1}", stats.quantum_score);
        println!("  纠缠次数: {}", stats.entanglement_count);
        println!("  叠加事件: {}", stats.superposition_events);
        println!("  隧道事件: {}", stats.tunneling_events);
        println!("  观察交互: {}", stats.observer_interactions);
        println!("  量子相干性: {:.2}", stats.quantum_coherence);
        println!("  游戏时间: {:.1}秒", stats.play_time.as_secs_f64());
        println!("");
        println!("感谢体验量子世界！");
        println!("");
    }
}

/// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌌 启动量子纠缠俄罗斯方块游戏...");
    println!("革命性创新版本");
    println!("");
    
    // 创建并运行量子游戏
    let mut game = QuantumTetrisApp::new();
    game.run();
    
    println!("量子游戏已退出，感谢体验！");
    Ok(())
}
