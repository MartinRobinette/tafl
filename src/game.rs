use crate::ai::AIPlayer;
use crate::human::HumanPlayer;
// did not want to use async-trait crate
pub enum Player {
    Human(HumanPlayer),
    AI(AIPlayer),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Attacker,
    Defender,
    King,
    Blank,
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
pub type Board = [[PieceType; BOARD_SIZE]; BOARD_SIZE];

fn new_brandubh() -> Board {
    use PieceType::{Attacker as A, Blank as B};
    use PieceType::{Defender as D, King as K};
    [
        [B, B, B, A, B, B, B],
        [B, B, B, A, B, B, B],
        [B, B, B, D, B, B, B],
        [A, A, D, K, D, A, A],
        [B, B, B, D, B, B, B],
        [B, B, B, A, B, B, B],
        [B, B, B, A, B, B, B],
    ]
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
        self.game = self.game.move_piece(src, dest);

        // ask
        self.game.defenders_turn = !self.game.defenders_turn;
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
            board: new_brandubh(), // only one board option
            defenders_turn: false, // attackers always make first move
            game_over: false,
            defender_won: false,
        }
    }

    fn piece_type(&self, tile: Tile) -> PieceType {
        self.board[tile.r][tile.c]
    }

    fn is_corner(&self, tile: Tile) -> bool {
        let size = self.board.len() - 1;
        tile.r == 0 && (tile.c == 0 || tile.r == size)
            || tile.c == 0 && (tile.r == 0 || tile.c == size)
    }
    fn throne_tile(&self) -> Tile {
        let size = self.board.len() - 1;
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
        vec![(0, -1), (0, 1), (1, 0), (-1, 0)]
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
        true
    }
    /// Checks for captures caused by given move, and if game has ended
    /// "update game"
    fn check_captures(&mut self, end: Tile) {
        let directions = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
        for dir in directions {
            let neighbor = next_tile(end, dir);
            let flank = next_tile(neighbor, dir);

            // capture if flanked
            if self.tile_on_board(flank) && self.enemy_piece(neighbor) && self.flanking_piece(flank)
            {
                if self.piece_type(neighbor) != PieceType::King {
                    self.board[neighbor.r][neighbor.c] = PieceType::Blank;
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

    pub fn move_piece(&self, src: Tile, dest: Tile) -> Game {
        // might want to add validation here for valid move
        // check end is blank, start is not blank, and start is current players piece
        // and check valid move function
        // could integrate selected piece into game struct to not have to recall get valid moves and ensue player piece

        let mut game = self.clone();
        game.board[dest.r][dest.c] = game.board[src.r][src.c];
        game.board[src.r][src.c] = PieceType::Blank;

        // check for king on exit
        if game.piece_type(dest) == PieceType::King && self.is_corner(dest) {
            game.defender_won = true;
            game.game_over = true;
        }

        game.check_captures(dest);

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
        tile.r < self.board.len() && tile.c < self.board.len()
    }

    pub fn tile_is_empty(&self, tile: Tile) -> bool {
        matches!(self.board[tile.r][tile.c], PieceType::Blank)
    }

    /// Returns true if the tile is a defender or king
    pub fn is_defender(&self, src: Tile) -> bool {
        match self.board[src.r][src.c] {
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

    pub fn get_valid_moves(&self, src: Tile) -> Vec<Tile> {
        // no piece can end on the throne, but can move though it
        // only king can end on an exit (corner)
        let mut valid_moves = Vec::<Tile>::new();
        if !self.tile_on_board(src) || self.tile_is_empty(src) {
            return valid_moves;
        }
        for (r, c) in &vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let dir = (*r, *c);
            let mut dest = next_tile(src, dir);
            while self.tile_on_board(dest) && self.tile_is_empty(dest) {
                if dest != self.throne_tile() {
                    valid_moves.push(dest);
                }
                dest = next_tile(dest, dir);
            }
        }
        valid_moves
    }

    pub fn get_all_valid_moves(&self) -> Vec<(Tile, Tile)> {
        let mut moves = Vec::<(Tile, Tile)>::new();
        for (r, row) in self.board.iter().enumerate() {
            for c in 0..row.len() {
                let tile = (r, c).into();
                if self.is_player_piece(tile) {
                    for dest in self.get_valid_moves(tile) {
                        moves.push((tile, dest));
                    }
                }
            }
        }
        moves
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
        for row in self.board.iter() {
            for piece in row.iter() {
                match piece {
                    PieceType::Defender => score += 1,
                    PieceType::Attacker => score -= 1,
                    PieceType::King | &PieceType::Blank => (),
                }
            }
        }
        score
    }
}
