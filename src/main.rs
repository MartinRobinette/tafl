use macroquad::prelude::*;
use tafl::display;
use tafl::display::highlight_tile;
use tafl::game::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game::new();

    //let mut mouse_pos = display::mouse_tile_position();

    // outline for how the code should be structured to handle game state and input
    // first we need to get the mouse position
    // then we need to check if the mouse is clicked

    // Main input loop
    loop {
        // draw board
        clear_background(BLACK);
        display::set_screen_size();
        display::draw_board(&game);
        display::draw_pieces(&game);

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

        // Highlight moves for selected tile
        if let Some(selected) = game.current_selection {
            for tile in game.get_valid_moves(selected) {
                highlight_tile(tile);
            }
        }

        // show fps
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);
        next_frame().await;
    }
}
