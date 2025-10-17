//! Tic-Tac-Toe 井字棋游戏 - 集成所有生命游戏的活力运行系统
//! 
//! 本程序不仅提供经典的井字棋游戏，还集成了所有已实现的生命游戏，
//! 确保它们都能有活力地运行，展示细胞自动机的魅力

use gameboy_emulator::entropy::{
    EntropyManager, EntropyError,
    entropy_pool::PooledEntropy,
};

use std::time::{Duration, Instant};
use std::thread;
use std::io;
use std::process::Command;

/// 井字棋游戏板
#[derive(Clone, Debug)]
struct TicTacToeBoard {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
    game_state: GameState,
    move_count: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Debug, PartialEq)]
enum GameState {
    Playing,
    Win(Player),
    Draw,
}

impl TicTacToeBoard {
    fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current_player: Player::X,
            game_state: GameState::Playing,
            move_count: 0,
        }
    }
    
    fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        if row >= 3 || col >= 3 {
            return Err("位置超出范围".to_string());
        }
        
        if self.board[row][col].is_some() {
            return Err("该位置已被占用".to_string());
        }
        
        if self.game_state != GameState::Playing {
            return Err("游戏已结束".to_string());
        }
        
        self.board[row][col] = Some(self.current_player);
        self.move_count += 1;
        
        // 检查胜利条件
        if self.check_win(row, col) {
            self.game_state = GameState::Win(self.current_player);
        } else if self.move_count == 9 {
            self.game_state = GameState::Draw;
        } else {
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
        
        Ok(())
    }
    
    fn check_win(&self, row: usize, col: usize) -> bool {
        let player = self.board[row][col].unwrap();
        
        // 检查行
        if self.board[row][0] == Some(player) && 
           self.board[row][1] == Some(player) && 
           self.board[row][2] == Some(player) {
            return true;
        }
        
        // 检查列
        if self.board[0][col] == Some(player) && 
           self.board[1][col] == Some(player) && 
           self.board[2][col] == Some(player) {
            return true;
        }
        
        // 检查对角线
        if row == col && 
           self.board[0][0] == Some(player) && 
           self.board[1][1] == Some(player) && 
           self.board[2][2] == Some(player) {
            return true;
        }
        
        if row + col == 2 && 
           self.board[0][2] == Some(player) && 
           self.board[1][1] == Some(player) && 
           self.board[2][0] == Some(player) {
            return true;
        }
        
        false
    }
    
    fn display(&self) {
        println!("🎮 井字棋游戏");
        println!("当前玩家: {}", match self.current_player {
            Player::X => "❌",
            Player::O => "⭕",
        });
        println!("┌───┬───┬───┐");
        
        for (i, row) in self.board.iter().enumerate() {
            print!("│");
            for cell in row {
                match cell {
                    Some(Player::X) => print!(" ❌ │"),
                    Some(Player::O) => print!(" ⭕ │"),
                    None => print!("   │"),
                }
            }
            println!();
            if i < 2 {
                println!("├───┼───┼───┤");
            }
        }
        
        println!("└───┴───┴───┘");
        
        match self.game_state {
            GameState::Playing => println!("游戏进行中..."),
            GameState::Win(player) => println!("🎉 玩家 {:?} 获胜！", player),
            GameState::Draw => println!("🤝 平局！"),
        }
    }
    
    fn get_available_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j].is_none() {
                    moves.push((i, j));
                }
            }
        }
        moves
    }
}

/// AI玩家
struct AI {
    difficulty: Difficulty,
    entropy_pool: PooledEntropy,
}

#[derive(Debug, Clone, Copy)]
enum Difficulty {
    Easy,    // 随机移动
    Medium,  // 简单策略
    Hard,    // 高级策略
}

impl AI {
    fn new(difficulty: Difficulty) -> Result<Self, EntropyError> {
        let mut entropy_pool = PooledEntropy::new(1024);
        
        // 初始化熵池
        let mut entropy_manager = EntropyManager::new();
        let _ = entropy_manager.collect_and_optimize();
        let random_data = entropy_manager.generate_random(256)?;
        entropy_pool.add_entropy_source(&random_data);
        
        Ok(Self {
            difficulty,
            entropy_pool,
        })
    }
    
