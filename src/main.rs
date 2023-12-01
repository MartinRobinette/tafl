// we only hold when printing the display
#![allow(clippy::await_holding_refcell_ref)]

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
    // thread::sleep(time::Duration::from_millis(100));
    display.borrow_mut().draw_game(&game_state.game).await;

    // Main graphics / input loop
    let mut total_time = 0_f64;
    let mut total_turns = 0;

    loop {
        let time = macroquad::time::get_time();
        game_state.next_turn().await;
        let time2 = macroquad::time::get_time();
        let time_taken = time2 - time;

        // only increment if plyaer is ai
        if let Player::AI(_) = game_state.current_player() {
            total_time += time_taken;
            total_turns += 1;
        }
        // println!("{}", game_state.game.board);
        println!("average time: {}", total_time / total_turns as f64);

        if game_state.game.game_over {
            break;
        }
        // render game
        display.borrow_mut().draw_game(&game_state.game).await;
        //wait for click
        display.borrow_mut().next_tile_click(&game_state.game).await;
    }
    // display winner
    println!("defender won: {:?}", game_state.game.defender_won);
    println!("game took {} turns", total_turns);
    println!("{}", game_state.game.board);
    println!("game over");
    loop {
        display.borrow_mut().draw_game(&game_state.game).await;
    }
}
