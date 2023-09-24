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