    fn get_move(&mut self, board: &TicTacToeBoard) -> Result<(usize, usize), EntropyError> {
        let available_moves = board.get_available_moves();
        
        if available_moves.is_empty() {
            return Err(EntropyError::InsufficientEntropy);
        }
        
        match self.difficulty {
            Difficulty::Easy => {
                // 随机选择
                let random_index = self.entropy_pool.get_random_range(0, available_moves.len() as u32) as usize;
                Ok(available_moves[random_index])
            }
            Difficulty::Medium => {
                // 简单策略：优先中心，然后角落，最后边缘
                let center = (1, 1);
                let corners = [(0, 0), (0, 2), (2, 0), (2, 2)];
                let edges = [(0, 1), (1, 0), (1, 2), (2, 1)];
                
                if available_moves.contains(&center) {
                    Ok(center)
                } else if let Some(&corner) = corners.iter().find(|&&pos| available_moves.contains(&pos)) {
                    Ok(corner)
                } else if let Some(&edge) = edges.iter().find(|&&pos| available_moves.contains(&pos)) {
                    Ok(edge)
                } else {
                    let random_index = self.entropy_pool.get_random_range(0, available_moves.len() as u32) as usize;
                    Ok(available_moves[random_index])
                }
            }
            Difficulty::Hard => {
                // 高级策略：使用minimax算法
                self.minimax_move(board)
            }
        }
    }
    
    fn minimax_move(&mut self, board: &TicTacToeBoard) -> Result<(usize, usize), EntropyError> {
        let available_moves = board.get_available_moves();
        let mut best_score = i32::MIN;
        let mut best_move = available_moves[0];
        
        for &(row, col) in &available_moves {
            let mut test_board = board.clone();
            test_board.make_move(row, col).unwrap();
            
            let score = self.minimax(&test_board, false, 0);
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }
        
        Ok(best_move)
    }
    
    fn minimax(&self, board: &TicTacToeBoard, is_maximizing: bool, depth: u8) -> i32 {
        match board.game_state {
            GameState::Win(Player::O) => return 10 - depth as i32, // AI获胜
            GameState::Win(Player::X) => return -10 + depth as i32, // 玩家获胜
            GameState::Draw => return 0,
            GameState::Playing => {}
        }
        
        if depth >= 9 {
            return 0;
        }
        
        let available_moves = board.get_available_moves();
        if is_maximizing {
            let mut max_eval = i32::MIN;
            for &(row, col) in &available_moves {
                let mut test_board = board.clone();
                test_board.make_move(row, col).unwrap();
                let eval = self.minimax(&test_board, false, depth + 1);
                max_eval = max_eval.max(eval);
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &(row, col) in &available_moves {
                let mut test_board = board.clone();
                test_board.make_move(row, col).unwrap();
                let eval = self.minimax(&test_board, true, depth + 1);
                min_eval = min_eval.min(eval);
            }
            min_eval
        }
    }
}

/// 生命游戏管理器
struct LifeGameManager {
    games: Vec<LifeGameInfo>,
    entropy_manager: EntropyManager,
}

#[derive(Debug, Clone)]
struct LifeGameInfo {
    name: String,
    executable: String,
    description: String,
    is_active: bool,
    last_run: Option<Instant>,
}

impl LifeGameManager {
    fn new() -> Result<Self, EntropyError> {
        let mut entropy_manager = EntropyManager::new();
        let _ = entropy_manager.collect_and_optimize();
        
        let games = vec![
            LifeGameInfo {
                name: "全新的生命游戏".to_string(),
                executable: "new-life-game".to_string(),
                description: "基于外部熵源的细胞自动机模拟".to_string(),
                is_active: false,
                last_run: None,
            },
            LifeGameInfo {
                name: "甜甜的生命游戏".to_string(),
                executable: "sweet-life-game".to_string(),
                description: "凸优化版本的生命游戏".to_string(),
                is_active: false,
                last_run: None,
            },
            LifeGameInfo {
                name: "优化的生命游戏".to_string(),
                executable: "sweet-life-optimized".to_string(),
                description: "性能优化版本的生命游戏".to_string(),
                is_active: false,
                last_run: None,
            },
        ];
        
        Ok(Self {
            games,
            entropy_manager,
        })
    }
    
    fn list_games(&self) {
        println!("🎮 可用的生命游戏:");
        println!("==================================================");
        
        for (i, game) in self.games.iter().enumerate() {
            let status = if game.is_active { "🟢 运行中" } else { "⚪ 未运行" };
            println!("{}. {} - {}", i + 1, game.name, status);
            println!("   描述: {}", game.description);
            if let Some(last_run) = game.last_run {
                println!("   最后运行: {:.1}秒前", last_run.elapsed().as_secs_f64());
            }
            println!();
        }
    }
    
