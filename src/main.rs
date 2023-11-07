use macroquad::prelude::{
    draw_text, get_fps, is_mouse_button_released, next_frame, MouseButton, WHITE,
};
use std::{thread, time};
use tafl::display;
use tafl::game::*;

#[macroquad::main("Tafl")]
async fn main() {
    let mut game = Game::new();

    // Main graphics / input loop
    loop {
        // update mouse position
        if is_mouse_button_released(MouseButton::Left) {
            let mouse_pos = display::mouse_tile_position();
            // let game know if a tile is clicked
            if let Some(t) = mouse_pos {
                if game.tile_on_board(t) {
                    game.tile_clicked(t);
                }
            }
        }

        // render game
        display::draw_game(&game);

        // show fps
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);

        // limit to fps
        thread::sleep(time::Duration::from_millis(20));

        next_frame().await;
    }
}
