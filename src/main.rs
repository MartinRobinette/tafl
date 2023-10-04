use macroquad::prelude::*;
use tafl::display;
use tafl::display::highlight_tile;
use tafl::game::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let game = Game::new();

    loop {
        clear_background(BLACK);
        display::set_screen_size();
        display::draw_board(&game);
        display::draw_pieces(&game);

        // highlight valid moves for tile under mouse
        if let Some(src) = display::mouse_tile_position() {
            if !game.is_defender(src) {
                game.get_valid_moves(src)
                    .into_iter()
                    .for_each(highlight_tile);
            }
        }

        // show fps
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);
        next_frame().await;
    }
}
