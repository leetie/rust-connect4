use colored::*;
use std::io;
use std::{thread, time};
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
    ////////////// VARIABLE INITIALIZATION //////////////
    let mut current_player: u8 = 1;
    let mut board = vec![vec!["O".black().on_green(); 7]; 5]; // 7X5

    ////////////// USER FUNCTIONS //////////////
    fn change_player(_current_player: &mut u8) {
        if *_current_player == 1 {
            *_current_player = 2
        } else {
            *_current_player = 1
        }
    }

    fn get_desired_choice(player: &u8) -> u8 {
        loop {
            println!("Player {}, please input column 0-6", player);
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Please input");
            let _choice: u8 = match choice.trim().parse() {
                Ok(num) => match num {
                    0..=6 => break num,
                    _ => continue,
                },
                Err(_) => {
                    println!("Please input a number");
                    continue;
                }
            };
        }
    }

    ////////////// GAME BOARD FUNCTIONS //////////////
    fn welcome_message() {
        println!("Welcome to Connect 4!");
        println!("Player 1 will be {}.", "red".red());
        println!("Player 2 will be {}.", "blue".blue());
    }

    fn print_board(board: &Vec<Vec<ColoredString>>) {
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

    fn clear_screen() {
        std::process::Command::new("clear").status().unwrap();
    }

    fn process_choice(choice: u8, board: &mut Vec<Vec<ColoredString>>, player: &mut u8) {
        let choice: usize = choice as usize;
        // get new choice and rerun function if column is filled
        if board[0][choice] != "O".black().on_green() {
            println!("That column is filled!");
            thread::sleep(time::Duration::from_secs(2));
            process_choice(get_desired_choice(player), board, player);
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

    fn game_loop(board: &mut Vec<Vec<ColoredString>>, current_player: &mut u8) {
        let mut _running = false;
        clear_screen();
        welcome_message();
        loop {
            print_board(&board);
            process_choice(get_desired_choice(&current_player), board, current_player);
            // check_win();
            clear_screen();
            change_player(current_player);
        }
    }

    ////////////// DRIVER SCRIPT //////////////
    game_loop(&mut board, &mut current_player);
}
