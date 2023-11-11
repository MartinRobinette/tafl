use std::fmt::Display;

use crate::ai::AIPlayer;
use crate::human::HumanPlayer;
// did not want to use async-trait crate
pub enum Player {
    Human(HumanPlayer),
    AI(AIPlayer),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PieceType {
    Attacker,
    Defender,
    King,
    Blank,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match self {
            PieceType::Attacker => "A",
            PieceType::Defender => "D",
            PieceType::King => "K",
            PieceType::Blank => " ",
        };
        write!(f, "{}", piece_str)
    }
}

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub defenders_turn: bool,
    pub game_over: bool,
    pub defender_won: bool,
}

pub struct GameState {
    pub game: Game,
    // used for highlighting options and moving pieces
    pub current_selection: Option<Tile>,
    // ? valid_moves: Vec<Tile>,
    defender_player: Player,
    attacker_player: Player,
}

pub struct Action {
    pub src: Tile,
    pub dest: Tile,
}

//pub fn takeTurn()
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Tile {
    pub c: usize,
    pub r: usize,
}

impl From<(usize, usize)> for Tile {
    fn from((r, c): (usize, usize)) -> Self {
        Tile { r, c }
    }
}

// TODO: this needs to change, don't want negatives
// this can be fixed by adding direction enum
impl From<(i32, i32)> for Tile {
    fn from((row, col): (i32, i32)) -> Self {
        Tile {
            r: row as usize,
            c: col as usize,
        }
    }
}

