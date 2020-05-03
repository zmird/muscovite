// use crate::constants::*;
use crate::network::ServerConnection;
use crate::game::{State, Status, Move};
use crate::rules::game_status;
use crate::search::alpha_beta_search;
use crate::serialization::*;
use std::io::Error;

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
         let m: Move = alpha_beta_search(&self.state, 1).unwrap();
         println!("Chosen move: {}", m);
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
         println!("=== Initial Board ===");
         self.receive_game_state();
         println!("{}", self.state.board);

         loop {
             if self.state.turn == self.state.color {
                 println!("\n=== My turn ===");
                 self.make_move();
             } else {
                 println!("\n=== Enemy turn ===\n");
             }
             self.receive_game_state();
             println!("{}", self.state.board);

             match self.state.status {
                 Status::WIN => { println!("WON!"); break; },
                 Status::LOSS => { println!("LOST :("); break; },
                 Status::DRAW => { println!("DRAW!"); break; }
                 Status::ONGOING => { continue; }
             }
         }
         println!("Game ended.");
     }
 }