pub const NAME: &str = "muscovite";

pub const WHITE: &str = "white";
pub const BLACK: &str = "black";

// pub const WIN: i32 = 1;
// pub const LOSS: i32 = -1;
// pub const DRAW: i32 = 0;
// pub const ONGOING: i32 = 2;

pub const DEFAULT_WHITE_PORT: u32 = 5800;
pub const DEFAULT_BLACK_PORT: u32 = 5801;

// Cell contents
pub const W: u32 = 1; // White
pub const B: u32 = 2; // Black
pub const K: u32 = 3; // King
pub const E: u32 = 0; // Empty

// Cell types
pub const R: u32 = 10; // Regular
pub const C: u32 = 20; // Camp
pub const T: u32 = 30; // Throne
pub const F: u32 = 40; // Escape Free

// pub const COLUMNS: u32 = 9;
// pub const ROWS: u32 = 9;
pub const BOARD_COLUMNS: [char; 9] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
pub const BOARD: [[u32; 9]; 9] = [
    [R, F, F, C, C, C, F, F, R],
    [F, R, R, R, C, R, R, R, F],
    [F, R, R, R, R, R, R, R, F],
    [C, R, R, R, R, R, R, R, C],
    [C, C, R, R, T, R, R, C, C],
    [C, R, R, R, R, R, R, R, C],
    [F, R, R, R, R, R, R, R, F],
    [F, R, R, R, C, R, R, R, F],
    [R, F, F, C, C, C, F, F, R]
];

pub const INITIAL_BOARD: [[u32; 9]; 9] = [
    [0, 0, 0, B, B, B, 0, 0, 0],
    [0, 0, 0, 0, B, 0, 0, 0, 0],
    [0, 0, 0, 0, W, 0, 0, 0, 0],
    [B, 0, 0, 0, W, 0, 0, 0, B],
    [B, B, W, W, K, W, W, B, B],
    [B, 0, 0, 0, W, 0, 0, 0, B],
    [0, 0, 0, 0, W, 0, 0, 0, 0],
    [0, 0, 0, 0, B, 0, 0, 0, 0],
    [0, 0, 0, B, B, B, 0, 0, 0]
];
