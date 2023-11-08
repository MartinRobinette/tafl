use crate::{
    game::{Game, Tile},
    graphics::Display,
};
use std::rc::Rc;

pub enum Player {
    Human(HumanPlayer),
    AI,
}
pub struct HumanPlayer {
    display: Rc<Display>,
}

impl HumanPlayer {
    pub fn new(display: Rc<Display>) -> Self {
        HumanPlayer { display }
    }
    pub async fn player_turn(&self, game: &Game) -> (Tile, Tile) {
        let mut src = self.display.next_tile_click(game).await;
        let mut dest = self.display.next_tile_click(game).await;

        //wait for valid move
        while !game.get_valid_moves(src).contains(&dest) {
            src = dest;
            dest = self.display.next_tile_click(game).await;
        }

        (src, dest)
    }
}
