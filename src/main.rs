use std::cell::RefCell;
use std::rc::Rc;
use tafl::ai::{AIKind, AIPlayer};
use tafl::game::{GameState, Player};
use tafl::graphics::Display;
use tafl::human::HumanPlayer;

#[macroquad::main("Tafl")]
async fn main() {
    let display = Rc::new(RefCell::new(Display::new()));

    // players
    let defender = Player::AI(AIPlayer {
        kind: AIKind::Random,
    });
    //let defender = Player::Human(HumanPlayer::new(Rc::clone(&display)));
    let attacker = Player::Human(HumanPlayer::new(Rc::clone(&display)));

    let mut game_state = GameState::new(defender, attacker);

    // Main graphics / input loop
    loop {
        game_state.next_turn().await;

        // render game
        display.borrow_mut().draw_game(&game_state.game).await;
    }
}
