use crate::game::{Game, Tile};
use rand::seq::SliceRandom;

pub enum AIKind {
    Random,
    Minimax(u32),
}

pub struct AIPlayer {
    pub kind: AIKind,
}

impl AIPlayer {
    pub fn take_turn(&self, game: &Game) -> (Tile, Tile) {
        match self.kind {
            AIKind::Random => self.random_turn(game),
            AIKind::Minimax(depth) => self.minimax_turn(game, depth),
        }
    }
    // minimax ai
    fn minimax_turn(&self, game: &Game, depth: u32) -> (Tile, Tile) {
        // defender is maximizing agent
        let is_maximizing = game.defenders_turn;

        let mut best_src = None;
        let mut best_dest = None;

        let mut best_score = if is_maximizing {
            std::i32::MIN
        } else {
            std::i32::MAX
        };

        for (src, dest) in game.get_all_valid_moves() {
            let new_game = game.gen_next(src, dest);
            let score = minimax(new_game, depth, std::i32::MIN, std::i32::MAX);
            //println!("score: {}", score);
            //println!("depth: {}, score: {}", depth, score);
            if best_dest == None {
                // init to first move
                best_src = Some(src);
                best_dest = Some(dest);
            }

            if (is_maximizing && score > best_score) || (!is_maximizing && score < best_score) {
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
    // // random ai
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

// depth counts down and stops at zero
fn minimax(game: Game, depth: u32, mut alpha: i32, mut beta: i32) -> i32 {
    let discount = 0.99;
    if depth == 0 || game.game_over {
        game.score()
    } else if game.defenders_turn {
        //maximizing player
        let mut max = std::i32::MIN;
        for (src, dest) in game.get_all_valid_moves() {
            let new_game = game.gen_next(src, dest);
            max = std::cmp::max(
                max,
                (minimax(new_game, depth - 1, alpha, beta) as f32 * discount) as i32,
            );
            if max > beta {
                break;
            }
            alpha = std::cmp::max(alpha, max);
        }
        max
    } else {
        // minimizing agent
        let mut min = std::i32::MAX;
        for (src, dest) in game.get_all_valid_moves() {
            let new_game = game.gen_next(src, dest);
            min = std::cmp::min(
                min,
                (minimax(new_game, depth - 1, alpha, beta) as f32 * discount) as i32,
            );
            if min < alpha {
                break;
            }
            beta = std::cmp::min(beta, min);
        }
        min
    }
}

// helper for random ai
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::{Board, PieceType};

    fn new_game(board: Board) -> Game {
        Game {
            board,
            defenders_turn: true,
            game_over: false,
            defender_won: false,
        }
    }

    fn take_minimax_turn(mut game: Game, depth: u32, defenders_turn: bool) -> Game {
        let ai = AIPlayer {
            kind: AIKind::Minimax(depth),
        };
        game.defenders_turn = defenders_turn;
        let (src, dest) = ai.take_turn(&game);
        game.gen_next(src, dest)
    }

    fn run_defender_only(mut game: Game, depth: u32, turns: i32) -> Game {
        for i in 0..turns {
            game = take_minimax_turn(game, depth, true);
            println!("Defender only Turn {} \n{}", i + 1, game.board);
            if game.game_over {
                break;
            }
        }
        game
    }

    #[test]
    fn take_the_winning_move_defender_only() {
        let mut board = Board::empty();
        board.0[3][3] = PieceType::King;
        board.0[2][2] = PieceType::Attacker;

        let game = new_game(board);

        println!("initial board \n{}", game.board);

        println!("testing depth 2");
        let depth_2 = run_defender_only(game.clone(), 2, 2);
        assert!(depth_2.game_over); // can win in 2 moves

        println!("testing depth 3");
        let depth_3 = run_defender_only(game.clone(), 3, 2);
        assert!(depth_3.game_over); // can win in 2 moves

        println!("testing depth 4");
        let depth_4 = run_defender_only(game, 3, 2);
        assert!(depth_4.game_over); // can win in 2 moves
    }

    fn run_minimax_game(mut game: Game, depth: u32, turns: i32) -> Game {
        for i in 0..turns {
            game = take_minimax_turn(game, depth, i % 2 == 0);
            println!("Turn {} \n{}", i + 1, game.board);
        }
        game
    }

    #[test]
    fn take_the_winning_move() {
        let mut board = Board::empty();
        board.0[3][3] = PieceType::King;
        board.0[2][2] = PieceType::Attacker;

        let game = new_game(board);

        println!("initial board \n{}", game.board);
        // should win in 2 moves (3 turns)
        println!("testing depth 2");
        let depth_2 = run_minimax_game(game.clone(), 2, 3);
        assert!(depth_2.game_over);

        println!("testing depth 3");
        let depth_3 = run_minimax_game(game.clone(), 3, 3);
        assert!(depth_3.game_over);

        println!("testing depth 4");
        let depth_4 = run_minimax_game(game, 3, 3);
        assert!(depth_4.game_over);
    }
}
