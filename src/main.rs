use std::io;
mod solver;
mod wordle;

use colored::*;
fn main() {
    let mut game = wordle::WordleGame::new();

    let stdin = io::stdin();

    loop {
        match game.status {
            wordle::WordleGameStatus::OnGoing => {
                let mut input = String::from("");
                println!("Enter your guess:");
                stdin.read_line(&mut input).unwrap();
                input = input.trim().to_string();
                match game.guess(&input) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e.red()),
                }

                game.render();
            }
            wordle::WordleGameStatus::Win => {
                println!("{}", "You Won".green());
                break;
            }
            wordle::WordleGameStatus::Lose(correct) => {
                println!(
                    "{}, The Word was: {}",
                    "Good luck next time".blue(),
                    correct.on_green()
                );
                break;
            }
        }
    }
}
