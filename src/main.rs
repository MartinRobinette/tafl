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

        match display::mouse_tile_position() {
            Some((r, c)) => display::highlight_tile(r, c),
            None => (),
        }

        //        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        next_frame().await
    }
}
