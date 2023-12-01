use crate::{
    game::{Game, Tile},
    graphics::Display,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct HumanPlayer {
    display: Rc<RefCell<Display>>,
}

// we only hold when printing the display
#[allow(clippy::await_holding_refcell_ref)]
impl HumanPlayer {
    pub fn new(display: Rc<RefCell<Display>>) -> Self {
        HumanPlayer { display }
    }

    async fn next_click(&self, game: &Game) -> Tile {
        self.display.borrow_mut().next_tile_click(game).await
    }

    fn set_selected(&self, tile: Option<Tile>) {
        self.display.borrow_mut().current_selection = tile;
    }
    pub async fn player_turn(&self, game: &Game) -> (Tile, Tile) {
        let mut last_src: Option<Tile> = None;
        loop {
            let src = match last_src {
                Some(tile) => tile,
                None => self.next_click(game).await,
            };

            if !game.is_player_piece(src) {
                continue;
            }
            self.set_selected(Some(src));

            let dest = self.next_click(game).await;

            // return valid move
            let valid_moves: Vec<(Tile, Tile)> = game.get_valid_moves(src).collect();
            if valid_moves.contains(&(src, dest)) {
                self.set_selected(None);
                return (src, dest);
            } else {
                // check if new player piece clicked on
                if game.is_player_piece(dest) {
                    last_src = Some(dest);
                    self.set_selected(None);
                }
            }
        }
    }
}
