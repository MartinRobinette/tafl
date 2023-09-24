use macroquad::prelude::*;
use tafl::display;
use tafl::game::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let board = new_board();
    assert_eq!(board[0][0], 0.into());
    board.into_iter().for_each(|row| println!("{:?}", row));

    loop {
        clear_background(BLACK);
        display::set_screen_size();
        display::draw_board(board.len());
        display::draw_pieces(&board);

        // highlight tile under mouse
        match display::mouse_tile_position() {
            Some(tile) => display::highlight_tile(tile),
            None => (),
        }

        next_frame().await
    }
}
