extern crate rand;

use std::io::*;
use std::fs::File;
use self::rand::{thread_rng, Rng};

pub fn generate_secret_word() -> Result<String> {
    let file = File::open(String::from("./resources/words.txt"))?;
    let reader = BufReader::new(file);

    let mut secret_word = Ok(String::from("fake"));
    let mut counter = 1.0;
    let mut rng = thread_rng();
    for line in reader.lines() {
        if counter == 1.0 || rng.gen_bool(1.0 / counter) {
            secret_word = line;
        }
        counter += 1.0;
    }

    secret_word
}
