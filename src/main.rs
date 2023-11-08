use std::rc::Rc;
use tafl::game::GameState;
use tafl::graphics;
use tafl::player::{HumanPlayer, Player};

#[macroquad::main("Tafl")]
async fn main() {
    let display = Rc::new(graphics::Display::new());
    let attacker = Player::Human(HumanPlayer::new(Rc::clone(&display)));
    let defender = Player::Human(HumanPlayer::new(Rc::clone(&display)));
    let mut game_state = GameState::new(defender, attacker);

    // Main graphics / input loop
    loop {
        // take human turn
        let (src, dest) = match game_state.current_player() {
            Player::Human(human) => human.player_turn(&game_state.game).await,
            Player::AI => todo!(),
        };

        game_state.game.move_piece(src, dest);

        // render game
        display.draw_game(&game_state.game).await;
    }
}
