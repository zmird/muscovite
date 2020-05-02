use crate::constants::*;
use std::fmt;
use std::cmp::Eq;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}->{}{}", BOARD_COLUMNS[self.from.x as usize],
               self.from.y+1, BOARD_COLUMNS[self.to.x as usize], self.to.y+1)
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Board {
    board: [[u32; 9]; 9]
}

impl Board {
    pub fn init() -> Board {
        Board {
            board: INITIAL_BOARD
        }
    }

    pub fn new(board: [[u32; 9]; 9]) -> Board {
       Board {
           board
       }
    }

    pub fn cell_type(&self, p: Position) -> u32 {
        BOARD[p.y as usize][p.x as usize]
    }

    pub fn cell_content(&self, p: Position) -> u32 {
        self.board[p.y as usize][p.x as usize]
    }

    pub fn is_empty(&self, p: Position) -> bool {
        self.cell_content(p) == E
    }

    fn filter_cells(&self, cell_type: u32) -> Vec<Position> {
        let mut cells: Vec<Position> = vec![];
        for (y, row) in self.board.iter().enumerate() {
            // println!("{:?}", row);
            for (x, cell) in row.iter().enumerate() {
                if cell == &cell_type {
                    let position = Position {
                        x: x as u32,
                        y: y as u32
                    };
                    // println!("{} {:?}", cell, position);
                    cells.push(position);
                }
            }
        }
        cells
    }

    // pub fn empty_cells(&self) -> Vec<Position> {
    //     self.filter_cells(E)
    // }

    pub fn white_cells(&self) -> Vec<Position> {
        self.filter_cells(W)
    }

    pub fn black_cells(&self) -> Vec<Position> {
        self.filter_cells(B)
    }

    pub fn king_cell(&self) -> Option<Position> {
        self.filter_cells(K).first().copied()
    }

    // pub fn move_white(&mut self, start: [usize; 2], end: [usize; 2]) {
    //     self.board[start[0]][start[1]] = E;
    //     self.board[end[0]][end[1]] = W;
    // }

    // pub fn move_black(&mut self, start: [usize; 2], end: [usize; 2]) {
    //     self.board[start[0]][start[1]] = E;
    //     self.board[end[0]][end[1]] = B;
    // }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = String::from("");
        out.push_str("    a   b   c   d   e   f   g   h   i\n");
        out.push_str("  ┌───┬───┬───┬───┬───┬───┬───┬───┬───┐\n");
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if x == 0 {
                   out.push_str(&format!("{} ", y+1));
                }
                if cell == &W {
                    out.push_str("│ ○ ");
                } else if cell == &B {
                    out.push_str("│ ● ");
                } else if cell == &K {
                    out.push_str("│ △ ");
                } else {
                    out.push_str("│   ");
                }
                if x == 8 {
                    out.push_str("│\n")
                }
            }
            if y < 8 {
                out.push_str("  ├───┼───┼───┼───┼───┼───┼───┼───┼───┤\n");
            }
        }
        out.push_str("  └───┴───┴───┴───┴───┴───┴───┴───┴───┘");
        write!(f, "{}", out)
    }
}

pub enum Status {
    WIN,
    LOSS,
    DRAW,
    ONGOING
}


pub struct State {
    pub board: Board,
    pub turn: String,
    pub history: Vec<Board>,
    pub status: Status,
}

impl State {
    pub fn init() -> State {
        State {
            board: Board::init(),
            turn: WHITE.to_string(),
            history: vec![],
            status: Status::ONGOING
        }
    }
}
