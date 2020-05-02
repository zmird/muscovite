use crate::game::{Move, Position, Status, State};
use crate::constants::*;

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
            if board.is_empty(to) && board.cell_type(to) == R {
                moves.push(Move { from, to });
            }
            else if color == BLACK &&
                board.cell_type(from) == C &&
                board.cell_type(to) == C &&
                board.is_empty(to) &&
                to.x <= from.x+2 {
                moves.push(Move { from, to });
            }
            else if !board.is_empty(to) || board.cell_type(to) != R {
                break;
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
            if board.is_empty(to) && board.cell_type(to) == R {
                moves.push(Move { from, to });
            }
            else if color == BLACK &&
                board.cell_type(from) == C &&
                board.cell_type(to) == C &&
                board.is_empty(to) &&
                to.x+2 >= from.x {
                moves.push(Move { from, to });
            }
            else if !board.is_empty(to) || board.cell_type(to) != R {
                break;
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
            if board.is_empty(to) && board.cell_type(to) == R {
                moves.push(Move { from, to });
            }
            else if color == BLACK &&
                board.cell_type(from) == C &&
                board.cell_type(to) == C &&
                board.is_empty(to) &&
                to.y <= from.y+2 {
                moves.push(Move { from, to });
            }
            else if !board.is_empty(to) || board.cell_type(to) != R {
                break;
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
            if board.is_empty(to) && board.cell_type(to) == R {
                moves.push(Move { from, to });
            }
            else if color == BLACK &&
                board.cell_type(from) == C &&
                board.cell_type(to) == C &&
                board.is_empty(to) &&
                to.y+2 >= from.y {
                moves.push(Move { from, to });
            }
            else if !board.is_empty(to) || board.cell_type(to) != R {
                break;
            }
        }
    }
    return moves
}

pub fn game_status(state: &State) -> Status {
    let board = &state.board;
    let history= &state.history;
    let color = &state.color;

    let king_cell: Option<Position> = board.king_cell();

    // King not present on board
    if king_cell.is_none() && color == WHITE {
        return Status::LOSS;
    }
    if king_cell.is_none() && color == BLACK {
        return Status::WIN;
    }

    // King on escape cell
    if board.cell_type(king_cell.unwrap()) == F && color == WHITE {
        return Status::WIN;
    }
    if board.cell_type(king_cell.unwrap()) == F && color == BLACK {
        return Status::LOSS;
    }

    // Check if king is captured
    if history.len() > 3 {
        // Current king position
        let ck: Position = king_cell.unwrap();
        // Previous king position
        let pk: Position = history.split_at(history.len()-3).1.first().unwrap().king_cell().unwrap();
        // Check if king is in throne or regular cell
        let king_type: u32 = board.cell_type(ck);

        let up_cell = Position { x: ck.x, y: ck.y + 1 };
        let down_cell = Position { x: ck.x, y: ck.y - 1 };
        let right_cell = Position { x: ck.x + 1, y: ck.y };
        let left_cell = Position { x: ck.x - 1, y: ck.y };

        let previous_up_cell = Position { x: pk.x, y: pk.y + 1 };
        let previous_down_cell = Position { x: pk.x, y: pk.y - 1 };
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

            // King surrounded adiacent to throne
            if king_type == R &&
               (up_type == T || up_content == B) &&
               (down_type == T || down_content == B) &&
               (right_type == T || right_content == B) &&
               (left_type == T || left_content == B) {
                println!("King surrounded adjacent to throne");
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
                println!("King capture on 4 sides");
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
                println!("King regular capture on 2 sides");
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
        return Status::LOSS;
    }

    if history.len() > 3 && history.split_at(history.len()-2).0.contains(board) {
        return Status::DRAW;
    }

    Status::ONGOING
}
