mod input;

pub fn run() {
    let game_state = GameState::initialize();

    while !game_state.has_won() {
        println!("{}", game_state.report());
        let guess = match get_guess() {
            input::UserGuess::Valid(c) => c,
            input::UserGuess::Invalid => {
                println!("Bad input! Try again.");
                continue;
            }
        };        
    }

    //TODO validate and parse guess
    //TODO update Game state

    //TODO wrap into loop
}

fn get_guess() -> input::UserGuess {
    input::UserGuess::Invalid
}

struct GameState {
    secret_word: String,
    guessed_letters: Vec<bool>,
    guesses_remaining: i8,
}

impl GameState {
    pub fn initialize() -> GameState {
        let secret_word: String = GameState::generate_secret_word();
        let guessed_letters = vec![false; secret_word.len()];
        
        GameState {
            secret_word: secret_word,
            guessed_letters: guessed_letters,
            guesses_remaining: 5
        }
    }

    pub fn report(&self) -> &'static str {
        //TODO  Game state report
        "Report"
    }

    pub fn has_won(&self) -> bool {
        match self.guessed_letters.iter().position(|&x| ! x ) {
            Some(_x) => false,
            None => true   
        }
    }

    fn generate_secret_word() -> String {
        String::from("grant")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_state_has_won() {

    }
}
