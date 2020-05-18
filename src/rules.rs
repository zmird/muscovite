use crate::game::{Move, Position, Status, State, Board};
use crate::constants::*;
use log::debug;

// Returns the opposite color
pub fn get_opposite_color(color: &String) -> String {
    if color == WHITE {
        BLACK.to_string()
    } else {
        WHITE.to_string()
    }
}

// Returns a cell closer by one to destination
fn get_one_cell_closer(from: Position, to: Position) -> Option<Position> {
    if from.x == to.x && from.y < from.y {
        return Some(Position { x: from.x, y: from.y+1 });
    }
    if from.x == to.x && from.y > from.y {
        return Some(Position { x: from.x, y: from.y-1 });
    }
    if from.y == to.y && from.x < from.x {
        return Some(Position { x: from.x+1, y: from.y });
    }
    if from.y == to.y && from.x < from.x {
        return Some(Position { x: from.x-1, y: from.y });
    }
    return None;
}

// Returns true if a cell is a barrier
pub fn is_barrier(board: &Board, cell: Position) -> bool {
    let cell_type = board.cell_type(cell);
    board.is_empty(cell) && (cell_type == C || cell_type == T)
}

// Return true if there are obstacles
pub fn obstacles(state: &State, m: &Move) -> bool {
    let mut p: Position = m.to;
    while (m.from.x == m.to.x && p.y != m.to.y) ||
        (m.from.y == m.to.y && p.x != m.to.x) {
        p = get_one_cell_closer(m.from, p).unwrap();
        if !is_legal_target_cell(state, p) {
            return true;
        }
    }
    return false;
}

// Checks if a cell is empty and regular
fn is_legal_target_cell(state: &State, cell: Position) -> bool {
    let cell_type: u32 = state.board.cell_type(cell);
    state.board.is_empty(cell) && (cell_type == R || cell_type == F)
}

// Check if is a legal move
pub fn legal_move(state: &State, m: &Move) -> bool {
    if m.from == m.to {
        return false;
    }

    // Check obstacles
    if obstacles(state, &m) {
        return false;
    }

    if is_legal_target_cell(state, m.to) {
        return true;
    }
    else if state.color == BLACK &&
        state.board.cell_type(m.from) == C &&
        state.board.cell_type(m.to) == C &&
        state.board.is_empty(m.to) &&
        (m.from.x as i32 - m.to.x as i32).abs() <= 2 {
        return true;
    }
    return true;
}

// Returns all legal moves
pub fn legal_moves(state: &State) -> Vec<Move> {
    let board = &state.board;
    let color = &state.color;
    let mut moves: Vec<Move> =  vec![];
    let mut cells: Vec<Position> = vec![];

    if color == WHITE {
        cells = board.white_cells();
        cells.push(board.king_cell().unwrap());
    }
    else if color == BLACK {
        cells = board.black_cells();
    }

    for cell in cells {
        let from: Position = cell;
        let mut to: Position = from;

        // Increment x
        loop {
            // Cell is on last column so no column is adjacent on the right
            if to.x == 8 {
                break
            }
            to.x += 1;
            if !is_legal_target_cell(state, to) {
                break
            }
            else if legal_move(&state, &Move { from, to }) {
                moves.push(Move { from, to });
            }
        }
        to = from;
        // Decrement x
        loop {
            // Cell is on first column so no column is adjacent on the left
            if to.x == 0 {
                break
            }
            to.x -= 1;
            if !is_legal_target_cell(state, to) {
                break
            }
            else if legal_move(&state, &Move { from, to }) {
                moves.push(Move { from, to });
            }
        }
        to = from;
        // Increment y
        loop {
            // Cell is on last row so no row is under
            if to.y == 8 {
                break
            }
            to.y += 1;
            if !is_legal_target_cell(state, to) {
                break
            }
            else if legal_move(&state, &Move { from, to }) {
                moves.push(Move { from, to });
            }
        }
        to = from;
        // Decrement y
        loop {
            // Cell is on first row so no row is above
            if to.y == 0 {
                break
            }
            to.y -= 1;
            if !is_legal_target_cell(state, to) {
                break
            }
            else if legal_move(&state, &Move { from, to }) {
                moves.push(Move { from, to });
            }
        }
    }
    return moves
}

