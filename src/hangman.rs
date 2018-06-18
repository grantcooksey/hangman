use std::io;

pub fn run() {
    let secret_word: String = generate_secret_word();
    let guesses = Vec::with_capacity(secret_word.len());

    let game_state = GameState {
        secret_word: secret_word,
        guesses: guesses
    };

    println!("{}", game_state.report());

    let guess = get_guess();

    //TODO validate and parse guess
    //TODO update Game state

    //TODO wrap into loop
}

fn generate_secret_word() -> String {
    String::from("grant")
}

fn get_guess() -> io::Result<i8> {
    //TODO IO for guessing
    Ok(5)
}

struct GameState {
    secret_word: String,
    guesses: Vec<bool>,
}

impl GameState {
    pub fn report(&self) -> &'static str {
        //TODO  Game state report
        "Report"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
