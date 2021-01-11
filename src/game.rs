// Module for game functions

pub mod board {
  pub use crate::player::*;
  use colored::*;
  use std::{thread, time};
  pub fn new() -> Vec<Vec<ColoredString>> {
    vec![vec!["O".black().on_green(); 7]; 5]
  }

  pub fn print_board(board: &Vec<Vec<ColoredString>>) {
    for row in board.iter() {
      for cell in row.iter() {
        print!(
          "{}{}{}",
          " ".black().on_green(),
          cell,
          " ".black().on_green()
        );
      }
      print!("\n"); // add new line after every row
    }
    println!(" 0  1  2  3  4  5  6");
  }

  pub fn welcome_message() {
    println!("Welcome to Connect 4!");
    println!("Player 1 will be {}.", "red".red());
    println!("Player 2 will be {}.", "blue".blue());
  }

  pub fn clear_screen() {
    std::process::Command::new("clear").status().unwrap();
  }

  pub fn process_choice(choice: u8, board: &mut Vec<Vec<ColoredString>>, player: &mut u8) {
    let choice: usize = choice as usize;
    // get new choice and rerun function if column is filled
    if board[0][choice] != "O".black().on_green() {
      println!("That column is filled!");
      thread::sleep(time::Duration::from_secs(2));
      // fix this
      process_choice(player::get_desired_choice(player), board, player);
      return;
    }
    for row in 0..board.len() {
      // if on last row && empty, place piece
      if row == 4 && board[row][choice] == "O".black().on_green() {
        match player {
          1 => board[row][choice] = "X".red(),
          2 => board[row][choice] = "X".blue(),
          _ => (),
        }
        return;
      } else if board[row][choice] != "O".black().on_green() {
        // if the column isnt filled, place piece
        // 1 row above existing piece
        match player {
          1 => board[row - 1][choice] = "X".red(),
          2 => board[row - 1][choice] = "X".blue(),
          _ => (),
        }
        return;
      }
    }
  }

  pub fn game_loop(board: &mut Vec<Vec<ColoredString>>, current_player: &mut u8) {
    let mut _running = false;
    clear_screen();
    welcome_message();
    loop {
      print_board(&board);
      process_choice(
        player::get_desired_choice(&current_player),
        board,
        current_player,
      );
      // check_win();
      clear_screen();
      player::change_player(current_player);
    }
  }
}
