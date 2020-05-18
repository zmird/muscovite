use crate::constants::*;
use crate::rules::captures;
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn apply_move(&mut self, m: &Move) {
        let cell_content: u32 = self.cell_content(m.from);
        self.board[m.to.y as usize][m.to.x as usize] = cell_content;
        self.board[m.from.y as usize][m.from.x as usize] = E;
        let checkers_captured = captures(&self, m);
        for checker in checkers_captured {
            self.board[checker.y as usize][checker.x as usize] = E;
        }
    }

    pub fn cell_type(&self, p: Position) -> u32 {
        BOARD[p.y as usize][p.x as usize]
    }

    pub fn cell_content(&self, p: Position) -> u32 {
        self.board[p.y as usize][p.x as usize]
    }

    pub fn cell_color(&self, p: Position) -> Option<String> {
        let content: u32 = self.cell_content(p);
        if content == W || content == K {
            return Some(WHITE.to_string());
        }
        else if content == B {
            return Some(BLACK.to_string());
        }
        None
    }

    pub fn surrounding_cells(&self, p: Position) -> [Option<Position>; 4] {
        // Up Down Right Left
        let mut s: [Option<Position>; 4] = [None, None, None, None];

        // Up
        if p.y > 0 {
            s[0] = Some(Position { x: p.x, y: p.y-1 });
        }
        // Down
        if p.y < 8 {
            s[1] = Some(Position { x: p.x, y: p.y+1 });
        }
        // Right
        if p.x < 8 {
            s[2] = Some(Position { x: p.x+1, y: p.y });
        }
        // Left
        if p.x > 0 {
            s[3] = Some(Position { x: p.x-1, y: p.y });
        }
        return s;
    }

    pub fn surrounding_diagonal_cells(&self, p: Position) -> [Option<Position>; 4] {
        let mut s: [Option<Position>; 4] = [None, None, None, None];

        // Up Right
        if p.y > 0 && p.x < 8 {
            s[0] = Some(Position { x: p.x+1, y: p.y-1 });
        }
        // Up Left
        if p.y > 0 && p.x > 0 {
            s[1] = Some(Position { x: p.x-1, y: p.y-1 });
        }
        // Down Right
        if p.y < 8 && p.x < 8 {
            s[2] = Some(Position { x: p.x+1, y: p.y+1 });
        }
        // Down Left
        if p.y < 8 && p.x > 0 {
            s[3] = Some(Position { x: p.x-1, y: p.y+1 });
        }

        return s;
    }

    pub fn upper_cell(&self, p: Position) -> Option<Position> {
        if p.x > 8 || p.y > 8 || p.y == 0 {
            None
        } else {
            Some(Position { x: p.x, y: p.y-1 })
        }
    }

    pub fn lower_cell(&self, p: Position) -> Option<Position> {
        if p.x > 8 || p.y > 8 || p.y == 8 {
            None
        } else {
            Some(Position { x: p.x, y: p.y+1 })
        }
    }

    pub fn right_cell(&self, p: Position) -> Option<Position> {
        if p.x > 8 || p.y > 8 || p.x == 8 {
            None
        } else {
            Some(Position { x: p.x+1, y: p.y })
        }
    }

    pub fn left_cell(&self, p: Position) -> Option<Position> {
        if p.x > 8 || p.y > 8 || p.x == 0 {
            None
        } else {
            Some(Position { x: p.x-1, y: p.y })
        }
    }

    pub fn filter_cells(&self, cell_content: u32) -> Vec<Position> {
        let mut cells: Vec<Position> = vec![];
        for (y, row) in self.board.iter().enumerate() {
            // println!("{:?}", row);
            for (x, cell) in row.iter().enumerate() {
                if cell == &cell_content {
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

    pub fn king_cell(&self) -> Option<Position> {
        self.filter_cells(K).first().copied()
    }

    pub fn white_cells(&self) -> Vec<Position> {
        self.filter_cells(W)
    }

    pub fn black_cells(&self) -> Vec<Position> {
        self.filter_cells(B)
    }

    pub fn is_empty(&self, p: Position) -> bool {
        self.cell_content(p) == E
    }

    pub fn is_king_in_throne(&self) -> bool {
        let king = self.king_cell();
        if king.is_none() {
            false
        } else {
            self.cell_type(king.unwrap()) == T
        }

    }

    pub fn is_king_next_throne(&self) -> bool {
        let king = self.king_cell();
        if king.is_none() {
            return false;
        }
        self.surrounding_cells(king.unwrap()).iter()
            .any(|cell| cell.is_some() && self.cell_type(cell.unwrap()) == T)
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    WIN,
    LOSS,
    DRAW,
    ONGOING
}

#[derive(Clone)]
pub struct State {
    pub color: String,
    pub board: Board,
    pub turn: String,
    pub history: Vec<Board>,
    pub status: Status,
}

impl State {
    pub fn init(color: String) -> State {
        State {
            color,
            board: Board::init(),
            turn: WHITE.to_string(),
            history: vec![Board::init()],
            status: Status::ONGOING
        }
    }

    pub fn apply_move(&mut self, m: &Move) {
        self.history.push(self.board);
        self.board.apply_move(&m);
    }
}
