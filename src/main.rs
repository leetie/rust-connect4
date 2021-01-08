use std::io;
use std::{thread, time}; // debugging purposes
                         // TODO
                         // variable to track player turn ✔️
                         // function to get desired column to 'drop' piece from player ✔️
                         // function to 'drop' piece into column if it is available
                         // function to check win
                         // function to change active player ✔️
                         // game loop ✔️

#[allow(dead_code)]
fn main() {
    ////////////// VARIABLE INITIALIZATION //////////////
    let mut current_player: u8 = 1;
    let mut board = vec![vec!["O"; 7]; 5]; // 7X5

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
                // logic here to make sure num is in 0..6
                Ok(num) => break num,
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
    }

    fn print_board(board: &Vec<Vec<&str>>) {
        for row in board.iter() {
            for cell in row.iter() {
                print!("  {}  ", cell);
            }
            print!("\n"); // add new line after every row
        }
    }

    fn clear_screen() {
        std::process::Command::new("clear").status().unwrap();
    }

    fn process_choice(choice: u8, board: &mut Vec<Vec<&str>>, player: &mut u8) {
        println!("current player is: {}", player);
        println!("choice is: {}", choice);
        // check top row to see if desired space is occupied
        // if so, invalid move because column is filled
        // if not, check next row down recursively until !empty space is found
        // add piece in space board[i-1][j], where 'i' is row and 'j' is col
    }

    fn game_loop(board: &mut Vec<Vec<&str>>, current_player: &mut u8) {
        let mut running = false;
        clear_screen();
        welcome_message();
        loop {
            print_board(&board);
            process_choice(
                get_desired_choice(&current_player),
                board,
                current_player, // current_player
            );
            thread::sleep(time::Duration::from_secs(4));
            // check_win();
            clear_screen();
            change_player(current_player);
        }
    }

    ////////////// DRIVER SCRIPT //////////////
    game_loop(&mut board, &mut current_player);
}
