// to be trait if more board types added
#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Attacker,
    Defender,
    King,
    Blank,
}

impl From<i32> for PieceType {
    fn from(item: i32) -> Self {
        match item {
            1 => PieceType::Attacker,
            2 => PieceType::Defender,
            3 => PieceType::King,
            _ => PieceType::Blank,
        }
    }
}

pub struct Game {
    board: Board,
    // Player attacker,
    // Player defender,
}

//pub fn takeTurn()

pub struct Tile {
    pub c: usize,
    pub r: usize,
}
impl From<(i32, i32)> for Tile {
    fn from((row, col): (i32, i32)) -> Self {
        Tile {
            r: row as usize,
            c: col as usize,
        }
    }
}

const BOARD_SIZE: usize = 7;
pub type Board = [[PieceType; BOARD_SIZE]; BOARD_SIZE];

// Brandubh style board
pub fn new_board() -> Board {
    let array = [
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 2, 0, 0, 0],
        [1, 1, 2, 3, 2, 1, 1],
        [0, 0, 0, 2, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
    ];
    array.map(|row| row.map(|cell| cell.into()))
}

pub fn valid_move() -> bool {
    false
}
