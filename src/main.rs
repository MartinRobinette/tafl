use macroquad::prelude::*;
use tafl::display;
use tafl::display::highlight_tile;
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

        // highlight valid moves for tile under mouse
        if let Some(src) = display::mouse_tile_position() {
            println!("Calling get_valid_moves");
            get_valid_moves(src, board)
                .into_iter()
                .for_each(highlight_tile);
        }
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);
        next_frame().await;
    }
}
