use crate::game::{Game, Tile};
use rand::seq::SliceRandom;

pub enum AIKind {
    Random,
    //Minimax,
}

pub struct AIPlayer {
    pub kind: AIKind,
}

impl AIPlayer {
    pub fn take_turn(&self, game: &Game) -> (Tile, Tile) {
        match self.kind {
            AIKind::Random => self.random_turn(game),
            //AIKind::Minimax => self.minimax_turn(game),
        }
    }
    // random ai
    fn random_turn(&self, game: &Game) -> (Tile, Tile) {
        let mut rng = rand::thread_rng();

        let pieces = friendly_piece_positions(game);
        loop {
            let src: Tile = *pieces.choose(&mut rng).unwrap(); // panics if no pieces
            let options: Vec<Tile> = game.get_valid_moves(src);
            if options.len() == 0 {
                // loops forever if no possible moves
                continue;
            }
            let dest = *options.choose(&mut rng).unwrap();

            break (src, dest);
        }
    }
}

fn friendly_piece_positions(game: &Game) -> Vec<Tile> {
    // TODO: change to iterator?
    let mut positions = Vec::new();
    let board_size = game.board_size();

    // TODO: this should not be here
    for r in 0..board_size {
        for c in 0..board_size {
            let tile = (r, c).into();
            if game.is_player_piece(tile) {
                positions.push(tile);
            }
        }
    }
    positions
}
