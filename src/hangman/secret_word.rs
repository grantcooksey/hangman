extern crate rand;

use std::io::{BufReader, BufRead};
use std::fs::File;
use self::rand::{thread_rng, Rng};
use super::hangman_error::HangmanError;

pub fn generate_secret_word() -> Result<String, HangmanError> {
    let file = File::open(String::from("./resources/words.txt"))?;
    let reader = BufReader::new(file);

    let mut secret_word: String = String::from("fake");
    let mut counter = 1.0;
    let mut rng = thread_rng();
    for line in reader.lines() {
        if counter == 1.0 || rng.gen_bool(1.0 / counter) {
            secret_word = line?;
        }
        counter += 1.0;
    }

    if secret_word.len() > 30 {
        return Err(HangmanError::SecretWordLen)
    }

    Ok(secret_word)
}
