use macroquad::prelude::*;
use tafl::display;
use tafl::display::highlight_tile;
use tafl::game::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let game = Game::new();

    let mut mouse_pos = display::mouse_tile_position();

    loop {
        clear_background(BLACK);
        display::set_screen_size();
        display::draw_board(&game);
        display::draw_pieces(&game);

        // highlight valid moves for tile under mouse
        if is_mouse_button_released(MouseButton::Left) {
            mouse_pos = display::mouse_tile_position();
        }

        if let Some(src) = mouse_pos {
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
