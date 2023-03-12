use rand::prelude::*;
use std::fs;

use colored::*;

#[derive(Debug, Clone, Copy)]
enum GuessCharStatus {
    Right,
    WrongPlace,
    Wrong,
}
#[derive(Debug, Clone, Copy)]
pub struct GuessChar {
    char: char,
    status: GuessCharStatus,
}

#[derive(Clone, Copy, Debug)]
pub struct GuessWord {
    word: [GuessChar; 5],
}

impl GuessWord {
    pub fn render(&self) {
        for guess in self.word {
            let char = guess.char.to_string();
            // word = word + char;
            let colred = {
                match guess.status {
                    GuessCharStatus::Right => char.green(),
                    GuessCharStatus::WrongPlace => char.yellow(),
                    GuessCharStatus::Wrong => char.red(),
                }
            };

            print!("{}", colred)
        }
        // println!("{}", word);
        println!("");
    }
}

#[derive(Debug)]
pub enum WordleGameStatus {
    OnGoing,
    Win,
    Lose,
}
#[derive(Debug)]
pub struct WordleGame {
    pub state: [Option<GuessWord>; 6],
    pub index: usize,
    pub correct: String,
    pub status: WordleGameStatus,
}

impl WordleGame {
    pub fn new() -> Self {
        let file = fs::read_to_string("words.txt").unwrap();
        let lines: Vec<&str> = file.split('\n').collect();
        let size = lines.len();
        let mut rng = rand::thread_rng();
        let y: usize = (rng.gen::<f64>() * (size as f64)).round() as usize; // generates a float between 0 and 1

        Self {
            state: [None; 6],
            correct: lines[y].trim().to_string(),
            index: 0,
            status: WordleGameStatus::OnGoing,
        }
    }

    pub fn render(&self) {
        for word in self.state {
            match word {
                Some(word) => word.render(),
                None => println!("_____"),
            };
        }
    }

    pub fn guess(&mut self, guess: &str) -> Result<(), String> {
        if guess.len() > 5 {
            return Err("Guess should be 5 char".to_string());
        }

        let mut word: [Option<GuessChar>; 5] = [None; 5];
        let mut right_places: u8 = 0;
        for (i, c) in guess.chars().collect::<Vec<char>>().iter().enumerate() {
            match self.correct.find(*c) {
                Some(index) => {
                    if index == i {
                        word[i] = Some(GuessChar {
                            char: *c,
                            status: (GuessCharStatus::Right),
                        });
                        right_places += 1;
                    } else {
                        word[i] = Some(GuessChar {
                            char: *c,
                            status: (GuessCharStatus::WrongPlace),
                        })
                    }
                }
                None => {
                    word[i] = Some(GuessChar {
                        char: *c,
                        status: (GuessCharStatus::Wrong),
                    })
                }
            }
        }

        self.state[self.index] = Some(GuessWord {
            word: word
                .map(|o| match o {
                    Some(a) => a,
                    _ => panic!("aa"),
                })
                .try_into()
                .unwrap(),
        });
        self.index += 1;

        if right_places == 5 {
            self.status = WordleGameStatus::Win;
        } else if self.index > 5 {
            self.status = WordleGameStatus::Lose;
        }
        Ok(())
    }
}