// Returns which checkers has been captured by a move
pub fn captures(board: &Board, m: &Move) -> Vec<Position> {
    let mut captured_checkers: Vec<Position> = vec![];
    // Moved checker
    let moved_checker: Position = m.to;

    // Checks that two given cell content are opposite color. es W -> B
    let same_color = |a: Position, b: Position| -> bool {
        let ca = board.cell_color(a);
        let cb = board.cell_color(b);
        if ca.is_none() || cb.is_none() {
            return false;
        }
        else if ca.unwrap() == cb.unwrap() {
            return true;
        }
        return false;
    };

    // Check if a cell contains the king
    let is_king = |cell: Position| -> bool {
        board.cell_content(cell) == K
    };

    // Surrounding cells
    let surrounding_cells: [Option<Position>; 4] = board.surrounding_cells(moved_checker);

    let surrounded = |cell: Position| {
        let surrounding_cells: [Option<Position>; 4] = board.surrounding_cells(cell);
        if surrounding_cells.iter().any(|o| o.is_none()) {
            return false;
        }
        if surrounding_cells.iter().all(|p| is_barrier(board, p.unwrap()) ||
            board.cell_content(p.unwrap()) == B) {
            return true;
        }
        return false;
    };

    // Capture up
    if surrounding_cells[0].is_some() && !board.is_empty(surrounding_cells[0].unwrap()) &&
        !same_color(moved_checker, surrounding_cells[0].unwrap()) {
        let up_cell = surrounding_cells[0].unwrap();
        if !same_color(moved_checker, up_cell) && up_cell.y > 0 {
            let up_up_cell = Position { x: up_cell.x, y: up_cell.y-1 };

            // King capture
            if is_king(up_cell) {
                // Four side capture
                if surrounded(up_cell) {
                    //debug!("Four side king capture up");
                    captured_checkers.push(up_cell);
                }
                // Two side capture
                else if !board.is_king_in_throne() && same_color(up_up_cell, moved_checker) || board.cell_type(up_up_cell) == C {
                    //debug!("Two side king capture up");
                    captured_checkers.push(up_cell);
                }
            }
            // Regular checker capture
            else if same_color(up_up_cell, moved_checker) || is_barrier(board, up_up_cell) {
                captured_checkers.push(up_cell);
            }
        }
    }

    // Capture down
    if surrounding_cells[1].is_some() && !board.is_empty(surrounding_cells[1].unwrap()) &&
        !same_color(moved_checker, surrounding_cells[1].unwrap()) {
        let down_cell = surrounding_cells[1].unwrap();
        if !same_color(moved_checker, down_cell) && down_cell.y < 8 {
            let down_down_cell = Position { x: down_cell.x, y: down_cell.y+1 };

            // King capture
            if is_king(down_cell) {
                // Four side capture
                if surrounded(down_cell) {
                    //debug!("Four side king capture down");
                    captured_checkers.push(down_cell);
                }
                // Two side capture
                else if !board.is_king_in_throne() && same_color(down_down_cell, moved_checker) || board.cell_type(down_down_cell) == C {
                    //debug!("Two side king capture down");
                    captured_checkers.push(down_cell);
                }
            }
            // Regular checker capture
            else if same_color(down_down_cell, moved_checker) || is_barrier(board, down_down_cell) {
                captured_checkers.push(down_cell);
            }
        }
    }

    // Capture right
    if surrounding_cells[2].is_some() && !board.is_empty(surrounding_cells[2].unwrap()) &&
        !same_color(moved_checker, surrounding_cells[2].unwrap()) {
        let right_cell = surrounding_cells[2].unwrap();
        if !same_color(moved_checker, right_cell) && right_cell.x < 8 {
            let right_right_cell = Position { x: right_cell.x+1, y: right_cell.y };

            // King capture
            if is_king(right_cell) {
                // Four side capture
                if surrounded(right_cell) {
                    //debug!("Four side king capture right");
                    captured_checkers.push(right_cell);
                }
                // Two side capture
                else if !board.is_king_in_throne() && same_color(right_right_cell, moved_checker) || board.cell_type(right_right_cell) == C {
                    //debug!("Two side king capture right");
                    captured_checkers.push(right_cell);
                }
            }
            // Regular checker capture
            else if same_color(right_right_cell, moved_checker) || is_barrier(board,right_right_cell) {
                captured_checkers.push(right_cell);
            }
        }
    }

    // Capture left
    if surrounding_cells[3].is_some() && !board.is_empty(surrounding_cells[3].unwrap()) &&
        !same_color(moved_checker, surrounding_cells[3].unwrap()) {
        let left_cell = surrounding_cells[3].unwrap();
        if !same_color(moved_checker, left_cell) && left_cell.x > 0 {
            let left_left_cell = Position { x: left_cell.x-1, y: left_cell.y };

            // King capture
            if is_king(left_cell) {
                // Four side capture
                if surrounded(left_cell) {
                    //debug!("Four side king capture left");
                    captured_checkers.push(left_cell);
                }
                // Two side capture
                else if !board.is_king_in_throne() && same_color(left_left_cell, moved_checker) || board.cell_type(left_left_cell) == C {
                    //debug!("Two side king capture left");
                    captured_checkers.push(left_cell);
                }
            }
            // Regular checker capture
            else if same_color(left_left_cell, moved_checker) || is_barrier(board, left_left_cell) {
                captured_checkers.push(left_cell);
            }
        }
    }

    captured_checkers
}

