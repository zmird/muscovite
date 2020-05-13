use crate::constants::*;
use crate::game::{Move, State, Status, Position};
use crate::rules::{legal_moves, game_status, obstacles, is_barrier, get_opposite_color};
use std::cmp::{max, min};
use log::debug;
use rand::Rng;

fn actions(state: &State) -> Vec<Move> {
    legal_moves(state)
}

fn result(state: &State, m: &Move) -> State {
    let mut new_state: State = state.clone();
    new_state.apply_move(m);
    new_state
}

fn terminal_test(state: &State) -> bool {
    let status: Status = game_status(state);
    if status != Status::ONGOING {
        true
    } else {
        false
    }
}

pub fn heuristic(state: &State) -> i32 {
    let board = state.board;
    let previous_board = if state.history.len() >= 2 {
        state.history.get(state.history.len() - 2)
    } else {
        None
    };

    let status = game_status(state);

    if status == Status::WIN {
        debug!("WIN move");
        return std::i32::MAX;
    }
    if status == Status::LOSS {
        debug!("LOSS move");
        return std::i32::MIN;
    }
    if status == Status::DRAW {
        return 0;
    }

    let king: Position = board.king_cell().unwrap();
    let previous_king: Position = if previous_board.is_some() {
        previous_board.unwrap().king_cell().unwrap()
    } else {
        king
    };
    let king_surrounding_cells: [Option<Position>; 4] = board.surrounding_cells(king);

    // Checker variation
    let current_checker_difference = board.white_cells().len() as i32 - board.black_cells().len() as i32 + 8;
    let previous_checker_difference = if previous_board.is_some() {
        previous_board.unwrap().white_cells().len() as i32 - previous_board.unwrap().black_cells().len() as i32 + 8
    } else {
        current_checker_difference
    };

    // King position
    let king_in_throne: bool = board.cell_type(king) == T;
    let king_next_throne: bool = king_surrounding_cells.iter()
        .any(|cell| cell.is_some() && board.cell_type(cell.unwrap()) == T);

    // King escapes
    let mut king_escapes: i32 = 0;
    let possible_king_escapes: [Position; 4] = [
        Position { x: king.x, y: 0 },
        Position { x: king.x, y: 8 },
        Position { x: 0, y: king.y },
        Position { x: 8, y: king.y }
    ];
    for cell in possible_king_escapes.iter() {
        if board.cell_type(*cell) == F && !obstacles(state, &Move{ from: king, to: *cell }) {
            king_escapes += 1;
        }
    }

    // White checkers in respect to king
    let white_checkers_around_king = king_surrounding_cells.iter()
        .fold(0, |acc, cell| if cell.is_some() && board.cell_content(cell.unwrap()) == W { acc + 1} else { acc });
    let previous_white_checkers_aroung_king: i32 = if previous_board.is_some() {
        previous_board.unwrap().surrounding_cells(previous_king).iter()
            .fold(0, |acc, cell| if cell.is_some() && previous_board.unwrap().cell_content(cell.unwrap()) == W { acc + 1} else { acc })
    } else {
        white_checkers_around_king
    };

    // Black checkers in respect to king
    let black_checkers_around_king = king_surrounding_cells.iter()
        .fold(0, |acc, cell| if cell.is_some() && board.cell_content(cell.unwrap()) == B { acc + 1} else { acc });
    let previous_black_checkers_aroung_king: i32 = if previous_board.is_some() {
        previous_board.unwrap().surrounding_cells(previous_king).iter()
            .fold(0, |acc, cell| if cell.is_some() && previous_board.unwrap().cell_content(cell.unwrap()) == B { acc + 1} else { acc })
    } else {
        black_checkers_around_king
    };

    let mut black_checkers_around_king_in_one_move: Vec<Position> = vec![];
    for surrounding_cell in king_surrounding_cells.iter() {
        if surrounding_cell.is_none() ||
            !state.board.is_empty(surrounding_cell.unwrap()) ||
            is_barrier(&state.board, surrounding_cell.unwrap()) {
            continue;
        }

        let mut up: Option<Position> = *surrounding_cell;
        loop {
            up = state.board.upper_cell(up.unwrap());
            if up.is_some() {
                if state.board.is_empty(up.unwrap()) {
                    continue;
                }
                if state.board.cell_color(up.unwrap()).unwrap() == WHITE {
                    break;
                } else {
                    if !black_checkers_around_king_in_one_move.contains(&up.unwrap()) {
                        black_checkers_around_king_in_one_move.push(up.unwrap());
                    }
                    break
                }
            } else {
                break;
            }
        }
        let mut down: Option<Position> = *surrounding_cell;
        loop {
            down = state.board.lower_cell(down.unwrap());
            if down.is_some() {
                if state.board.is_empty(down.unwrap()) {
                    continue;
                }
                if state.board.cell_color(down.unwrap()).unwrap() == WHITE {
                    break;
                } else {
                    if !black_checkers_around_king_in_one_move.contains(&down.unwrap()) {
                        black_checkers_around_king_in_one_move.push(down.unwrap());
                    }
                    break
                }
            } else {
                break;
            }
        }
        let mut right: Option<Position> = *surrounding_cell;
        loop {
            right = state.board.right_cell(right.unwrap());
            if right.is_some() {
                if state.board.is_empty(right.unwrap()) {
                    continue;
                }
                if state.board.cell_color(right.unwrap()).unwrap() == WHITE {
                    break;
                } else {
                    if !black_checkers_around_king_in_one_move.contains(&right.unwrap()) {
                        black_checkers_around_king_in_one_move.push(right.unwrap());
                    }
                    break
                }
            } else {
                break;
            }
        }
        let mut left: Option<Position> = *surrounding_cell;
        loop {
            left = state.board.left_cell(left.unwrap());
            if left.is_some() {
                if state.board.is_empty(left.unwrap()) {
                    continue;
                }
                if state.board.cell_color(left.unwrap()).unwrap() == WHITE {
                    break;
                } else {
                    if !black_checkers_around_king_in_one_move.contains(&left.unwrap()) {
                        black_checkers_around_king_in_one_move.push(left.unwrap());
                    }
                    break
                }
            } else {
                break;
            }
        }
    }

    // Barriers in respect to king
    let barriers_around_king: u32 = king_surrounding_cells.iter()
        .fold(0, |acc, cell| if cell.is_some() && is_barrier(&state.board, cell.unwrap()){ acc } else { acc });

    let position_weights: [[i32; 9]; 9] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 5, 2, 0, 2, 5, 2, 0],
        [0, 5, 5, 5, 5, 5, 5, 5, 0],
        [0, 2, 5, 0, 0, 0, 5, 2, 0],
        [0, 0, 5, 0, -10, 0, 5, 0, 0],
        [0, 2, 5, 0, 0, 0, 5, 2, 0],
        [0, 5, 5, 5, 5, 5, 5, 5, 0],
        [0, 2, 5, 2, 0, 2, 5, 2, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0]
    ];


    debug!("Heuristic:\n{}\n{:?}\n\tKing in throne: {}\n\tKing next to throne: {}\n\tEscapes: {}\n\tWhite checkers around king: {}\n\tPrevious White checkers around king: {}\n\tBarriers around king: {}\n\tBlack checkers around king: {}\n\tBlack checkers around king in one move: {}", board, board, king_in_throne, king_next_throne, king_escapes, white_checkers_around_king, previous_white_checkers_aroung_king, barriers_around_king, black_checkers_around_king, black_checkers_around_king_in_one_move.len());

    // WINNING AND LOSING CONDITIONS
    if king_escapes >= 2 && barriers_around_king == 0 && black_checkers_around_king == 0 && black_checkers_around_king_in_one_move.len() == 0 {
        debug!("Choose 1");
        return std::i32::MAX;
    }
    if !king_in_throne && barriers_around_king > 0 && black_checkers_around_king_in_one_move.len() > 0 {
        debug!("Choose: 2");
        return std::i32::MIN;
    }
    if !king_in_throne && barriers_around_king == 0 && black_checkers_around_king >= 1 && black_checkers_around_king_in_one_move.len() >= 1 {
        debug!("Choose: 3");
        return std::i32::MIN;
    }
    if king_next_throne && black_checkers_around_king >= 2 && black_checkers_around_king_in_one_move.len() >= 1 {
        debug!("Choose: 4");
        return std::i32::MIN;
    }
    if king_in_throne && black_checkers_around_king >= 3 && black_checkers_around_king_in_one_move.len() >= 1 {
        debug!("Choose: 5");
        return std::i32::MIN;
    }

    // Value calculation
    let mut value: i32 = 0;

    // Checkers variation
    value += (current_checker_difference - previous_checker_difference) * 5;

    // King escapes
    value += king_escapes * 10;

    // Barriers around king
    value -= barriers_around_king as i32 * 5;

    // Black checkers around king
    value -= black_checkers_around_king * 5;

    // Black checkers around king in one move
    // value -= black_checkers_around_king_in_one_move.len() as usize;

    // King captures risks
    if king_escapes == 1 && barriers_around_king == 0 && black_checkers_around_king == 0 && black_checkers_around_king_in_one_move.len() == 0 {
        debug!("Applied: 1");
        value += 15;
    }
    // if !king_in_throne && barriers_around_king == 0 && black_checkers_around_king == 1 && black_checkers_around_king_in_one_move.len() == 0 {
    //     debug!("Applied: 2");
    //     value += -5;
    // }
    // if !king_in_throne && barriers_around_king >= 1 && black_checkers_around_king == 0 && black_checkers_around_king_in_one_move.len() == 0 {
    //     debug!("Applied: 3");
    //     value += -5;
    // }
    if king_next_throne && black_checkers_around_king == 1 && black_checkers_around_king_in_one_move.len() >= 2 {
        debug!("Applied: 4");
        value += -10;
    }
    // if king_next_throne && black_checkers_around_king == 0 && black_checkers_around_king_in_one_move.len() >= 3 {
    //     debug!("Applied: 5");
    //     value += -5;
    // }
    if king_in_throne && black_checkers_around_king == 2 && black_checkers_around_king_in_one_move.len() == 0 {
        debug!("Applied: 6");
        value += -10;
    }
    if king_in_throne && black_checkers_around_king == 2 && black_checkers_around_king_in_one_move.len() >= 1 {
        debug!("Applied: 7");
        value += -15;
    }
    if king_in_throne && black_checkers_around_king == 3 && black_checkers_around_king_in_one_move.len() == 0 {
        debug!("Applied: 8");
        value += -15;
    }

    // White checkers in respect to king
    if previous_white_checkers_aroung_king >= 3 && white_checkers_around_king < previous_white_checkers_aroung_king {
        debug!("Applied: 9");
        value += 6;
    }
    value += position_weights[king.y as usize][king.x as usize];

    // debug!("Score: {}", value);
    value
}

