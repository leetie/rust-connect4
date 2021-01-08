use std::io;

#[allow(dead_code)]
fn main() {
    // initialize variable to track current player
    let mut current_player: u8 = 1;
    // initialize 7X5 board of "O"s as game board
    let mut board = vec![vec!["O"; 7]; 5];
    for row in board.iter() {
        // println!("row is {:?}", row);
        for cell in row.iter() {
            print!("  {}  ", cell);
        }
        print!("\n"); // add new line after every row
    }
    board[0][0] = "X";
    // TODO
    // variable to track player turn ✔️
    // function to get desired column to 'drop' piece from player ✔️
    // function to 'drop' piece into column if it is available
    // function to check win
    // function to change active player ✔️

    //     { #game loop stuff
    //          change_player(current_player);
    //     }
    fn change_player(_current_player: u8) -> u8 {
        if _current_player == 1 {
            2
        } else {
            1
        }
    }

    fn get_desired_choice() -> u8 {
        loop {
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
}
