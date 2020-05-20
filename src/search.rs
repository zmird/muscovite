use crate::constants::*;
use crate::game::{Move, State, Status, Position};
use crate::rules::{legal_moves, game_status, obstacles, is_barrier, get_opposite_color, is_legal_target_cell};
use std::cmp::{max, min};
use std::time::Instant;
use rand::Rng;
use log::info;

fn actions(state: &State) -> Vec<Move> {
    legal_moves(state)
}

fn result(state: &State, m: &Move) -> State {
    let mut new_state: State = state.clone();
    new_state.apply_move(m);
    new_state.color = get_opposite_color(&new_state.color);
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
        return if state.color == WHITE { std::i32::MAX } else { std::i32::MIN };
    }
    if status == Status::LOSS {
        return if state.color == WHITE { std::i32::MIN } else { std::i32::MAX };
    }
    if status == Status::DRAW {
        return 0;
    }

    // Checker variation
    let current_checker_difference = board.white_cells().len() as i32 - board.black_cells().len() as i32 + 7;

    let king: Position = board.king_cell().unwrap();
    let previous_king: Position = if previous_board.is_some() {
        previous_board.unwrap().king_cell().unwrap()
    } else {
        king
    };
    let king_moved: bool = king != previous_king;
    let king_surrounding_cells: [Option<Position>; 4] = board.surrounding_cells(king);
    let previous_king_surrounding_cells: [Option<Position>; 4] = previous_board.unwrap().surrounding_cells(king);
    let king_surrounding_cells_diagonal: [Option<Position>; 4] = board.surrounding_diagonal_cells(king);

    // King position
    let king_in_throne: bool = board.is_king_in_throne();
    let king_next_throne: bool = board.is_king_next_throne();

    fn get_king_escapes(state: &State, king: Position) -> u32 {
        let mut king_escapes: u32 = 0;
        let possible_king_escapes: [Position; 4] = [
            Position { x: king.x, y: 0 },
            Position { x: king.x, y: 8 },
            Position { x: 0, y: king.y },
            Position { x: 8, y: king.y }
        ];
        for cell in possible_king_escapes.iter() {
            if state.board.cell_type(*cell) == F && !obstacles(state, &Move{ from: king, to: *cell }) {
                king_escapes += 1;
            }
        }
        king_escapes
    }

    // King escapes
    let king_escapes: i32 = get_king_escapes(state, king) as i32;

    // King escapes in one move
    let mut king_escapes_in_one_move: u32 = 0;
    let mut king_up = board.upper_cell(king);
    while king_up.is_some() && is_legal_target_cell(state, king_up.unwrap()) {
        let escapes = get_king_escapes(state, king_up.unwrap());
        if escapes > king_escapes_in_one_move {
            king_escapes_in_one_move = escapes;
        }
        king_up = board.upper_cell(king_up.unwrap());
    }
    let mut king_down = board.lower_cell(king);
    while king_down.is_some() && is_legal_target_cell(state, king_down.unwrap()) {
        let escapes = get_king_escapes(state, king_down.unwrap());
        if escapes > king_escapes_in_one_move {
            king_escapes_in_one_move = escapes;
        }
        king_down = board.lower_cell(king_down.unwrap());
    }
    let mut king_right = board.right_cell(king);
    while king_right.is_some() && is_legal_target_cell(state, king_right.unwrap()) {
        let escapes = get_king_escapes(state, king_right.unwrap());
        if escapes > king_escapes_in_one_move {
            king_escapes_in_one_move = escapes;
        }
        king_right = board.right_cell(king_right.unwrap());
    }
    let mut king_left = board.left_cell(king);
    while king_left.is_some() && is_legal_target_cell(state, king_left.unwrap()) {
        let escapes = get_king_escapes(state, king_left.unwrap());
        if escapes > king_escapes_in_one_move {
            king_escapes_in_one_move = escapes;
        }
        king_left = board.left_cell(king_left.unwrap());
    }

    // Black checkers in respect to king
    let black_checkers_around_king = king_surrounding_cells.iter()
        .fold(0, |acc, cell| if cell.is_some() && board.cell_content(cell.unwrap()) == B { acc + 1} else { acc });
    let previous_black_checkers_around_king = if previous_board.is_some() {
        previous_king_surrounding_cells.iter()
            .fold(0, |acc, cell| if cell.is_some() && previous_board.unwrap().cell_content(cell.unwrap()) == B { acc + 1} else { acc })
    } else {
        black_checkers_around_king
    };
    let black_checkers_around_king_changed: bool = black_checkers_around_king != previous_black_checkers_around_king;
    let black_checkers_around_king_diagonal = king_surrounding_cells_diagonal.iter()
        .fold(0, |acc, cell| if cell.is_some() && board.cell_content(cell.unwrap()) == B { acc + 1} else { acc });

    let mut black_checkers_around_king_in_one_move: Vec<Position> = vec![];
    let mut up: Option<Position> = king_surrounding_cells[0];
    let mut down: Option<Position> = king_surrounding_cells[1];
    let mut right: Option<Position> = king_surrounding_cells[2];
    let mut left: Option<Position> = king_surrounding_cells[3];
    if up.is_some() && down.is_some() && board.cell_content(down.unwrap()) == B &&
        (state.board.is_empty(up.unwrap()) || is_barrier(&state.board, up.unwrap())) {
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
    }
    if down.is_some() && up.is_some() && board.cell_content(up.unwrap()) == B &&
        (state.board.is_empty(down.unwrap()) || is_barrier(&state.board, down.unwrap())) {
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
    }
    if right.is_some() && left.is_some() && board.cell_content(left.unwrap()) == B &&
        (state.board.is_empty(right.unwrap()) || is_barrier(&state.board, right.unwrap())) {
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
    }
    if left.is_some() && right.is_some() && board.cell_content(right.unwrap()) == B &&
        (state.board.is_empty(left.unwrap()) || is_barrier(&state.board, left.unwrap())) {
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
    let barriers_around_king: u32 = king_surrounding_cells.iter().fold(0, |acc, cell| if cell.is_some() && is_barrier(&state.board, cell.unwrap()){ acc } else { acc });

    let position_weights: [[i32; 9]; 9] = [
        [0,  0,  0,  0,  0,  0,  0,  0,  0],
        [0,  5, 10,  0,  0,  0, 10,  5,  0],
        [0,  5, 10,  5,  5,  5, 10,  5,  0],
        [0,  0,  0,  0,  0,  0,  0,  0,  0],
        [0,  0,  5,  0, -10,  0,  5,  0,  0],
        [0,  0,  0,  0,  0,  0,  0,  0,  0],
        [0,  5, 10,  5,  5,  5, 10,  5,  0],
        [0,  5, 10,  0,  0,  0, 10,  5,  0],
        [0,  0,  0,  0,  0,  0,  0,  0,  0],
    ];

    // WINNING IN ONE MOVE
    if king_escapes >= 2 && (barriers_around_king == 0 || black_checkers_around_king <= 1) {
        return 10000;
    }

    // WINNING IN TWO MOVES
    if king_escapes_in_one_move >= 2 && king_in_throne && black_checkers_around_king <= 3 && black_checkers_around_king_in_one_move.len() == 0 {
        return 5000;
    }
    if king_escapes_in_one_move >= 2 && king_in_throne && black_checkers_around_king <= 2 {
        return 5000;
    }
    if king_escapes_in_one_move >= 2 && king_next_throne && black_checkers_around_king <= 2 && black_checkers_around_king_in_one_move.len() == 0 {
        return 5000;
    }
    if king_escapes_in_one_move >= 2 && king_next_throne && black_checkers_around_king <= 1 {
        return 5000;
    }
    if king_escapes_in_one_move >= 2 && !king_in_throne && !king_next_throne && black_checkers_around_king <= 1 && black_checkers_around_king_in_one_move.len() == 0 {
        return 5000;
    }
    if king_escapes_in_one_move >= 2 && !king_in_throne && !king_next_throne && black_checkers_around_king == 0 {
        return 5000;
    }

    // LOSING IN ONE MOVE
    if king_moved && !king_in_throne && !king_next_throne && barriers_around_king > 0 && black_checkers_around_king_in_one_move.len() > 0 {
        return -10000;
    }
    if king_moved && !king_in_throne && !king_next_throne && !king_moved && barriers_around_king == 0 && black_checkers_around_king_changed && black_checkers_around_king >= 1 && black_checkers_around_king_in_one_move.len() >= 1 {
        return -10000;
    }
    if king_moved && king_next_throne && !king_moved && black_checkers_around_king_changed && black_checkers_around_king >= 2 && black_checkers_around_king_in_one_move.len() >= 1 {
        return -10000;
    }
    if king_moved && king_in_throne && !king_moved && black_checkers_around_king_changed && black_checkers_around_king >= 3 && black_checkers_around_king_in_one_move.len() >= 1 {
        return -10000;
    }

    // Value calculation
    let mut value: i32 = 0;

    // Checkers variation
    value += current_checker_difference * 25;

    // King escapes
    value += king_escapes * 70;

    // King escapes in one move
    value += king_escapes_in_one_move as i32 * 35;

    // Barriers around king
    value -= barriers_around_king as i32 * 5;

    // Black checkers around king
    value -= black_checkers_around_king * 10;

    // Black checker around king diagonal
    value -= black_checkers_around_king_diagonal * 10;

    // Position weights
    value += position_weights[king.y as usize][king.x as usize];

    value
}

