pub mod ai;
pub mod game;
pub mod graphics;
pub mod human;

pub mod preleude {
    pub use crate::game::{Game, PieceType, Tile};
    pub use crate::graphics::Display;
}
