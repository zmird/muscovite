use crate::constants::*;
use crate::game::{Move, State, Status, Position};
use crate::rules::{legal_moves, game_status, obstacles};
use std::cmp::{max, min};
use rand::Rng;

fn actions(state: &State) -> Vec<Move> {
    legal_moves(state)
}

fn result(state: &State, m: &Move) -> State {
    let mut new_state: State = state.clone();
    new_state.board.apply_move(m);
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
    let mut value: i32 = 0;
    let status = game_status(state);

    // Max and min value for win and loss
    if status == Status::WIN {
        return std::i32::MAX;
    }
    if status == Status::LOSS {
        return std::i32::MIN;
    }
    if status == Status::DRAW {
        return 0;
    }

    let escapes_near_king = |king: Position| -> i32 {
        let mut near_escapes: i32 = 0;
        let possible_escapes: [Position; 4] = [
            Position { x: king.x, y: 0 },
            Position { x: king.x, y: 8 },
            Position { x: 0, y: king.y },
            Position { x: 8, y: king.y }
        ];
        for cell in possible_escapes.iter() {
            if state.board.cell_type(*cell) == F && !obstacles(state, &Move{ from: king, to: *cell }) {
                near_escapes += 1;
            }
        }
        near_escapes
    };

    if state.color == WHITE {
        value += escapes_near_king(state.board.king_cell().unwrap()) * 500;
    } else {
        value -= escapes_near_king(state.board.king_cell().unwrap()) * 500;
    }

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
    let beta = std::i32::MAX;
    for action in actions(state) {
        let value = min_value(&result(state, &action), alpha, beta, depth);
        if value > alpha {
            alpha = value;
            best_action = Some(action);
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

