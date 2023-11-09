use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time};
use tafl::ai::{AIKind, AIPlayer};
use tafl::game::{GameState, Player};
use tafl::graphics::Display;
use tafl::human::HumanPlayer;

#[macroquad::main("Tafl")]
async fn main() {
    let display = Rc::new(RefCell::new(Display::new()));

    // players
    let attacker = Player::AI(AIPlayer {
        kind: AIKind::Minimax,
    });
    //let defender = Player::Human(HumanPlayer::new(Rc::clone(&display)));
    //let defender = Player::Human(HumanPlayer::new(Rc::clone(&display)));
    let defender = Player::AI(AIPlayer {
        kind: AIKind::Minimax,
    });

    let mut game_state = GameState::new(defender, attacker);

    // render game once, to show initial state
    display.borrow_mut().draw_game(&game_state.game).await;

    // Main graphics / input loop
    let mut total_time = 0_f64;
    let mut total_turs = 0;
    loop {
        let time = macroquad::time::get_time();
        game_state.next_turn().await;
        let time2 = macroquad::time::get_time();
        let time_taken = time2 - time;
        //println!("turn took {} seconds", time_taken);
        if !game_state.game.defenders_turn {
            // tracks defender
            total_time += time_taken;
            total_turs += 1;
            println!("average time: {}", total_time / total_turs as f64);
        }
        if game_state.game.game_over {
            break;
        }
        // render game
        display.borrow_mut().draw_game(&game_state.game).await;
    }
    println!("game over");
    loop {
        display.borrow_mut().draw_game(&game_state.game).await;
    }
}
