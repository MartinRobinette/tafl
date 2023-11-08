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

// board and turn as game
// have main hold the players

pub struct Game {
    pub board: Board,
    pub defenders_turn: bool,
}

pub struct GameState {
    pub game: Game,
    // used for highlighting options and moving pieces
    pub current_selection: Option<Tile>,
    // ? valid_moves: Vec<Tile>,
    defender_player: PlayerType,
    attacker_player: PlayerType,
}

//pub fn takeTurn()
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

const BOARD_SIZE: usize = 7; // TODO: unify board size
pub type Board = [[PieceType; BOARD_SIZE]; BOARD_SIZE];

// Brandubh style board
fn new_brandubh() -> Board {
    // 7 x 7 board
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

fn next_tile(src: Tile, dir: (i32, i32)) -> Tile {
    (src.r as i32 + dir.0, src.c as i32 + dir.1).into()
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Copy)]
enum PlayerType {
    Human,
    AI,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            game: Game::new(),
            current_selection: None,
            defender_player: PlayerType::Human,
            attacker_player: PlayerType::Human,
        }
    }

    fn current_player(&self) -> PlayerType {
        if self.game.defenders_turn {
            self.defender_player
        } else {
            self.attacker_player
        }
    }
    pub fn tile_clicked(&mut self, tile: Tile) {
        if let PlayerType::Human = self.current_player() {
            self.player_turn(tile);
        }
    }

    fn player_turn(&mut self, tile: Tile) {
        // if previous tile selected
        if let Some(selected) = self.current_selection {
            // if tile is valid move
            if self.game.get_valid_moves(selected).contains(&tile) {
                // move piece
                self.game.move_piece(selected, tile);
                self.current_selection = None;
                self.game.defenders_turn = !self.game.defenders_turn;
            } else {
                // prev selection and non valid move
                match self.game.is_player_piece(tile) {
                    true => self.current_selection = None,
                    false => self.current_selection = Some(tile),
                }
            }
        } else {
            // no previous tile selected
            match self.game.is_player_piece(tile) {
                true => self.current_selection = Some(tile),
                false => self.current_selection = None,
            }
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: new_brandubh(), // only one board option
            defenders_turn: false, // attackers always make first move
        }
    }

    // favor no mutation TODO: return a new game
    fn move_piece(&mut self, src: Tile, dest: Tile) {
        // might want to add validation here for valid move
        // check end is blank, start is not blank, and start is current players piece
        // and check valid move function
        // could integrate selected piece into game struct to not have to recall get valid moves and ensue player piece
        self.board[dest.r][dest.c] = self.board[src.r][src.c];
        self.board[src.r][src.c] = PieceType::Blank;

        // check for captures
        let directions = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
        for dir in directions {
            let next = next_tile(dest, dir);
            let flank = next_tile(next, dir);

            // capture if flanked
            if self.tile_on_board(flank) && self.enemy_piece(next) && self.friendly_piece(flank) {
                self.board[next.r][next.c] = PieceType::Blank;
            }
            // TODO:
            // if next if king bead needs to be flanked on all four sides, depending on rule set
            // need to check if flank is corner or throne, depending on rule set

            // win conditions
            // opponent has no move options
            // king is captured and all defender beads are captured
            // king bead on any of 4 corners (some rules say any edge piece)
        }
    }

    fn friendly_piece(&self, tile: Tile) -> bool {
        if self.tile_is_empty(tile) {
            return false;
        }
        self.defenders_turn == self.is_defender(tile)
    }

    fn enemy_piece(&self, tile: Tile) -> bool {
        if self.tile_is_empty(tile) {
            return false;
        }
        self.defenders_turn != self.is_defender(tile)
    }

    pub fn board_size(&self) -> usize {
        // used in display
        7 // only one board option currently // TODO: unify board size
    }

    pub fn tile_on_board(&self, tile: Tile) -> bool {
        // TODO: look at const types
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
        let mut valid_moves = Vec::<Tile>::new();
        if !self.tile_on_board(src) || self.tile_is_empty(src) {
            return valid_moves;
        }
        for (r, c) in &vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let dir = (*r, *c);
            let mut dest = next_tile(src, dir);
            while self.tile_on_board(dest) && self.tile_is_empty(dest) {
                valid_moves.push(dest);
                dest = next_tile(dest, dir);
            }
        }
        valid_moves
    }
}

//fn new_tawlbwrdd() -> Board {
//    // 11 x 11 board
//    let array = [
//        [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
//        [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
//        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
//        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
//        [1, 1, 0, 0, 0, 2, 0, 0, 0, 1, 1],
//        [1, 1, 2, 2, 2, 3, 2, 2, 2, 1, 1],
//        [1, 1, 0, 0, 0, 2, 0, 0, 0, 1, 1],
//        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
//        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
//        [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
//        [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
//    ];
//    array.map(|row| row.map(|cell| cell.into()))
//}
