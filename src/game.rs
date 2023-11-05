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
    pub board: Board,
    // used for highlighting options and moving pieces
    pub current_selection: Option<Tile>,
    // ? valid_moves: Vec<Tile>,
    pub defenders_turn: bool,
    //temp
    pub player_is_defender: bool,
    // have class with sub classes for AI or user input
    // Player attacker,
    // Player defender,
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

impl From<(usize, usize)> for Tile {
    fn from((row, col): (usize, usize)) -> Self {
        Tile { r: row, c: col }
    }
}

const BOARD_SIZE: usize = 7;
pub type Board = [[PieceType; BOARD_SIZE]; BOARD_SIZE];

// Brandubh style board
fn new_brandubh() -> Board {
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

impl Game {
    pub fn new() -> Self {
        Game {
            board: new_brandubh(), // only one board option
            current_selection: None,
            defenders_turn: true,
            player_is_defender: true,
        }
    }

    fn move_piece(&mut self, src: Tile, dest: Tile) {
        // might want to add validation here for valid move
        // check end is blank, start is not blank, and start is current players piece
        // and check valid move function
        // could integrate selected piece into game struct to not have to recall get valid moves and ensue player piece
        self.board[dest.r][dest.c] = self.board[src.r][src.c];
        self.board[src.r][src.c] = PieceType::Blank;
    }

    pub fn tile_clicked(&mut self, tile: Tile) {
        // assuming player turn

        // if previous tile selected
        if let Some(selected) = self.current_selection {
            // if tile is valid move
            if self.get_valid_moves(selected).contains(&tile) {
                // move piece
                self.move_piece(selected, tile);
                self.current_selection = None;
                self.defenders_turn = !self.defenders_turn;
                // also change player_is_defender to allow for user input
                self.player_is_defender = !self.player_is_defender;
            } else {
                // prev selection and non valid move
                match self.is_player_piece(tile) {
                    true => self.current_selection = None,
                    false => self.current_selection = Some(tile),
                }
            }
        } else {
            // no previous tile selected
            match self.is_player_piece(tile) {
                true => self.current_selection = Some(tile),
                false => self.current_selection = None,
            }
        }
    }

    pub fn board_size(&self) -> usize {
        7 // only one bard option
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
        self.player_is_defender == self.is_defender(src)
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
