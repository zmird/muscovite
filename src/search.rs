use crate::game::{Board, Move};
use crate::rules::legal_moves;
use rand::Rng;

pub fn random(board: &Board, color: &String ) -> Move {
    let moves: Vec<Move> = legal_moves(&board, &color);
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range::<usize, usize, usize>(0, moves.len()-1);
    moves.get(i).cloned().unwrap()
}

// fn states() {}
//
// fn result() {}
//
// fn terminal_state() {}
//
// fn utility() {}
//
// pub fn heuristic() -> i32 {
//     let x = rand::random::<u32>();
//     return x;
// }
