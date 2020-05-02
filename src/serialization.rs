use crate::constants::*;
use crate::game::{Move, Board};
use crate::serde::{Serialize, Deserialize};
use crate::serde_json::{Value, Map};


#[derive(Serialize, Deserialize)]
struct ServerMove {
    from: String,
    to: String,
    turn: String
}

pub fn serialize_move(m: &Move, color: &str) -> String {
    let sm: ServerMove = ServerMove {
        from: format!("{}{}", BOARD_COLUMNS[m.from.x as usize], m.from.y+1),
        to: format!("{}{}", BOARD_COLUMNS[m.to.x as usize], m.to.y+1),
        turn: color.to_string()
    };
    serde_json::to_string(&sm).unwrap()
}

// pub fn deserialize_move(s: String) -> Move {
//     let sm: ServerMove = serde_json::from_str(&s).unwrap();
//     let from_column: u32 = BOARD_COLUMNS.iter().position(|&c| c == sm.from.chars().nth(0).unwrap()).unwrap() as u32;
//     let from_row: u32 = sm.from.chars().nth(1).unwrap().to_digit(10).unwrap();
//     let to_column: u32 = BOARD_COLUMNS.iter().position(|&c| c == sm.to.chars().nth(0).unwrap()).unwrap() as u32;
//     let to_row: u32 = sm.to.chars().nth(1).unwrap().to_digit(10).unwrap();
//     Move {
//         from: Position {
//             x: from_column,
//             y: from_row
//         },
//         to: Position {
//             x: to_column,
//             y: to_row
//         }
//     }
// }

pub fn deserialize_board(input: &String) -> Board {
    let wrapper: Value = serde_json::from_str(&input).unwrap();
    let data: &Map<String, Value> = wrapper.as_object().unwrap();
    let matrix: &Vec<Value> = data.get("board").unwrap().as_array().unwrap();

    let mut board= [[0u32; 9]; 9];
    for (y, row_wrapper) in matrix.iter().enumerate() {
        let row: &Vec<Value>= row_wrapper.as_array().unwrap();
        for (x, cell_wrapper) in row.iter().enumerate() {
            let cell: &str = cell_wrapper.as_str().unwrap();
            if cell == "WHITE" {
                board[y][x] = W;
            } else if cell =="BLACK" {
                board[y][x] = B;
            } else if cell == "KING" {
                board[y][x] = K;
            } else {
                board[y][x] = E;
            }
        }
    }
    Board::new(board)
}

pub fn deserialize_turn(input: &String) -> String {
    let wrapper: Value = serde_json::from_str(&input).unwrap();
    let data: &Map<String, Value> = wrapper.as_object().unwrap();
    data.get("turn").unwrap().as_str().unwrap().to_string().to_lowercase()
}
