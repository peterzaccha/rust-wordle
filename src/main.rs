use std::io;
mod wordle;

use colored::*;
fn main() {
    let mut game = wordle::WordleGame::new();

    let stdin = io::stdin();

    for _ in 0..6 {
        match game.status {
            wordle::WordleGameStatus::OnGoing => {
                let mut input = String::from("");
                println!("Enter your guess:");
                stdin.read_line(&mut input).unwrap();
                input = input.trim().to_string();
                game.guess(&input)
                    .unwrap_or_else(|e| println!("{}", e.red()));
                game.render();
            }
            wordle::WordleGameStatus::Win => {
                println!("{}", "You Won".green());
            }
            wordle::WordleGameStatus::Lose => {
                println!("{}", "Good luck next time".green());
            }
        }
    }
}
