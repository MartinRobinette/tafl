use crate::game::{Game, Tile};
use rand::seq::SliceRandom;

pub enum AIKind {
    Random,
    Minimax,
}

pub struct AIPlayer {
    pub kind: AIKind,
}

impl AIPlayer {
    pub fn take_turn(&self, game: &Game) -> (Tile, Tile) {
        match self.kind {
            AIKind::Random => self.random_turn(game),
            AIKind::Minimax => self.minimax_turn(game),
        }
    }
    // minimax ai
    fn minimax_turn(&self, game: &Game) -> (Tile, Tile) {
        // defender is maximizing agent
        let is_maximizing = game.defenders_turn;
        let depth = 4;

        let alpha = std::i32::MIN;
        let beta = std::i32::MAX;

        let mut best_src = None;
        let mut best_dest = None;

        let mut best_score = if is_maximizing {
            std::i32::MIN
        } else {
            std::i32::MAX
        };
        for (src, dest) in game.get_all_valid_moves() {
            let new_game = game.move_piece(src, dest);
            let score = self.minimax(new_game, is_maximizing, depth, alpha, beta);
            if is_maximizing && score > best_score {
                best_score = score;
                best_src = Some(src);
                best_dest = Some(dest);
            }
            if !is_maximizing && score < best_score {
                best_score = score;
                best_src = Some(src);
                best_dest = Some(dest);
            }
        }
        if let None = best_src {
            panic!("no valid moves");
        }

        (best_src.unwrap(), best_dest.unwrap())
    }
    // depth counts down and stops at zero
    fn minimax(&self, game: Game, is_maximizing: bool, depth: u32, alpha: i32, beta: i32) -> i32 {
        // check depth
        // check terminal state
        if depth == 0 || game.game_over {
            return game.score();
        }

        if is_maximizing {
            let mut max = std::i32::MIN;
            let mut alpha = alpha;
            for (src, dest) in game.get_all_valid_moves() {
                let new_game = game.move_piece(src, dest);
                let score = self.minimax(new_game, false, depth - 1, alpha, beta);
                max = std::cmp::max(max, score);
                if max >= beta {
                    break;
                }
                alpha = std::cmp::max(alpha, max);
            }
            max
        } else {
            // minimizing agent
            let mut min = std::i32::MAX;
            let mut beta = beta;
            for (src, dest) in game.get_all_valid_moves() {
                let new_game = game.move_piece(src, dest);
                let score = self.minimax(new_game, true, depth - 1, alpha, beta);
                min = std::cmp::min(min, score);
                if min <= alpha {
                    break;
                }
                beta = std::cmp::min(beta, min);
            }
            min
        }
    }
    // random ai
    fn random_turn(&self, game: &Game) -> (Tile, Tile) {
        let mut rng = rand::thread_rng();

        let pieces = friendly_piece_positions(game);
        loop {
            let src: Tile = *pieces.choose(&mut rng).unwrap(); // panics if no pieces
            let options: Vec<(Tile, Tile)> = game.get_valid_moves(src).collect();
            if options.len() == 0 {
                // loops forever if no possible moves
                continue;
            }
            break *options.choose(&mut rng).unwrap();
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
