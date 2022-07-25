/*!
Welcome to `quarto_rs`
*/

#![warn(clippy::cargo)]
#![deny(clippy::cargo_common_metadata)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::unreadable_literal,
    clippy::type_repetition_in_bounds,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_docs_in_private_items
)]
#![deny(
    missing_debug_implementations,
    missing_docs,
    //trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_must_use,
    missing_docs,
    //unused_results
)]
#![cfg_attr(
    not(debug_assertions),
    deny(
        bad_style,
        const_err,
        dead_code,
        improper_ctypes,
        non_shorthand_field_patterns,
        no_mangle_generic_items,
        overflowing_literals,
        path_statements,
        patterns_in_fns_without_body,
        private_in_public,
        unconditional_recursion,
        unused,
        unused_allocation,
        unused_comparisons,
        unused_parens,
        while_true
    )
)]

use std::io;

use field::Pos;
use game::{Game, Player, Status};

use crate::{
    ai::SimpleAi,
    field::{try_parse_pos, Field},
    piece::{Piece, Property},
};

mod ai;
mod field;
mod game;
mod piece;
mod rng;

fn main() {
    /*let test_light_tall: Piece =
        Piece::new_with_props(Property::Tall as u8 | Property::Light as u8);
    let test_dark_short: Piece = Piece::new();

    let mut field = Field::new();

    field.put((3, 0), test_light_tall).unwrap();
    field.put((2, 1), test_light_tall).unwrap();
    field.put((1, 2), test_dark_short).unwrap();

    assert!(!field.check_field_for_win());

    field.put((0, 3), test_light_tall).unwrap();

    field.pp();*/

    ai_wars();

    return;

    let mut game = Game::new(Player::PlayerOne);

    let stdin = io::stdin();
    let mut buf = String::new();

    let mut ai = SimpleAi::new(Player::PlayerTwo, 1);

    println!("Let the games begin!");

    loop {
        game.pp();
        if !game.running() {
            return;
        }

        if game.player() == Player::PlayerOne {
            let piece_id: usize = loop {
                println!(
                    "{:?}, please chose the id of the next piece:",
                    game.player()
                );
                buf.clear();
                stdin.read_line(&mut buf).unwrap();
                let num = buf.trim().parse();
                if let Ok(num) = num {
                    if num < game.remaining_pieces().len() {
                        break num;
                    }
                }
                #[cfg(debug_assertions)]
                println!("{} (str: {})", num.err().unwrap(), &buf);
                println!(
                    "Illegal choice: {}, please pick the id of a remaining piece:",
                    buf
                );
                game.pp_remaining_pieces();
            };
            let next_piece = game.remaining_pieces()[piece_id];

            if game.is_initial_move() {
                game.initial_move(next_piece).unwrap();
            } else {
                loop {
                    println!("Select x,y to put the place to:");
                    buf.clear();
                    stdin.read_line(&mut buf).unwrap();
                    let pos = try_parse_pos(&buf);
                    if let Ok(pos) = pos {
                        if game.do_move(pos, next_piece).is_ok() {
                            break;
                        }
                    }
                    println!("Illegal move! The x,y value must be an empty place on the field!");
                }
            }
            println!();
        } else {
            game = ai.play_iteratively(&mut game).expect("AI should not fail!");
        }
    }
}

fn ai_wars() {
    let it = std::time::Instant::now();
    const ITERS: usize = 10_000;

    let mut ai_one_wins = 0;
    let mut ai_two_wins = 0;
    let mut turns = 0;

    'outer: for _ in 0..ITERS {
        let mut game = Game::new(Player::PlayerOne);

        let mut ai_one = SimpleAi::new(Player::PlayerOne, 1);
        let mut ai_two = SimpleAi::new(Player::PlayerTwo, 1);

        loop {
            //game.pp();
            if !game.running() {
                if let Status::Won { winner } = game.status {
                    if winner == Player::PlayerOne {
                        ai_one_wins += 1;
                    } else {
                        ai_two_wins += 1;
                    }
                }
                turns += game.round();
                continue 'outer;
            }

            if game.player() == Player::PlayerOne {
                game = ai_one
                    .play_iteratively(&mut game)
                    .expect("AI should not fail!");
            } else {
                game = ai_two
                    .play_iteratively(&mut game)
                    .expect("AI should not fail!");
            }
        }
    }

    let elapsed = it.elapsed();

    println!(
        "Did {} games in {} seconds({:05.3} games/sec)",
        ITERS,
        elapsed.as_secs(),
        ITERS as f64 / (elapsed.as_secs() as f64)
    );
    println!(
        "Did {} turns in total, average of {} turns per game",
        turns,
        turns as f64 / ITERS as f64
    );
    println!(
        "PlayerOne had {} wins({}%), PlayerTwo had {} wins({}%).",
        ai_one_wins,
        (ai_one_wins as f64 / ITERS as f64) * 100.,
        ai_two_wins,
        (ai_two_wins as f64 / ITERS as f64) * 100.
    );
    let mut draws = ITERS - ai_one_wins - ai_two_wins;
    println!(
        "We had {} draws ({}%)",
        draws,
        (draws as f64 / ITERS as f64) * 100.
    );
}
