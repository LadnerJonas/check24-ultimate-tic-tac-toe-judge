use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    X = 1,
    O = 2,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoardState {
    InProgress,
    Won(Player),
    Draw,
}

pub struct UltimateTicTacToe {
    small_boards: [u32; 9], // 9 boards × 9 cells × 2 bits = 162 bits
    pub small_board_state: [BoardState; 9], // track win/draw status
    global_state: BoardState,
    current_player: Player,
    next_small_board: Option<usize>, // 0–8 or None
}

impl UltimateTicTacToe {
    pub fn new() -> Self {
        Self {
            small_boards: [0; 9],
            small_board_state: [BoardState::InProgress; 9],
            global_state: BoardState::InProgress,
            current_player: Player::X,
            next_small_board: None,
        }
    }

    pub fn play(&mut self, global_row: usize, global_col: usize) -> Result<(), &'static str> {
        if global_row >= 9 || global_col >= 9 {
            return Err("Invalid coordinates");
        }

        let small_board = (global_row / 3) * 3 + (global_col / 3);
        let cell_idx = (global_row % 3) * 3 + (global_col % 3);

        if let Some(forced) = self.next_small_board {
            if forced != small_board && self.small_board_state[forced] == BoardState::InProgress {
                return Err("Must play in the forced small board");
            }
        }

        if self.small_board_state[small_board] != BoardState::InProgress {
            return Err("This small board is closed");
        }

        if get_cell(self.small_boards[small_board], cell_idx) != 0 {
            return Err("Cell is already taken");
        }

        set_cell(
            &mut self.small_boards[small_board],
            cell_idx,
            self.current_player as u8,
        );

        let board = self.small_boards[small_board];
        self.small_board_state[small_board] = Self::check_board(board);

        let meta_board = self.small_board_state.map(|s| match s {
            BoardState::Won(p) => p as u8,
            _ => 0,
        });
        self.global_state = Self::check_board_mask(&meta_board);

        let next = cell_idx;
        self.next_small_board = match self.small_board_state[next] {
            BoardState::InProgress => Some(next),
            _ => None,
        };

        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        Ok(())
    }

    fn check_board(board: u32) -> BoardState {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // rows
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // cols
            [0, 4, 8],
            [2, 4, 6], // diags
        ];

        for line in LINES {
            let a = get_cell(board, line[0]);
            if a != 0 && a == get_cell(board, line[1]) && a == get_cell(board, line[2]) {
                return BoardState::Won(if a == 1 { Player::X } else { Player::O });
            }
        }

        if (0..9).all(|i| get_cell(board, i) != 0) {
            BoardState::Draw
        } else {
            BoardState::InProgress
        }
    }

    fn check_board_mask(cells: &[u8; 9]) -> BoardState {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in LINES {
            let a = cells[line[0]];
            if a != 0 && a == cells[line[1]] && a == cells[line[2]] {
                return BoardState::Won(if a == 1 { Player::X } else { Player::O });
            }
        }

        if cells.iter().all(|&c| c != 0) {
            BoardState::Draw
        } else {
            BoardState::InProgress
        }
    }

    pub fn check_winner(&self) -> Result<Player, &'static str> {
        let mut global_winner = None;

        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in lines {
            let a = self.small_board_state[line[0]];
            if a != BoardState::Draw
                && a != BoardState::InProgress
                && a == self.small_board_state[line[1]]
                && a == self.small_board_state[line[2]]
            {
                global_winner = Some(a);
                break;
            }
        }

        if global_winner.is_some() {
            Ok(match global_winner.unwrap() {
                BoardState::Won(Player::X) => Player::X,
                BoardState::Won(Player::O) => Player::O,
                _ => unreachable!(),
            })
        } else if self
            .small_board_state
            .iter()
            .all(|&s| s != BoardState::InProgress)
        {
            Err("Draw")
        } else {
            Err("InProgress")
        }
    }
    pub fn print_board(&self) {
        let mut rows = vec![];

        for big_row in 0..3 {
            for small_row in 0..3 {
                let mut line = String::new();
                for big_col in 0..3 {
                    let sb_idx = big_row * 3 + big_col;
                    let mut small_cells = vec![];
                    for small_col in 0..3 {
                        let cell_idx = small_row * 3 + small_col;
                        let val = get_cell(self.small_boards[sb_idx], cell_idx);
                        small_cells.push(match val {
                            1 => 'X',
                            2 => 'O',
                            _ => ' ',
                        });
                    }
                    line.push_str(&format!(
                        "{}│{}│{}",
                        small_cells[0], small_cells[1], small_cells[2]
                    ));
                    if big_col < 2 {
                        line.push('┃');
                    }
                }
                rows.push(line);
            }
            if big_row < 2 {
                rows.push("━┿━┿━╋━┿━┿━╋━┿━┿━".to_string());
            }
        }

        for row in rows {
            println!("{}", row);
        }
        println!();
    }
}

fn get_cell(board: u32, idx: usize) -> u8 {
    ((board >> (idx * 2)) & 0b11) as u8
}

fn set_cell(board: &mut u32, idx: usize, value: u8) {
    let mask = !(0b11 << (idx * 2));
    *board = (*board & mask) | ((value as u32) << (idx * 2));
}

impl fmt::Debug for UltimateTicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Global State: {:?}\n", self.global_state)?;
        for (i, &state) in self.small_board_state.iter().enumerate() {
            write!(f, "Small Board {}: {:?}\n", i, state)?;
        }
        write!(f, "Next Board: {:?}\n", self.next_small_board)?;
        write!(f, "Current Player: {:?}\n", self.current_player)
    }
}
