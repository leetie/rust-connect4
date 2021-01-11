mod game;
mod player;
// TODO
// variable to track player turn ✔️
// function to get desired column to 'drop' piece from player ✔️
// function to 'drop' piece into column if it is available ✔️
// add color (and spaghetti) ✔️
// function to check win
// function to change active player ✔️
// game loop ✔️

#[allow(dead_code)]
fn main() {
    pub use crate::game::*;

    ////////////// VARIABLE INITIALIZATION //////////////
    let mut board = game::board::new();
    let mut current_player: u8 = 1;

    // ////////////// DRIVER SCRIPT //////////////
    game::board::game_loop(&mut board, &mut current_player);
}
