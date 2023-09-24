use crate::game::*;
use macroquad::prelude::*;

const BOARD_SIZE: f32 = 600.0;
const SCREEN_EDGE: f32 = 20.0;

const SCREEN_WIDTH: f32 = BOARD_SIZE + 2.0 * SCREEN_EDGE;
const TOP_BAR_HEIGHT: f32 = 28.0;
const SCREEN_HEIGHT: f32 = BOARD_SIZE + TOP_BAR_HEIGHT + 2.0 * SCREEN_EDGE;

const TILE_SIZE: f32 = BOARD_SIZE / 7.0;
const PIECE_SIZE: f32 = TILE_SIZE / 2.5;

pub fn highlight_tile(r: usize, c: usize) {
    let x = tile_position(r) + 2.0;
    let y = tile_position(c) + 2.0;
    draw_rectangle_lines(x, y, TILE_SIZE - 4.0, TILE_SIZE - 4.0, 2.0, GREEN)
}

pub fn set_screen_size() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
}

pub fn tile_position(i: usize) -> f32 {
    TILE_SIZE * (i as f32) + SCREEN_EDGE
}

pub fn rev_tile_position(i: f32) -> i32 {
    ((i - SCREEN_EDGE - TILE_SIZE / 2.0) / TILE_SIZE).round() as i32
}

pub fn mouse_tile_position() -> Option<(usize, usize)> {
    let (x, y) = mouse_position();
    let (r, c) = (rev_tile_position(x), rev_tile_position(y));
    if r < 0 || c < 0 {
        ()
    }
    Some((r as usize, c as usize))
}

pub fn draw_board(tile_count: usize) {
    for r in 0..tile_count {
        for c in 0..tile_count {
            let x = tile_position(r);
            let y = tile_position(c);
            draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 2.0, BLUE)
        }
    }
}

pub fn draw_pieces(board: &Board) {
    for (r, row) in board.clone().iter().enumerate() {
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
