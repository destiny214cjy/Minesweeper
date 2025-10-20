use rand::prelude::*;

pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 9;
pub const MINES: usize = 10;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Covered,
    Uncovered,
    Flagged,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub is_mine: bool,
    pub neighbor_mines: u8,
    pub state: CellState,
}

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub first_click: bool,
    pub game_over: bool,
}

impl Board {
    pub fn new() -> Self {
        let cells = vec![
            vec![
                Cell {
                    is_mine: false,
                    neighbor_mines: 0,
                    state: CellState::Covered,
                };
                WIDTH
            ];
            HEIGHT
        ];
        Self {
            cells,
            first_click: true,
            game_over: false,
        }
    }

    // 放置雷，保证第一次点击安全
    pub fn place_mines(&mut self, safe_x: usize, safe_y: usize) {
        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;

        while mines_placed < MINES {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);

            if (x == safe_x && y == safe_y) || self.cells[y][x].is_mine {
                continue;
            }

            self.cells[y][x].is_mine = true;
            mines_placed += 1;
        }

        // 计算邻近雷数
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x].is_mine {
                    continue;
                }
                let mut count = 0;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0
                            && nx < WIDTH as i32
                            && ny >= 0
                            && ny < HEIGHT as i32
                            && self.cells[ny as usize][nx as usize].is_mine
                        {
                            count += 1;
                        }
                    }
                }
                self.cells[y][x].neighbor_mines = count;
            }
        }
    }

    // 翻开格子
    pub fn uncover(&mut self, x: usize, y: usize) {
        if self.cells[y][x].state != CellState::Covered {
            return;
        }

        self.cells[y][x].state = CellState::Uncovered;

        if self.cells[y][x].is_mine {
            self.game_over = true;
            return;
        }

        // 自动展开空白格
        if self.cells[y][x].neighbor_mines == 0 {
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                        self.uncover(nx as usize, ny as usize);
                    }
                }
            }
        }
    }

    // 插旗
    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        let cell = &mut self.cells[y][x];
        match cell.state {
            CellState::Covered => cell.state = CellState::Flagged,
            CellState::Flagged => cell.state = CellState::Covered,
            _ => {}
        }
    }

    // 双胜利判定：非雷全翻开 或 所有雷被旗标记
    pub fn check_win_dual(&self) -> bool {
        // 游戏刚开始，没有雷也不可能胜利
        if self.first_click {
            return false;
        }

        let mut all_non_mines_uncovered = true;
        let mut all_mines_flagged = true;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = &self.cells[y][x];

                if !cell.is_mine && cell.state != CellState::Uncovered {
                    all_non_mines_uncovered = false;
                }

                if cell.is_mine && cell.state != CellState::Flagged {
                    all_mines_flagged = false;
                }
            }
        }

        all_non_mines_uncovered || all_mines_flagged
    }
}