pub fn alpha_beta_search(state: &State, depth: u32) -> Option<Move> {

    fn max_value(state: &State, alpha: i32, beta: i32, depth: u32) -> i32 {
        let mut a = alpha;
        let b = beta;
        if depth == 0 || terminal_test(state) {
            return heuristic(state);
        }
        let mut value = std::i32::MIN;
        for action in actions(state) {
            debug!("{}Evaluating {}", "-> ".repeat(depth as usize), action);
            value = max(value, min_value(&result(state, &action), a, b, depth - 1));
            if value >= b {
                return value;
            }
            a = max(a, value);
        }
        return value;
    };

    fn min_value(state: &State, alpha: i32, beta: i32, depth: u32) -> i32 {
        let a = alpha;
        let mut b = beta;
        if depth == 0 || terminal_test(state) {
            return heuristic(state);
        }
        let mut value = std::i32::MAX;
        for action in actions(state) {
            debug!("{}Evaluating {}", "-> ".repeat(depth as usize), action);
            value = min(value, max_value(&result(state, &action), a, b, depth - 1));
            if value <= alpha {
                return value;
            }
            b = min(b, value);
        }
        return value;
    };

    let mut best_action = None;
    let mut alpha = std::i32::MIN;
    let mut beta = std::i32::MAX;
    for action in actions(state) {
        debug!("{}Evaluating {}", "-> ".repeat(depth as usize), action);
        if state.color == WHITE {
            let value = min_value(&result(state, &action), alpha, beta, depth);
            debug!("WHITE Move {} value is {}", action, value);
            if value > alpha || best_action.is_none() {
                debug!("Found new best move: {} with value {}", action, value);
                debug!("Alpha: {}, Beta: {}", alpha, beta);
                alpha = value;
                best_action = Some(action);
            }
        } else {
            let value = max_value(&result(state, &action), alpha, beta, depth);
            debug!("BLACK Move {} value is {}", action, value);
            if value < beta || best_action.is_none() {
                debug!("Found new best move: {} with value {}", action, value);
                debug!("Alpha: {}, Beta: {}", alpha, beta);
                beta = value;
                best_action = Some(action);
            }
        }
    }
    return best_action;
}

#[allow(dead_code)]
pub fn random(state: &State) -> Move {
    let actions = actions(state);
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range::<usize, usize, usize>(0, actions.len()-1);
    actions.get(i).cloned().unwrap()
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_heuristic() {
        let mut state = State::init(WHITE.to_string());
        let mut score: i32 = heuristic(&state);
        assert_eq!(score, -10);

        // e4->f4
        let mut m = Move {
            from: Position {
                x: 4,
                y: 3
            },
            to: Position {
                x: 5,
                y: 3
            }
        };
        state.apply_move(&m);
        score = heuristic(&state);
        assert_eq!(score, -4);
    }
}
