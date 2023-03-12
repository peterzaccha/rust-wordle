use rand::prelude::*;

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
    pub word: [GuessChar; 5],
}

impl GuessWord {
    pub fn render(&self) {
        for guess in self.word {
            let char = guess.char.to_string();
            let colred = {
                match guess.status {
                    GuessCharStatus::Right => char.on_green(),
                    GuessCharStatus::WrongPlace => char.on_yellow(),
                    GuessCharStatus::Wrong => char.on_red(),
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
    Lose(String),
}

static WORDS: &str = include_str!("../words.txt");
#[derive(Debug)]
pub struct WordleGame {
    pub state: [Option<GuessWord>; 6],
    pub index: usize,
    pub correct: String,
    pub status: WordleGameStatus,
    pub valid_words: Vec<&'static str>,
}

impl WordleGame {
    pub fn new() -> Self {
        let lines: Vec<&str> = WORDS.split('\n').collect();
        let size = lines.len();
        let mut rng = rand::thread_rng();
        let y: usize = (rng.gen::<f64>() * (size as f64)).round() as usize; // generates a float between 0 and 1

        Self {
            state: [None; 6],
            correct: lines[y].trim().to_string(),
            index: 0,
            status: WordleGameStatus::OnGoing,
            valid_words: lines,
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

    pub fn guess(&mut self, guess: &str) -> Result<GuessWord, String> {
        if guess.len() != 5 {
            return Err("Guess should be 5 char".to_string());
        }
        if !self.valid_words.contains(&guess) {
            return Err("Word is not allowed".to_string());
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
            self.status = WordleGameStatus::Lose(self.correct.clone());
        }
        Ok(self.state[self.index - 1].unwrap())
    }
}
