extern crate rand;
use macroquad::prelude::Color;

use rand::Rng;
use rust_grid::*;
use std::time::Instant;

#[derive(PartialEq)]
pub enum GameState {
    Ongoing,
    Win,
    Fail,
}

#[derive(PartialEq, Copy, Clone)]
pub enum CellState {
    Number(u8),
    Mine,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub state: CellState,
    pub flagged: bool,
    pub revealed: bool,
}

pub struct Board {
    pub grid: Grid<Cell>,
    pub mine_count: usize,
    pub revealed_cell_count: usize,
    pub flag_count: usize,
    pub start_time: Instant,
    pub state: GameState,
    pub color: Color,
    pub hovering: (usize, usize),
}

impl Board {
    pub fn new((width, height): (usize, usize), mine_count: usize, color: Color) -> Self {
        let cell = Cell {
            state: CellState::Number(0),
            flagged: false,
            revealed: false,
        };

        let mut grid = Grid::new(width, height, cell);

        let mut rng = rand::thread_rng();
        for _ in 0..mine_count {
            loop {
                let (width, height) = grid.size();

                let coordinates = (rng.gen_range(0..width), rng.gen_range(0..height));

                if grid[coordinates].state == CellState::Mine {
                    continue;
                } else {
                    grid[coordinates].state = CellState::Mine;
                    break;
                };
            }
        }

        for x in 0..width {
            for y in 0..height {
                if grid[y][x].state == CellState::Mine {
                    continue;
                };

                let mut neighbouring_mines: u8 = 0;

                for offset_x in -1..=1 as isize {
                    if (x == 0 && offset_x == -1) || (x == width - 1 && offset_x == 1) {
                        continue;
                    };

                    for offset_y in -1..=1 as isize {
                        if (y == 0 && offset_y == -1) || (y == height - 1 && offset_y == 1) {
                            continue;
                        };

                        let coordinates = (
                            (x as isize + offset_x) as usize,
                            (y as isize + offset_y) as usize,
                        );

                        if grid[coordinates].state == CellState::Mine {
                            neighbouring_mines += 1;
                        };
                    }
                }

                grid[y][x].state = CellState::Number(neighbouring_mines);
            }
        }

        Board {
            grid,
            mine_count,
            revealed_cell_count: 0,
            flag_count: 0,
            start_time: Instant::now(),
            state: GameState::Ongoing,
            color,
            hovering: (0, 0),
        }
    }

    pub fn reveal_cell(&mut self, coordinates: (usize, usize)) {
        let (width, height) = self.grid.size();

        if self.state != GameState::Ongoing {
            return;
        };

        if self.grid[coordinates].flagged {
            return;
        };

        if self.revealed_cell_count == 0 && self.grid[coordinates].state != CellState::Number(0) {
            let (width, height) = self.grid.size();

            loop {
                let new_board = Board::new((width, height), self.mine_count, self.color);
                if new_board.grid[coordinates].state == CellState::Number(0) {
                    self.grid = new_board.grid;
                    break;
                };
            }
        }

        if self.grid[coordinates].state == CellState::Mine {
            self.state = GameState::Fail;
            return;
        };

        if self.grid[coordinates].revealed {
            return;
        };

        self.grid[coordinates].revealed = true;
        let mut revealed_last_round = 1;
        loop {
            if revealed_last_round == 0 {
                break;
            }

            self.revealed_cell_count += revealed_last_round;
            revealed_last_round = 0;

            for x in 0..width {
                for y in 0..height {
                    if self.grid[y][x].revealed {
                        continue;
                    };

                    let mut has_neighbours = false;
                    for offset_x in -1..=1 {
                        if has_neighbours {
                            break;
                        };

                        if (x == 0 && offset_x == -1) || (x == width - 1 && offset_x == 1) {
                            continue;
                        };

                        for offset_y in -1..=1 {
                            if (y == 0 && offset_y == -1) || (y == height - 1 && offset_y == 1) {
                                continue;
                            };

                            let coordinates = (
                                (x as isize + offset_x) as usize,
                                (y as isize + offset_y) as usize,
                            );

                            if self.grid[coordinates].state == CellState::Number(0)
                                && self.grid[coordinates].revealed
                            {
                                has_neighbours = true;
                                break;
                            };
                        }
                    }

                    if has_neighbours {
                        revealed_last_round += 1;
                        self.grid[y][x].revealed = true;

                        if self.grid[y][x].flagged {
                            self.grid[y][x].flagged = false;
                            self.flag_count -= 1;
                        }
                    };
                }
            }
        }

        if width * height - self.revealed_cell_count == self.mine_count {
            self.state = GameState::Win;
            return;
        };
    }

    pub fn toggle_flag(&mut self, coordinates: (usize, usize)) {
        if self.state != GameState::Ongoing {
            return;
        };

        if self.grid[coordinates].revealed {
            return;
        };

        if self.grid[coordinates].flagged {
            self.flag_count -= 1;
            self.grid[coordinates].flagged = false;
        } else {
            self.flag_count += 1;
            self.grid[coordinates].flagged = true;
        };
    }
}