    fn run_game(&mut self, index: usize) -> Result<(), String> {
        if index >= self.games.len() {
            return Err("无效的游戏索引".to_string());
        }
        
        let game = &mut self.games[index];
        println!("🚀 启动 {}...", game.name);
        
        // 检查可执行文件是否存在
        let executable_path = format!("target/release/{}", game.executable);
        if !std::path::Path::new(&executable_path).exists() {
            println!("⚠️  可执行文件不存在，尝试编译...");
            let output = Command::new("cargo")
                .args(&["build", "--bin", &game.executable, "--release"])
                .output()
                .map_err(|e| format!("编译失败: {}", e))?;
            
            if !output.status.success() {
                return Err(format!("编译失败: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
        
        // 运行游戏
        let output = Command::new(&executable_path)
            .output()
            .map_err(|e| format!("运行失败: {}", e))?;
        
        game.is_active = true;
        game.last_run = Some(Instant::now());
        
        println!("✅ {} 运行完成", game.name);
        if !output.stdout.is_empty() {
            println!("输出:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            println!("错误:\n{}", String::from_utf8_lossy(&output.stderr));
        }
        
        Ok(())
    }
    
    fn run_all_games(&mut self) -> Result<(), String> {
        println!("🌟 启动所有生命游戏，让它们充满活力！");
        println!("==================================================");
        
        for i in 0..self.games.len() {
            match self.run_game(i) {
                Ok(()) => {
                    println!("✅ {} 成功启动", self.games[i].name);
                    thread::sleep(Duration::from_millis(500)); // 短暂延迟
                }
                Err(e) => {
                    println!("❌ {} 启动失败: {}", self.games[i].name, e);
                }
            }
        }
        
        println!("\n🎉 所有生命游戏已启动！");
        Ok(())
    }
    
    fn get_entropy_stats(&self) -> String {
        let stats = self.entropy_manager.get_entropy_stats();
        format!(
            "熵源数量: {}, 池大小: {} 字节, 分布质量: {:.3}, 量子强度: {:.3}",
            stats.source_count, stats.pool_size, 
            stats.optimizer_stats.distribution_quality,
            stats.quantum_stats.post_quantum_strength
        )
    }
}

/// 主游戏系统
struct GameSystem {
    tic_tac_toe: TicTacToeBoard,
    ai: AI,
    life_manager: LifeGameManager,
    entropy_manager: EntropyManager,
    stats: GameStats,
}

#[derive(Debug)]
struct GameStats {
    games_played: u32,
    ai_wins: u32,
    player_wins: u32,
    draws: u32,
    total_moves: u32,
    start_time: Instant,
}

impl GameSystem {
    fn new() -> Result<Self, EntropyError> {
        let tic_tac_toe = TicTacToeBoard::new();
        let ai = AI::new(Difficulty::Hard)?;
        let life_manager = LifeGameManager::new()?;
        let entropy_manager = EntropyManager::new();
        
        Ok(Self {
            tic_tac_toe,
            ai,
            life_manager,
            entropy_manager,
            stats: GameStats {
                games_played: 0,
                ai_wins: 0,
                player_wins: 0,
                draws: 0,
                total_moves: 0,
                start_time: Instant::now(),
            },
        })
    }
    
    fn play_tic_tac_toe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎮 欢迎来到井字棋游戏！");
        println!("选择难度:");
        println!("1. 简单 (随机移动)");
        println!("2. 中等 (简单策略)");
        println!("3. 困难 (高级AI)");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let difficulty = match input.trim() {
            "1" => Difficulty::Easy,
            "2" => Difficulty::Medium,
            "3" => Difficulty::Hard,
            _ => Difficulty::Medium,
        };
        
        self.ai = AI::new(difficulty)?;
        println!("✅ 难度设置为: {:?}", difficulty);
        
        loop {
            self.tic_tac_toe.display();
            
            if self.tic_tac_toe.game_state != GameState::Playing {
                self.handle_game_end();
                break;
            }
            
            if self.tic_tac_toe.current_player == Player::X {
                // 玩家回合
                println!("请输入位置 (行 列，例如: 1 1):");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() != 2 {
                    println!("❌ 请输入两个数字，用空格分隔");
                    continue;
                }
                
                let row: usize = parts[0].parse().map_err(|_| "无效的行")?;
                let col: usize = parts[1].parse().map_err(|_| "无效的列")?;
                
                match self.tic_tac_toe.make_move(row, col) {
                    Ok(()) => {
                        self.stats.total_moves += 1;
                        println!("✅ 移动成功");
                    }
                    Err(e) => {
                        println!("❌ {}", e);
                        continue;
                    }
                }
            } else {
                // AI回合
                println!("🤖 AI正在思考...");
                thread::sleep(Duration::from_millis(1000));
                
                match self.ai.get_move(&self.tic_tac_toe) {
                    Ok((row, col)) => {
                        self.tic_tac_toe.make_move(row, col).unwrap();
                        self.stats.total_moves += 1;
                        println!("🤖 AI选择了位置 ({}, {})", row, col);
                    }
                    Err(e) => {
                        println!("❌ AI错误: {}", e);
                        break;
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(500));
        }
        
        Ok(())
    }
    
    fn handle_game_end(&mut self) {
        self.stats.games_played += 1;
        
        match self.tic_tac_toe.game_state {
            GameState::Win(Player::X) => {
                self.stats.player_wins += 1;
                println!("🎉 恭喜！你赢了！");
            }
            GameState::Win(Player::O) => {
                self.stats.ai_wins += 1;
                println!("🤖 AI获胜！");
            }
            GameState::Draw => {
                self.stats.draws += 1;
                println!("🤝 平局！");
            }
            _ => {}
        }
        
        println!("是否再玩一局？(y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().to_lowercase() == "y" {
            self.tic_tac_toe = TicTacToeBoard::new();
        }
    }
    
    fn show_menu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            println!("\n🎮 游戏系统主菜单");
            println!("==============================");
            println!("1. 玩井字棋");
            println!("2. 查看生命游戏");
            println!("3. 运行所有生命游戏");
            println!("4. 运行特定生命游戏");
            println!("5. 查看统计信息");
            println!("6. 查看熵源信息");
            println!("0. 退出");
            println!("==============================");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "1" => {
                    self.play_tic_tac_toe()?;
                }
                "2" => {
                    self.life_manager.list_games();
                }
                "3" => {
                    self.life_manager.run_all_games()?;
                }
                "4" => {
                    self.life_manager.list_games();
                    println!("请输入游戏编号:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let index: usize = input.trim().parse().map_err(|_| "无效编号")?;
                    self.life_manager.run_game(index - 1)?;
                }
                "5" => {
                    self.show_stats();
                }
                "6" => {
                    println!("🔬 熵源系统信息:");
                    println!("{}", self.life_manager.get_entropy_stats());
                }
                "0" => {
                    println!("👋 再见！");
                    break;
                }
                _ => {
                    println!("❌ 无效选择");
                }
            }
        }
        
        Ok(())
    }
    
    fn show_stats(&self) {
        println!("\n📊 游戏统计信息");
        println!("==============================");
        println!("井字棋游戏:");
        println!("  总游戏数: {}", self.stats.games_played);
        println!("  玩家获胜: {}", self.stats.player_wins);
        println!("  AI获胜: {}", self.stats.ai_wins);
        println!("  平局: {}", self.stats.draws);
        println!("  总移动数: {}", self.stats.total_moves);
        
        if self.stats.games_played > 0 {
            let win_rate = (self.stats.player_wins as f64 / self.stats.games_played as f64) * 100.0;
            println!("  玩家胜率: {:.1}%", win_rate);
        }
        
        println!("\n生命游戏:");
        for game in &self.life_manager.games {
            let status = if game.is_active { "🟢 运行中" } else { "⚪ 未运行" };
            println!("  {}: {}", game.name, status);
        }
        
        println!("\n系统运行时间: {:.1}秒", self.stats.start_time.elapsed().as_secs_f64());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Tic-Tac-Toe 井字棋游戏系统");
    println!("集成所有生命游戏的活力运行");
    println!("==================================================");
    
    let mut game_system = GameSystem::new()?;
    
    println!("🌟 系统初始化完成！");
    println!("🔬 熵源系统已激活");
    println!("🎮 所有生命游戏已准备就绪");
    println!();
    
    game_system.show_menu()?;
    
    Ok(())
}
