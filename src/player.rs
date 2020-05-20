use crate::network::ServerConnection;
use crate::game::{State, Status, Move};
use crate::rules::game_status;
use crate::search::iterative_time_bound_alpha_beta_search;
use crate::serialization::*;
use std::io::Error;
use log::{info};
use std::time::{Instant, Duration};

pub struct Player {
     connection: ServerConnection,
     state: State
 }

 impl Player {
     pub fn init(name: String, color: String, address: String, port: u32) -> Result<Player, Error> {
         let mut connection = ServerConnection::connect(&address, port)?;
         connection.write_string(&name);
         Ok(Player {
             connection,
             state: State::init(color)
         })
     }

     fn make_move(&mut self) {
         let start_instant = Instant::now();
         let end_instant = start_instant.checked_add(Duration::new(58, 0)).unwrap();
         let m: Move = iterative_time_bound_alpha_beta_search(&self.state, 6, end_instant).unwrap();
         // let m: Move = alpha_beta_search(&self.state, 3).0.unwrap();
         info!("Chosen move: {} in {:?}", m, start_instant.elapsed());
         self.connection.write_string(&serialize_move(&m, &self.state.color));
     }

     fn receive_game_state(&mut self)  {
         let res: String = self.connection.read_string();
         self.state.board = deserialize_board(&res);
         self.state.turn = deserialize_turn(&res);
         self.state.history.push(self.state.board);
         self.state.status = game_status(&self.state);
     }


     pub fn game_loop(&mut self) {
         // info!("=== Initial Board ===");
         self.receive_game_state();
         // info!("{}", self.state.board);

         loop {
             if self.state.turn == self.state.color {
                 // info!("\n\n=== My turn ===");
                 self.make_move();
             } else {
                 // info!("\n\n=== Enemy turn ===\n");
             }
             self.receive_game_state();
             // info!("{}", self.state.board);
             // info!("{:?}", self.state.board);

             match self.state.status {
                 Status::WIN => { info!("WON!"); break; },
                 Status::LOSS => { info!("LOST :("); break; },
                 Status::DRAW => { info!("DRAW!"); break; }
                 Status::ONGOING => { continue; }
             }
         }
         info!("Game ended.");
     }
 }