const BOARD_SIZE: usize = 7; // TODO: unify board size
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Board(pub [[PieceType; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn new_brandubh() -> Board {
        use PieceType::{Attacker as A, Blank as B};
        use PieceType::{Defender as D, King as K};
        Board([
            [B, B, B, A, B, B, B],
            [B, B, B, A, B, B, B],
            [B, B, B, D, B, B, B],
            [A, A, D, K, D, A, A],
            [B, B, B, D, B, B, B],
            [B, B, B, A, B, B, B],
            [B, B, B, A, B, B, B],
        ])
    }
    #[cfg(test)]
    pub fn empty() -> Board {
        use PieceType::Blank as B;
        Board([
            [B, B, B, B, B, B, B],
            [B, B, B, B, B, B, B],
            [B, B, B, B, B, B, B],
            [B, B, B, B, B, B, B],
            [B, B, B, B, B, B, B],
            [B, B, B, B, B, B, B],
            [B, B, B, B, B, B, B],
        ])
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::new();
        let size = self.0.len();
        for r in 0..size {
            board_str.push('|');
            for c in 0..size {
                // print flipped to match display
                board_str.push_str(&format!("{}|", self.0[c][r]));
            }
            board_str.push('\n');
        }
        write!(f, "{}", board_str)
    }
}

// i do not like this
fn next_tile(src: Tile, dir: (i32, i32)) -> Tile {
    (src.r as i32 + dir.0, src.c as i32 + dir.1).into()
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new(defender: Player, attacker: Player) -> Self {
        GameState {
            game: Game::new(),
            current_selection: None,
            defender_player: defender,
            attacker_player: attacker,
        }
    }

    // take player turn
    pub async fn next_turn(&mut self) {
        let (src, dest) = match self.current_player() {
            Player::Human(human) => human.player_turn(&self.game).await,
            Player::AI(ai) => ai.take_turn(&self.game),
        };
        self.game = self.game.gen_next(src, dest); // this also changes turn
    }

    pub fn current_player(&self) -> &Player {
        if self.game.defenders_turn {
            &self.defender_player
        } else {
            &self.attacker_player
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new_brandubh(), // only one board option
            defenders_turn: false,        // attackers always make first move
            game_over: false,
            defender_won: false,
        }
    }

    fn piece_type(&self, tile: Tile) -> PieceType {
        self.board.0[tile.r][tile.c]
    }

    fn is_corner(&self, tile: Tile) -> bool {
        let size = self.board.0.len() - 1;
        (tile.r == size || tile.r == 0) && (tile.c == size || tile.c == 0)
    }

    fn throne_tile(&self) -> Tile {
        let size = self.board.0.len() - 1;
        (size / 2, size / 2).into()
    }
    fn empty_throne(&self, tile: Tile) -> bool {
        tile == self.throne_tile() && self.piece_type(self.throne_tile()) == PieceType::Blank
    }
    fn flanking_piece(&self, tile: Tile) -> bool {
        self.is_corner(tile) || self.empty_throne(tile) || self.friendly_piece(tile)
    }

    /// no checks for out of bounds, only to be used by check_king_capture
    fn adjacent_tiles(&self, tile: Tile) -> Vec<Tile> {
        [(0, -1), (0, 1), (1, 0), (-1, 0)]
            .iter()
            .map(|dir| next_tile(tile, *dir))
            .collect::<Vec<Tile>>()
    }

    /// assumes tile is already flanked and tile is king
    fn check_king_capture(&self, tile: Tile) -> bool {
        // on throne and flanked on all sides or next to throne and flanked on 3 sides
        if tile == self.throne_tile() || self.adjacent_tiles(self.throne_tile()).contains(&tile) {
            // must flanked on all sides (by throne or enemy)
            for dir in self.adjacent_tiles(tile) {
                if !self.flanking_piece(dir) {
                    return false;
                }
            }
        }
        // king cannot be captured against a corner
        if self
            .adjacent_tiles(tile)
            .iter()
            .any(|&adjacent| self.is_corner(adjacent))
        {
            return false;
        }
        true
    }
    /// Checks for captures caused by given move, and if game has ended
    /// "update game"
    /// TODO: change to not mut
    fn check_captures(&mut self, end: Tile) {
        let directions = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
        for dir in directions {
            let neighbor = next_tile(end, dir);
            let flank = next_tile(neighbor, dir);

            // capture if flanked
            if self.tile_on_board(flank) && self.enemy_piece(neighbor) && self.flanking_piece(flank)
            {
                if self.piece_type(neighbor) != PieceType::King {
                    self.board.0[neighbor.r][neighbor.c] = PieceType::Blank;
                } else if self.check_king_capture(neighbor) {
                    self.defender_won = false;
                    self.game_over = true;
                    return;
                }
            }

            // other win conditions
            // opponent has no move options
            // for loc in self.get_enemies() {
            //     // TODO: use has valid move function
            //     if !self.get_valid_moves(loc).is_empty() {
            //         return;
            //     }
            // }
        }
    }

    pub fn gen_next(&self, src: Tile, dest: Tile) -> Game {
        // might want to add validation here for valid move
        // check end is blank, start is not blank, and start is current players piece
        // and check valid move function
        // could integrate selected piece into game struct to not have to recall get valid moves and ensue player piece

        let mut game = self.clone();
        game.board.0[dest.r][dest.c] = game.board.0[src.r][src.c];
        game.board.0[src.r][src.c] = PieceType::Blank;

        // check for king on exit
        if game.piece_type(dest) == PieceType::King && self.is_corner(dest) {
            game.defender_won = true;
            game.game_over = true;
        }

        game.check_captures(dest);

        //change turn
        game.defenders_turn = !game.defenders_turn;

        game
    }

    fn friendly_piece(&self, tile: Tile) -> bool {
        !self.tile_is_empty(tile) && self.defenders_turn == self.is_defender(tile)
    }

    fn enemy_piece(&self, tile: Tile) -> bool {
        !self.tile_is_empty(tile) && self.defenders_turn != self.is_defender(tile)
    }

    pub fn board_size(&self) -> usize {
        // used in display
        7 // only one board option currently // TODO: unify board size
    }

    pub fn tile_on_board(&self, tile: Tile) -> bool {
        tile.r < self.board.0.len() && tile.c < self.board.0.len()
    }

    pub fn tile_is_empty(&self, tile: Tile) -> bool {
        matches!(self.board.0[tile.r][tile.c], PieceType::Blank)
    }

    /// Returns true if the tile is a defender or king
    pub fn is_defender(&self, src: Tile) -> bool {
        match self.board.0[src.r][src.c] {
            PieceType::Attacker | PieceType::Blank => false,
            PieceType::King | PieceType::Defender => true,
        }
    }

    pub fn is_player_piece(&self, src: Tile) -> bool {
        if self.tile_is_empty(src) {
            return false;
        }

        self.defenders_turn == self.is_defender(src)
    }

    pub fn moves_in_direction(&self, src: Tile, dir: (i32, i32)) -> Vec<(Tile, Tile)> {
        let mut moves = Vec::<(Tile, Tile)>::new();

        let mut dest = next_tile(src, dir);

        while self.tile_on_board(dest) && self.tile_is_empty(dest) {
            if dest != self.throne_tile()
                && !(self.is_corner(dest) && !(self.piece_type(src) == PieceType::King))
            {
                moves.push((src, dest));
            }
            dest = next_tile(dest, dir);
        }

        moves
    }

    pub fn get_valid_moves(&self, src: Tile) -> impl Iterator<Item = (Tile, Tile)> + '_ {
        // no piece can end on the throne, but can move though it
        // only king can end on an exit (corner)

        // if no piece on src return empty
        if !self.tile_on_board(src) || self.tile_is_empty(src) {
            panic!("why are you checking valid moves for an empty tile");
        };

        [(0, -1), (0, 1), (1, 0), (-1, 0)]
            .iter()
            .flat_map(move |dir| self.moves_in_direction(src, *dir).into_iter())
    }

    // remove closure and call function
    pub fn get_all_valid_moves(&self) -> impl Iterator<Item = (Tile, Tile)> + '_ {
        let size = self.board.0.len();
        (0..size).flat_map(move |r| {
            (0..size)
                // for all locations
                .filter(move |&c| self.is_player_piece((r, c).into()))
                .flat_map(move |c| self.get_valid_moves((r, c).into()))
        })
    }

    pub fn score(&self) -> i32 {
        // defender maximizing
        if self.game_over && self.defender_won {
            return std::i32::MAX;
        }
        if self.game_over && !self.defender_won {
            return std::i32::MIN;
        }
        let mut score = 0;
        let attacker_score = 20;
        let defender_score = attacker_score * 2;
        let king_score = defender_score * 10;
        for (r, row) in self.board.0.iter().enumerate() {
            let mut has_def = false;
            let mut has_atk = false;
            for (c, piece) in row.iter().enumerate() {
                match piece {
                    PieceType::Defender => {
                        score += defender_score;
                        has_def = true
                    }
                    PieceType::Attacker => {
                        score -= attacker_score;
                        has_atk = true
                    }
                    PieceType::King => {
                        if (Tile { r, c }) == self.throne_tile() {
                            score -= 1; // move off of throne early
                        }
                        score += king_score;
                        has_def = true;
                    }
                    PieceType::Blank => (),
                }
            }
            score += if has_def { 1 } else { 0 };
            score += if has_atk { -1 } else { 0 };
        }
        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_game(board: Board) -> Game {
        Game {
            board,
            defenders_turn: true,
            game_over: false,
            defender_won: false,
        }
    }

    #[test]
    fn win_loss() {
        let mut board = Board::empty();
        board.0[0][3] = PieceType::King;
        let game = new_game(board);
        assert_eq!(game.score(), 201); // 200 king + 1 flank held

        let src = (0, 3).into();
        let dest = (0, 0).into();
        let game = game.gen_next(src, dest);
        assert!(game.game_over);
        assert!(game.defender_won);
        assert_eq!(game.score(), std::i32::MAX);
    }

    #[test]
    fn game_change_on_move() {
        let game = Game::new();
        let src = (2, 3).into();
        let dest = (2, 2).into();
        let new_game = game.gen_next(src, dest);

        assert_ne!(game.defenders_turn, new_game.defenders_turn);
        assert_ne!(game.board, new_game.board);

        assert_eq!(new_game.board.0[src.r][src.c], PieceType::Blank);
        assert_ne!(new_game.board.0[dest.r][dest.c], PieceType::Blank);
    }
}