// Returns the status of the game
pub fn game_status(state: &State) -> Status {
    let board = &state.board;
    let history= &state.history;
    let color = &state.color;

    let king_cell: Option<Position> = board.king_cell();

    // King not present on board
    if king_cell.is_none() && color == WHITE {
        //debug!("King not present on the board: LOSS");
        return Status::LOSS;
    }
    if king_cell.is_none() && color == BLACK {
        //debug!("King not present on the board: WIN");
        return Status::WIN;
    }

    // King on escape cell
    if board.cell_type(king_cell.unwrap()) == F && color == WHITE {
        //debug!("King escaped: WIN");
        return Status::WIN;
    }
    if board.cell_type(king_cell.unwrap()) == F && color == BLACK {
        //debug!("King escaped: LOSS");
        return Status::LOSS;
    }

    // Check if king is captured
    if history.len() >= 2 {
        // Current king position
        let ck: Position = king_cell.unwrap();
        // Previous king position
        let pk: Position = history.split_at(history.len()-2).1.first().unwrap().king_cell().unwrap();
        // Check if king is in throne or regular cell
        let king_type: u32 = board.cell_type(ck);

        let up_cell = Position { x: ck.x, y: ck.y - 1 };
        let down_cell = Position { x: ck.x, y: ck.y + 1 };
        let right_cell = Position { x: ck.x + 1, y: ck.y };
        let left_cell = Position { x: ck.x - 1, y: ck.y };

        let previous_up_cell = Position { x: pk.x, y: pk.y - 1 };
        let previous_down_cell = Position { x: pk.x, y: pk.y + 1 };
        let previous_right_cell = Position { x: pk.x + 1, y: pk.y };
        let previous_left_cell = Position { x: pk.x - 1, y: pk.y };

        // King is in same position and at least one cell around changed
        if ck == pk && !(up_cell != previous_up_cell ||
           down_cell != previous_down_cell ||
           right_cell != previous_right_cell ||
           left_cell != previous_left_cell) {

            let up_content: u32 = board.cell_content(up_cell);
            let down_content: u32 = board.cell_content(down_cell);
            let right_content: u32 = board.cell_content(right_cell);
            let left_content: u32 = board.cell_content(left_cell);

            let up_type: u32 = board.cell_type(up_cell);
            let down_type: u32 = board.cell_type(down_cell);
            let right_type: u32 = board.cell_type(right_cell);
            let left_type: u32 = board.cell_type(left_cell);

            // King surrounded adjacent to throne
            if king_type == R &&
               (up_type == T || up_content == B) &&
               (down_type == T || down_content == B) &&
               (right_type == T || right_content == B) &&
               (left_type == T || left_content == B) {
                //debug!("King surrounded adjacent to throne");
                if color == WHITE {
                    return Status::LOSS;
                } else {
                    return Status::WIN;
                }
            }

            // King capture on 4 sides
            if up_content == B &&
               down_content == B &&
               right_content == B &&
               left_content == B {
                //debug!("King capture on 4 sides");
                if color == WHITE {
                    return Status::LOSS;
                } else {
                    return Status::WIN;
                }
            }

            // King regular capture on 2 sides with king not adjacent to throne
            if (up_type != T && down_type != T && right_type != T && left_type != T) &&
               (((up_content == B || up_type == C) && (down_content == B || down_type == C)) ||
               ((right_content == B || right_type == C) && (left_content == B || left_type == C))) {
                //debug!("King regular capture on 2 sides");
                if color == WHITE {
                    return Status::LOSS;
                } else {
                    return Status::WIN;
                }
            }
        }

    }

    // TODO optimize
    // No moves possible
    if legal_moves(state).len() == 0 {
        //debug!("No legal moves: LOSS");
        return Status::LOSS;
    }

    if history.len() > 3 && history.split_at(history.len()-2).0.contains(board) {
        //debug!("Same state twice: DRAW");
        return Status::DRAW;
    }

    Status::ONGOING
}

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::game::{Move, Position, Status, State, Board};
    use crate::rules::{legal_moves, captures, game_status};

    #[test]
    fn test_captures() {
        // No capture
        let board = Board::new([
            [0, 0, 0, 2, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [2, 2, 1, 1, 3, 1, 1, 2, 2],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 5,
                y: 2
            },
            to: Position {
                x: 4,
                y: 2
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![], "Incorrect no capture");

        let board = Board::new([
            [0, 0, 0, 2, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0],
            [2, 0, 0, 0, 1, 0, 2, 0, 0],
            [2, 2, 1, 1, 3, 1, 0, 2, 2],
            [2, 0, 0, 0, 1, 0, 1, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 4,
                y: 2
            },
            to: Position {
                x: 6,
                y: 2
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![], "Incorrect no capture");

        let board = Board::new([
            [0, 0, 2, 0, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 1, 0, 0, 0],
            [2, 0, 1, 0, 0, 0, 0, 0, 2],
            [2, 2, 1, 1, 3, 0, 1, 2, 2],
            [2, 0, 0, 0, 1, 1, 0, 0, 2],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 1,
                y: 0
            },
            to: Position {
                x: 2,
                y: 0
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![], "Incorrect no capture");

        // Single capture
        let board = Board::new([
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 1, 2, 0, 0, 0],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [2, 2, 1, 1, 3, 1, 1, 2, 2],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 5,
                y: 0
            },
            to: Position {
                x: 5,
                y: 2
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![Position{x: 4, y: 2}], "Incorrect single capture");

        // Double capture
        let board = Board::new([
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 1, 2, 1, 2, 0],
            [2, 0, 0, 0, 1, 0, 0, 0, 0],
            [2, 2, 1, 1, 3, 1, 0, 2, 2],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 5,
                y: 0
            },
            to: Position {
                x: 5,
                y: 2
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, [Position{x: 6, y: 2}, Position {x: 4, y: 2}]);

        // King no capture in throne
        let board = Board::new([
            [0, 0, 0, 0, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 2, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 1, 1, 2],
            [2, 2, 1, 2, 3, 2, 0, 2, 2],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 5,
                y: 8
            },
            to: Position {
                x: 5,
                y: 4
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![]);

        let board = Board::new([
            [0, 0, 0, 2, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [2, 2, 1, 1, 3, 0, 1, 2, 2],
            [2, 0, 0, 0, 1, 1, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        // f1->f5
        let m = Move {
            from: Position {
                x: 6,
                y: 0
            },
            to: Position {
                x: 6,
                y: 4
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![]);

        // King capture in throne
        let board = Board::new([
            [0, 0, 0, 0, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 2, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 1, 1, 2],
            [2, 2, 1, 2, 3, 2, 0, 2, 2],
            [0, 0, 0, 0, 2, 0, 0, 1, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 5,
                y: 8
            },
            to: Position {
                x: 5,
                y: 4
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![Position {x: 4, y: 4}]);

        // King no capture next to throne
        let board = Board::new([
            [0, 0, 1, 2, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 2, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 1, 2],
            [2, 2, 2, 3, 0, 2, 0, 2, 2],
            [0, 0, 0, 2, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 2,
                y: 3
            },
            to: Position {
                x: 2,
                y: 4
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![]);

        // King capture next to throne
        let board = Board::new([
            [0, 0, 1, 0, 2, 2, 0, 0, 0],
            [0, 0, 0, 0, 2, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 2, 1, 0, 0, 1, 2],
            [2, 2, 2, 3, 0, 2, 0, 2, 2],
            [0, 0, 0, 2, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 2,
                y: 3
            },
            to: Position {
                x: 2,
                y: 4
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![Position {x: 3, y: 4}]);


        // King capture not in throne
        let board = Board::new(	[
            [0, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 2, 0, 3, 0, 0, 0, 0],
            [0, 0, 1, 0, 2, 0, 2, 0, 0],
            [2, 2, 0, 0, 0, 1, 1, 2, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 2, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 3,
                y: 4
            },
            to: Position {
                x: 4,
                y: 3
            }
        };
        let c = captures(&board, &m);
        assert_eq!(c, vec![Position {x: 4, y: 2}]);
    }

    #[test]
    fn test_legal_moves() {
        let white_state = State::init(WHITE.to_string());
        // let black_state = State::init(BLACK.to_string());

        // Initial moves
        let moves = legal_moves(&white_state);
        assert_eq!(moves.len(), 56);
    }

    #[test]
    fn test_game_status() {
        let mut state = State::init(WHITE.to_string());
        let board = Board::new(	[
            [0, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 2, 0, 3, 0, 0, 0, 0],
            [0, 0, 1, 2, 0, 0, 2, 0, 0],
            [2, 2, 0, 0, 0, 1, 1, 2, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 2, 0, 0, 0]
        ]);
        state.history.push(board);
        state.board = board;
        let m = Move {
            from: Position {
                x: 3,
                y: 3
            },
            to: Position {
                x: 4,
                y: 3
            }
        };
        state.apply_move(&m);
        let status = game_status(&state);
        assert_eq!(status, Status::LOSS);
    }
}