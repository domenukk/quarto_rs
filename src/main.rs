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

use crate::{
    field::Field,
    piece::{Piece, Property},
};

mod ai;
mod field;
mod game;
mod piece;

fn main() {
    let test_light_tall: Piece =
        Piece::new_with_props(Property::Tall as u8 | Property::Light as u8);
    let test_dark_short: Piece = Piece::new();

    let mut field = Field::new();

    field.put((3, 0), test_light_tall).unwrap();
    field.put((2, 1), test_light_tall).unwrap();
    field.put((1, 2), test_dark_short).unwrap();

    assert!(!field.check_field_for_win());

    field.put((0, 3), test_light_tall).unwrap();

    field.pp()
}
