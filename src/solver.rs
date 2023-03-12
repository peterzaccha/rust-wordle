use crate::wordle::{WordleGame, WordleGameStatus};
use rand::prelude::*;

pub struct WordleSolver<'a> {
    game: &'a mut WordleGame,
    valid_words: Vec<&'static str>,
}

impl<'a> WordleSolver<'a> {
    pub fn new(game: &'a mut WordleGame) -> Self {
        Self {
            game,
            valid_words: vec![],
        }
    }

    fn generate_guess(&self) -> String {
        let size = &self.valid_words.len();
        let mut rng = rand::thread_rng();
        let y: usize = (rng.gen::<f64>() * (*size as f64)).round() as usize; // generates a float between 0 and 1

        self.valid_words[y].to_string()
    }

    pub fn solve(&mut self) -> Result<String, ()> {
        self.valid_words = self.game.valid_words.clone();

        for _ in 0..6 {
            let guess = self.generate_guess();
            let guess_result = self.game.guess(&guess).unwrap();

            if let WordleGameStatus::Win = self.game.status {
                return Ok(guess);
            }

            let filtered: Vec<&str> = vec![];

            for guess_char in guess_result.word {}

            for i in &self.valid_words {}
        }
        Ok("".to_string())
    }
}
