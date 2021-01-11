// Module for player functions

pub mod player {
  use std::io;
  pub fn get_desired_choice(player: &u8) -> u8 {
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

  pub fn change_player(_current_player: &mut u8) {
    if *_current_player == 1 {
      *_current_player = 2
    } else {
      *_current_player = 1
    }
  }
}
