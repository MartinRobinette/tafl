// we only hold when printing the display
#![allow(clippy::await_holding_refcell_ref)]

use std::cell::RefCell;
use std::rc::Rc;
use tafl::ai::{AIKind, AIPlayer};
use tafl::game::{GameState, Player};
use tafl::graphics::Display;
use tafl::human::HumanPlayer;

#[macroquad::main("Tafl")]
async fn main() {
    let display = Rc::new(RefCell::new(Display::new()));
    let depth = 4;
    // players
    // let attacker = Player::AI(AIPlayer {
    //     kind: AIKind::Minimax(depth),
    // });
    let defender = Player::AI(AIPlayer {
        kind: AIKind::Minimax(depth),
    });
    // let defender = Player::Human(HumanPlayer::new(Rc::clone(&display)));
    let attacker = Player::Human(HumanPlayer::new(Rc::clone(&display)));

    let mut game_state = GameState::new(defender, attacker);

    // render game once, to show initial state
    display.borrow_mut().draw_game(&game_state.game).await;

    // Main graphics / input loop
    loop {
        game_state.next_turn().await;

        if game_state.game.game_over {
            break;
        }
        // render game
        display.borrow_mut().draw_game(&game_state.game).await;
    }
    // display winner
    loop {
        display.borrow_mut().draw_game(&game_state.game).await;
    }
}
