/*!
Welcome to `quarto_rs`
*/

#![warn(clippy::cargo)]
#![deny(clippy::cargo_common_metadata)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
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
        improper_ctypes,
        non_shorthand_field_patterns,
        no_mangle_generic_items,
        overflowing_literals,
        path_statements,
        patterns_in_fns_without_body,
        private_in_public,
        unconditional_recursion,
        unused_allocation,
        unused_comparisons,
        unused_parens,
        while_true
    )
)]

mod ai;
mod field;
mod game;
mod piece;
mod rng;

use std::{env::args, io::stdin};

use crate::{
    ai::SimpleAi,
    field::try_parse_pos,
    game::{Game, Player, Status},
    piece::Piece,
};

fn main() {
    if args().any(|x| x.contains("help") || x == "-h") {
        let current_exe = std::env::current_exe().unwrap();
        let current_exe_name = current_exe.file_name().unwrap().to_string_lossy();
        println!(
            "Your friendly Quarto game.

Usage: {current_exe_name} <Options>

Options:
    --ai-wars|-a:       Watch the AI battle itself.
    --ai-reasoning|-r:  Print information about what the AI is doing, and why, during the game.
    --square-mode|-s:   Enable harder rules: not only 4 of the same in a row, but also a square of 4 is considered a win.
    --help|-h:          Print this help screen.
"
        );
        return;
    }

    if args().any(|x| x == "--ai-wars" || x == "-a") {
        ai_wars();
        return;
    }

    let mut game = Game::new(Player::PlayerOne);

    if args().any(|x| x == "--ai-reasoning" || x == "-r") {
        game.ai_reasoning = true;
    }

    if args().any(|x| x == "--square-mode" || x == "-s") {
        game.field.square_mode = true;
    }

    play(game);
}

fn play(mut game: Game) {
    let mut buf = String::new();

    let mut ai = SimpleAi::new(Player::PlayerTwo);

    println!("Let the games begin!");

    loop {
        game.pp();
        if !game.running() {
            return;
        }

        if game.player() == Player::PlayerOne {
            if game.is_initial_move() {
                let next_piece = read_piece(&game);
                game.initial_move(next_piece).unwrap();
            } else {
                loop {
                    println!("Select x,y to put the piece to:");
                    buf.clear();
                    stdin().read_line(&mut buf).unwrap();
                    let pos = try_parse_pos(&buf);
                    if let Ok(pos) = pos {
                        let next_piece = read_piece(&game);
                        if game.do_move(pos, next_piece).is_ok() {
                            break;
                        }
                    }
                    println!("Illegal move! The x,y value must be an empty place on the field!");
                }
            }
            println!();
        } else {
            game = ai.play_iteratively(&mut game);
        }
    }
}

fn read_piece(game: &Game) -> Piece {
    let mut buf = String::with_capacity(16);
    let piece_id: usize = loop {
        println!(
            "\n{:?}, please chose your opponen's next piece (0-{}):",
            game.player(),
            game.remaining_pieces().len() - 1
        );
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let num = buf.trim().parse();
        if let Ok(num) = num {
            if num < game.remaining_pieces().len() {
                break num;
            }
        }
        #[cfg(debug_assertions)]
        println!("{} (str: {buf})", num.err().unwrap());
        println!("Illegal choice: {buf}, please pick the id of a remaining piece:");
        game.pp_remaining_pieces();
    };
    game.remaining_pieces()[piece_id]
}

#[allow(clippy::cast_precision_loss, clippy::cast_lossless)]
fn ai_wars() {
    const ITERS: usize = 100;

    let it = std::time::Instant::now();

    let mut ai_one_wins = 0;
    let mut ai_two_wins = 0;
    let mut turns = 0_u64;

    'outer: for _ in 0..ITERS {
        let mut game = Game::new(Player::PlayerOne);

        let mut ai_one = SimpleAi::new(Player::PlayerOne);
        let mut ai_two = SimpleAi::new(Player::PlayerTwo);

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
                turns += game.round() as u64;
                continue 'outer;
            }

            if game.player() == Player::PlayerOne {
                game = ai_one.play_iteratively(&mut game);
            } else {
                game = ai_two.play_iteratively(&mut game);
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
    let draws = ITERS - ai_one_wins - ai_two_wins;
    let draw_percentage = (draws as f64 / ITERS as f64) * 100.;

    println!("We had {draws} draws ({draw_percentage}%)");
}

#[cfg(test)]
mod test {
    use crate::{
        field::Field,
        piece::{Piece, Property},
    };

    #[test]
    fn test_check_field_for_win() {
        let test_light_tall: Piece =
            Piece::with_props_props(Property::Tall as u8 | Property::Light as u8);
        let test_dark_short: Piece = Piece::new();

        let mut field = Field::new();

        field.put((3, 0), test_light_tall).unwrap();
        field.put((2, 1), test_light_tall).unwrap();
        field.put((1, 2), test_dark_short).unwrap();

        assert!(!field.check_field_for_win());

        field.put((0, 3), test_light_tall).unwrap();

        field.pp();
    }
}