#[allow(dead_code)]
pub fn random_heuristic(_state: &State) -> i32 {
    rand::random::<i32>()
}

#[allow(dead_code)]
pub fn alpha_beta_search(state: &State, depth: u32) -> (Option<Move>, i32) {

    fn max_value(state: &State, alpha: i32, beta: i32, depth: u32) -> i32 {
        let mut a = alpha;
        let b = beta;
        if depth == 0 || terminal_test(state) {
            return heuristic(state);
        }
        let mut value = std::i32::MIN;
        for action in actions(state) {
            value = max(value, min_value(&result(state, &action), a, b, depth - 1));
            if value > b {
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
            if value < alpha {
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
        if state.color == WHITE {
            let value = min_value(&result(state, &action), alpha, beta, depth);
            if value > alpha || best_action.is_none() {
                alpha = value;
                best_action = Some(action);
            }
        } else {
            let value = max_value(&result(state, &action), alpha, beta, depth);
            if value < beta || best_action.is_none() {
                beta = value;
                best_action = Some(action);
            }
        }
    }
    return if state.color == WHITE { ( best_action, alpha) } else { ( best_action, beta ) };
}

pub fn time_bound_alpha_beta_search(state: &State, depth: u32, end_instant: Instant) -> (Option<Move>, i32, bool) {

    fn max_value(state: &State, mut alpha: i32, beta: i32, depth: u32, end_instant: Instant) -> (i32, bool) {
        if Instant::now() >= end_instant {
            return (0, false);
        }
        if depth == 0 || terminal_test(state) {
            return (heuristic(state), true);
        }
        let mut best_value = std::i32::MIN;
        let mut completed = true;

        for action in actions(&state) {
            let result = min_value(&result(&state, &action), alpha, beta, depth - 1, end_instant);
            let value = result.0;
            completed = result.1;
            best_value = max(best_value, value);
            alpha = max(alpha, value);
            if beta <= alpha || !completed {
                break;
            }
        }
        return (best_value, completed);
    };

    fn min_value(state: &State, alpha: i32, mut beta: i32, depth: u32, end_instant: Instant) -> (i32, bool) {
        if Instant::now() >= end_instant {
            return (0, false);
        }
        if depth == 0 || terminal_test(state) {
            return (heuristic(state), true);
        }
        let mut best_value = std::i32::MAX;
        let mut completed = true;

        for action in actions(&state) {
            let result = max_value(&result(&state, &action), alpha, beta, depth - 1, end_instant);
            let value = result.0;
            completed = result.1;
            best_value = min(best_value, value);
            beta = min(beta, value);
            if beta <= alpha || !completed {
                break;
            }
        }
        return (best_value, completed);
    };

    let mut best_action = None;
    let mut alpha = std::i32::MIN;
    let mut beta = std::i32::MAX;
    let mut completed = true;
    for action in actions(state) {
        if state.color == WHITE {
            let result = min_value(&result(state, &action), alpha, beta, depth, end_instant);
            let value = result.0;
            completed = result.1;
            if value > alpha || best_action.is_none() {
                alpha = value;
                best_action = Some(action);
            }
        } else {
            let result = max_value(&result(state, &action), alpha, beta, depth, end_instant);
            let value = result.0;
            completed = result.1;
            if value < beta || best_action.is_none() {
                beta = value;
                best_action = Some(action);
            }
        }
        if Instant::now() >= end_instant {
            completed = false;
            break;
        }
    }
    return if state.color == WHITE { ( best_action, alpha, completed ) } else { ( best_action, beta, completed ) };
}

pub fn iterative_time_bound_alpha_beta_search(state: &State, depth: u32, end_instant: Instant) -> Option<Move> {
    let mut best_action: Option<Move> = None;
    let mut best_value: i32 = if state.color == WHITE {
        std::i32::MIN
    } else {
        std::i32::MAX
    };
    let mut current_depth: u32 = 0;

    let start_instant = Instant::now();
    while current_depth <= depth && Instant::now() < end_instant {
        let result = time_bound_alpha_beta_search(state, current_depth, end_instant);
        let completed = result.2;
        if !completed {
            info!("Depth {} not completed, discarding it", current_depth);
            break;
        }
        if best_action.is_none() {
            best_action = result.0;
            best_value = result.1;
        }
        else if state.color == WHITE && result.1 > best_value {
            best_action = result.0;
            best_value = result.1;
            if result.1 == std::i32::MAX {
                break;
            }
        }
        else if state.color == BLACK && result.1 < best_value {
            best_action = result.0;
            best_value = result.1;
            if result.1 == std::i32::MIN {
                break;
            }
        }
        if (state.color == WHITE && best_value == std::i32::MAX) || (state.color == BLACK && best_value == std::i32::MIN) {
            break;
        }
        info!("Depth {} in {:?} with chosen move {} with value {}", current_depth, start_instant.elapsed(), result.0.unwrap(), result.1);
        current_depth += 1;
    }
    best_action
}

#[allow(dead_code)]
pub fn concurrent_iterative_alpha_beta_search(state: &State, depth: u32, end_instant: Instant, workers: u32) -> Option<Move> {
    let mut current_depth = 0;
    let mut best_action: Option<Move> = None;
    let mut best_value: i32 = if state.color ==  WHITE {
        std::i32::MIN
    } else {
        std::i32::MAX
    };

    let possible_results: Vec<(State, Move)> = actions(&state).iter().map(|a: &Move| (result(&state, a), a.clone())).collect::<Vec<(State, Move)>>();

    let starting_instant = Instant::now();
    while current_depth <= depth {
        info!("Current depth {} in {:?}", current_depth, starting_instant.elapsed());
        let mut evaluated_actions: Vec<(Move, i32)> = vec![];

        crossbeam::scope(|scope| {
            for slice in possible_results.chunks(possible_results.len() / workers as usize) {
                let handle = scope.spawn(move |_| {
                    let mut thread_evaluated_results: Vec<(Move, i32)> = vec![];
                    for (s, m) in slice.iter() {
                        let result = time_bound_alpha_beta_search(s, current_depth, end_instant);
                        thread_evaluated_results.push((m.clone(), result.1));
                    }
                    thread_evaluated_results
                });
                evaluated_actions.append(&mut handle.join().unwrap());
            }
        }).unwrap();

        for action in evaluated_actions {
            if best_action.is_none() || (state.color == WHITE && best_value < action.1) ||
                (state.color == BLACK && best_value > action.1) {
                best_action = Some(action.0);
                best_value = action.1;
            }
        }

        if Instant::now() >= end_instant {
            break;
        }

        current_depth += 1;
    }

    best_action
}

#[allow(dead_code)]
pub fn search_random(state: &State) -> Move {
    let actions = actions(state);
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range::<usize, usize, usize>(0, actions.len()-1);
    actions.get(i).cloned().unwrap()
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::game::Board;
    use std::time::Duration;

    #[test]
    fn test_heuristic() {
        let mut state = State::init(WHITE.to_string());
        let mut score: i32 = heuristic(&state);
        assert_eq!(score, -10);

        state.board = Board::new([
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 2, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 2, 2, 3, 2, 0, 1, 2],
            [2, 2, 1, 1, 0, 1, 1, 2, 2],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        score = heuristic(&state);
        assert_eq!(score, -20);

        let mut state = State::init(BLACK.to_string());
        state.board = Board::new([
            [2, 0, 3, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 2, 1, 0],
            [2, 0, 0, 0, 1, 0, 0, 0, 2],
            [2, 2, 1, 1, 0, 1, 1, 2, 2],
            [2, 0, 0, 0, 1, 0, 2, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        score = heuristic(&state);
        assert_eq!(score, std::i32::MAX);

        let mut state = State::init(BLACK.to_string());
        state.board = Board::new([
            [0, 0, 0, 2, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 2, 0, 0, 0],
            [2, 0, 0, 1, 3, 0, 0, 0, 2],
            [2, 2, 1, 1, 0, 2, 0, 2, 2],
            [2, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let m = Move {
            from: Position {
                x: 8,
                y: 3
            },
            to: Position {
                x: 5,
                y: 3
            }
        };
        state.apply_move(&m);
        let score = heuristic(&state);
        assert_eq!(score, -95);

        let mut state = State::init(WHITE.to_string());
        state.board = Board::new([
            [0, 0, 0, 2, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [2, 0, 0, 1, 0, 0, 0, 0, 2],
            [2, 2, 1, 1, 3, 2, 0, 2, 2],
            [2, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        state.apply_move(&Move {
            from: Position {
                x: 4,
                y: 4
            },
            to: Position {
                x: 4,
                y: 3
            }
        });
        state.board = Board::new([
            [0, 0, 0, 2, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [2, 0, 0, 1, 3, 0, 0, 0, 2],
            [2, 2, 1, 1, 0, 2, 0, 2, 2],
            [2, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        state.apply_move(&Move {
            from: Position {
                x: 3,
                y: 0
            },
            to: Position {
                x: 3,
                y: 2
            }
        });
        let score = heuristic(&state);
        assert_eq!(score, 5000);

        let mut state = State::init(BLACK.to_string());
        state.board = Board::new([
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 2, 0],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [0, 0, 0, 2, 0, 3, 0, 0, 2],
            [2, 2, 0, 2, 0, 0, 1, 2, 2],
            [0, 0, 2, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0]
        ]);
        let score1 = heuristic(&state);
        state.board = Board::new([
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 2, 0],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [0, 0, 0, 2, 0, 3, 0, 0, 2],
            [2, 2, 0, 2, 0, 0, 1, 2, 2],
            [0, 0, 2, 1, 0, 2, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let score2 = heuristic(&state);
        assert!(score1 > score2);
    }

    #[test]
    fn test_time_bound_alpha_beta_search() {
        let mut state = State::init(BLACK.to_string());
        state.board = Board::new([
            [0, 0, 0, 2, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [2, 0, 0, 1, 3, 0, 0, 0, 2],
            [2, 2, 1, 1, 0, 2, 0, 2, 2],
            [2, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let end_instant = Instant::now().checked_add(Duration::new(60, 0)).unwrap();
        let chosen_move = time_bound_alpha_beta_search(&state, 2, end_instant);
        let predicted_move = Move {
            from: Position {
                x: 8,
                y: 3,
            },
            to: Position {
                x: 5,
                y: 3
            }
        };
        assert!(chosen_move.0.is_some());
        assert_eq!(chosen_move.0.unwrap(), predicted_move);
    }

    #[test]
    fn test_iterative_time_bound_alpha_beta_search() {
        let mut state = State::init(WHITE.to_string());
        state.board = Board::new([
            [2, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 2, 1, 0],
            [2, 0, 3, 0, 1, 0, 0, 0, 2],
            [2, 2, 1, 1, 0, 1, 1, 2, 2],
            [2, 0, 0, 0, 1, 0, 2, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0, 0]
        ]);
        let end_instant = Instant::now().checked_add(Duration::new(60, 0)).unwrap();
        let chosen_move = iterative_time_bound_alpha_beta_search(&state, 2, end_instant);
        let predicted_move = Move {
            from: Position {
                x: 2,
                y: 3,
            },
            to: Position {
                x: 2,
                y: 0
            }
        };
        assert!(chosen_move.is_some());
        assert_eq!(chosen_move.unwrap(), predicted_move);

        let mut state = State::init(BLACK.to_string());
        state.board = Board::new([
            [0, 0, 0, 2, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [2, 0, 0, 1, 3, 0, 0, 0, 2],
            [2, 2, 1, 1, 0, 2, 0, 2, 2],
            [2, 0, 0, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let end_instant = Instant::now().checked_add(Duration::new(60, 0)).unwrap();
        let chosen_move = iterative_time_bound_alpha_beta_search(&state, 2, end_instant);
        let predicted_move = Move {
            from: Position {
                x: 8,
                y: 3,
            },
            to: Position {
                x: 5,
                y: 3
            }
        };
        assert!(chosen_move.is_some());
        assert_eq!(chosen_move.unwrap(), predicted_move);

        let mut state = State::init(BLACK.to_string());
        state.board = Board::new([
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 1, 2, 0, 0, 2, 0, 0, 0],
            [2, 0, 3, 0, 0, 2, 0, 0, 0],
            [2, 2, 0, 2, 0, 2, 0, 2, 2],
            [0, 0, 2, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0]
        ]);
        let end_instant = Instant::now().checked_add(Duration::new(60, 0)).unwrap();
        let chosen_move = iterative_time_bound_alpha_beta_search(&state, 2, end_instant);
        let predicted_move = Move {
            from: Position {
                x: 1,
                y: 4,
            },
            to: Position {
                x: 2,
                y: 4
            }
        };
        assert!(chosen_move.is_some());
        assert_eq!(chosen_move.unwrap(), predicted_move);

        let mut state = State::init(BLACK.to_string());
        state.board = Board::new([
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 2, 0],
            [0, 0, 0, 0, 1, 2, 0, 0, 0],
            [0, 0, 0, 2, 0, 3, 0, 0, 2],
            [2, 2, 0, 2, 0, 0, 1, 2, 2],
            [0, 0, 2, 1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 0, 0, 0, 0]
        ]);
        let end_instant = Instant::now().checked_add(Duration::new(60, 0)).unwrap();
        let chosen_move = iterative_time_bound_alpha_beta_search(&state, 2, end_instant);
        let predicted_move = Move {
            from: Position {
                x: 4,
                y: 5,
            },
            to: Position {
                x: 5,
                y: 5
            }
        };
        assert!(chosen_move.is_some());
        assert_eq!(chosen_move.unwrap(), predicted_move);
    }
}
