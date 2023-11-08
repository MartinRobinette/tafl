use crate::game::{Game, PieceType, Tile};
use macroquad::prelude::*;
use std::{thread, time};

const BOARD_SIZE: f32 = 600.0;
const SCREEN_EDGE: f32 = 20.0;

const SCREEN_WIDTH: f32 = BOARD_SIZE + 2.0 * SCREEN_EDGE;
const TOP_BAR_HEIGHT: f32 = 28.0;
const SCREEN_HEIGHT: f32 = BOARD_SIZE + TOP_BAR_HEIGHT + 2.0 * SCREEN_EDGE;

//const NUM_TILES: i32 = 7; // TODO: unify board size
//const TILE_SIZE: f32 = BOARD_SIZE / NUM_TILES as f32;
const TILE_SIZE: f32 = 85.0;
const PIECE_SIZE: f32 = TILE_SIZE / 2.5;

pub struct Display {
    pub current_selection: Option<Tile>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            current_selection: None,
        }
    }

    pub async fn draw_game(&self, game: &Game) {
        clear_background(BLACK);
        request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
        draw_board(game);
        draw_pieces(game);

        //Highlight moves for selected tile
        if let Some(selected) = self.current_selection {
            for tile in game.get_valid_moves(selected) {
                highlight_tile(tile);
            }
        }

        // show fps
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);

        // limit to fps (macroquad will sync up frames with display refresh rate)
        thread::sleep(time::Duration::from_millis(20));

        next_frame().await;
    }

    pub async fn next_tile_click(&self, game: &Game) -> Tile {
        loop {
            // draw game need to happen first
            // as mouse pressed is frame dependent
            self.draw_game(game).await;

            if is_mouse_button_released(MouseButton::Left) {
                if let Some(tile) = mouse_tile_position() {
                    if game.tile_on_board(tile) {
                        return tile;
                    }
                }
            }
        }
    }
}

//
// Helper functions for Display
//

fn highlight_tile(tile: Tile) {
    //println!("{}", TILE_SIZE);
    let x = tile_position(tile.r) + 2.0;
    let y = tile_position(tile.c) + 2.0;
    draw_rectangle_lines(x, y, TILE_SIZE - 4.0, TILE_SIZE - 4.0, 2.0, GREEN)
}

/// maps tile index to pixel position
fn tile_position(i: usize) -> f32 {
    TILE_SIZE * (i as f32) + SCREEN_EDGE
}

fn tile_index_from_mouse(i: f32) -> usize {
    ((i - SCREEN_EDGE - TILE_SIZE / 2.0) / TILE_SIZE).round() as usize
}

fn mouse_out_of_bounds() -> bool {
    let (x, y) = mouse_position();
    let window_range = SCREEN_EDGE..=BOARD_SIZE + SCREEN_EDGE;
    !window_range.contains(&x) || !window_range.contains(&y)
}

// Option as mouse could be off of window
fn mouse_tile_position() -> Option<Tile> {
    if mouse_out_of_bounds() {
        return None;
    }
    let (x, y) = mouse_position();
    Some(Tile {
        r: tile_index_from_mouse(x),
        c: tile_index_from_mouse(y),
    })
}

fn draw_board(game: &Game) {
    let tile_count = game.board_size();
    for r in 0..tile_count {
        for c in 0..tile_count {
            let x = tile_position(r);
            let y = tile_position(c);
            draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 2.0, BLUE)
        }
    }
}

fn draw_pieces(game: &Game) {
    for (r, row) in game.board.clone().iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            use PieceType::*;
            let color = match item {
                King => WHITE,
                Attacker => RED,
                Defender => BLUE,
                Blank => BLACK,
            };
            let x = tile_position(r) + TILE_SIZE / 2.0;
            let y = tile_position(c) + TILE_SIZE / 2.0;
            draw_circle(x, y, PIECE_SIZE, color);
        }
    }
}
