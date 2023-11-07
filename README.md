# Quarto-rs

Your friendly Quarto game.

Install using `cargo install quarto` or clone this repo and `cargo run`

On NetBSD, a package is available from the official repositories.
To install it, simply run `pkgin install quarto`.

![Quarto In-Game Screenshot](quarto.png?raw=true "Quarto In-Game Screenshot")

The game is played on a 4x4 board with 16 pieces. Each piece has four distinct
characteristics: size (large/✋ or small/🤏), color (light/⬜ or dark/⬛),
shape (round/🟠 or square/🔶), and fill (filled/🔴 or hollow/⭕).
On your turn, you choose one of the 16 pieces and give it to your opponent.
Your opponent then places that piece on any empty space on the board.

The first player to create a row of four pieces with at least one matching
characteristic wins the game. Matching characteristics can be in any direction,
horizontally, vertically, or diagonally.
In the harder square mode, (-q), a square of 4 is also considered a win.

If all 16 pieces have been placed and there is no winner, the game is a tie.

```
Usage: {current_exe_name} <Options>

Options:
    --square-mode|-q:   Enable harder rules: not only 4 of the same in a row,
                        but also a square of 4 is considered a win.
    --base0|-0:         Starts to count at 0 instead of 1 (programmer style)
    --ai-reasoning|-r:  Print information about what the AI is doing, and why,
                        during the game.
    --ai-simulation|-a: Simulate a bunch of AI battles.
    --seed=<>|-s=<>:    Seed the AI RNG
    --pvp|-p            No AI, just humans (player vs player)
    --help|-h:          Print this help screen.
```

Good luck!


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

<br>
