use crate::game::{Move, State};
use crate::rules::legal_moves;
use rand::Rng;

fn actions(state: &State) -> Vec<Move> {
    legal_moves(state)
}

pub fn random(state: &State) -> Move {
    let actions = actions(state);
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range::<usize, usize, usize>(0, actions.len()-1);
    actions.get(i).cloned().unwrap()
}

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
