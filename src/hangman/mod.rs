use std::io::{ self, Write };

static BAD_INPUT_MESSAGE: &'static str = "Bad input.  Must be a single letter a-z.";
static GUESS_MESSAGE: &'static str = "Enter your guess: ";

pub fn run() {
    let secret_word = generate_secret_word();
    let game_state = GameState::initialize(secret_word, 8);

    while !game_state.has_won() {
        println!("{}", game_state.report());
        print!("{}", GUESS_MESSAGE);
        io::stdout().flush().expect("Failed to flush");
        let guess: char = match get_guess() {
            Ok(guess) => guess,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };

    }

    //TODO validate and parse guess
}

fn get_guess() -> Result<char, &'static str> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Could not read line");
    parse_input(buffer)
}

fn parse_input(buffer: String) -> Result<char, &'static str> {
    if buffer.len() != 1 || ! buffer.chars().next().unwrap().is_ascii() {
        return Err(BAD_INPUT_MESSAGE)
    }

    Ok(buffer.chars().next().unwrap().to_ascii_lowercase())
}

fn generate_secret_word() -> String {
    String::from("grantypanty")
}

struct GameState {
    secret_word: String,
    guessed_letters: Vec<bool>,
    guesses_remaining: i8,
}

// TODO add trait for secret word to make it generic for testing
impl GameState {
    fn initialize(secret_word: String, num_guesses: i8) -> GameState {
        if secret_word.len() < 5 || secret_word.len() > 30 {
            panic!("Cannot initialize GameState: Secret word len out of bounds");
        }

        let guessed_letters = vec![false; secret_word.len()];

        GameState {
            secret_word: secret_word,
            guessed_letters: guessed_letters,
            guesses_remaining: num_guesses
        }
    }

    fn current_word(secret_word: &String, guessed_letters: &Vec<bool>) -> String {
        secret_word.chars().zip(guessed_letters.iter())
            .map(|zip| if *zip.1 { zip.0 } else { '_' } )
            .collect::<String>()
    }

    fn report(&self) -> String {
        let current_word_report = GameState::current_word(&self.secret_word, &self.guessed_letters);
        let guesses_remaining_report = self.guesses_remaining.to_string();
        format!("Current word: {}\nTries left: {}", current_word_report, guesses_remaining_report)
    }

    fn has_won(&self) -> bool {
        match self.guessed_letters.iter().position(|&x| ! x ) {
            Some(_x) => false,
            None => true
        }
    }

    fn match_letter(&self, letter: char) -> Vec<i8> {
        self.secret_word.chars().enumerate().fold(
            Vec::new(),
            |mut acc, (i, x)| if x == letter { acc.push(i as i8); acc } else { acc }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // pub trait GameStateTestExt {

    // }

    #[test]
    fn game_state_has_won() {
        let won_game = GameState {
            secret_word: String::new(),
            guesses_remaining: 0,
            guessed_letters: vec![true, true, true]
        };
        assert!(won_game.has_won());
    }

    #[test]
    fn game_state_has_not_won() {
        let won_game = GameState {
            secret_word: String::new(),
            guesses_remaining: 0,
            guessed_letters: vec![true, true, false]
        };
        assert!(! won_game.has_won());
    }

    #[test]
    fn game_state_has_won_empty() {
        let won_game = GameState {
            secret_word: String::new(),
            guesses_remaining: 0,
            guessed_letters: vec![]
        };
        assert!(won_game.has_won());
    }

    #[test]
    #[should_panic]
    fn game_state_secret_word_too_short() {
        GameState::initialize("yo".to_string(), 4);
    }

    #[test]
    fn game_state_setup() {
        let secret_word = "testing".to_string();
        let guesses_remaining = 5;
        let expected_game_state = GameState {
            secret_word: secret_word,
            guessed_letters: vec![false, false, false, false, false, false, false],
            guesses_remaining: guesses_remaining
        };

        let new_game_state = GameState::initialize("testing".to_string(), guesses_remaining);

        assert!(compare_game_state(new_game_state, expected_game_state));
    }

    #[test]
    fn matches_multiple() {
        let game_state = GameState::initialize("testing".to_string(), 5);
        assert_eq!(vec![0, 3], game_state.match_letter('t'));
    }

    #[test]
    fn matches_if_not_in() {
        let game_state = GameState::initialize("testing".to_string(), 5);
        let expected_matches: Vec<i8> = Vec::new();
        assert_eq!(expected_matches, game_state.match_letter('x'));
    }

    #[test]
    fn matches_single() {
        let game_state = GameState::initialize("testing".to_string(), 5);
        assert_eq!(vec![6], game_state.match_letter('g'));
    }

    #[test]
    fn current_word_multiple_matches() {
        let secret_word = "secretword".to_string();
        let guessed_letters = vec![true, false, false, true, false, false, false, true, false, true];
        let current_word = GameState::current_word(&secret_word, &guessed_letters);
        let expected_current_word = String::from("s__r___o_d");

        assert_eq!(current_word, expected_current_word);
    }

    #[test]
    fn current_word_no_matches() {
        let secret_word = "secret".to_string();
        let guessed_letters = vec![false, false, false, false, false];
        let current_word = GameState::current_word(&secret_word, &guessed_letters);
        let expected_current_word = String::from("_____");

        assert_eq!(current_word, expected_current_word);
    }

    #[test]
    fn current_word_single_matches() {
        let secret_word = "secret".to_string();
        let guessed_letters = vec![false, true, false, false, false];
        let current_word = GameState::current_word(&secret_word, &guessed_letters);
        let expected_current_word = String::from("_e___");

        assert_eq!(current_word, expected_current_word);
    }

    #[test]
    fn parse_valid_input() {
        let buffer = String::from("g");
        assert_eq!(parse_input(buffer), Ok("g".chars().next().unwrap()));
    }

    #[test]
    fn parse_empty_input() {
        let buffer = String::from("");
        assert_eq!(parse_input(buffer), Err(BAD_INPUT_MESSAGE));
    }

    #[test]
    fn parse_input_too_long() {
        let buffer = String::from("grrr");
        assert_eq!(parse_input(buffer), Err(BAD_INPUT_MESSAGE));
    }

    #[test] 
    fn parse_failes_on_non_ascii() {
        let buffer = String::from("†");
        assert_eq!(parse_input(buffer), Err(BAD_INPUT_MESSAGE));
    }

    #[test]
    fn parse_convert_uppercase_ascii_to_lowercase() {
        let buffer = String::from("G");
        assert_eq!(parse_input(buffer), Ok("g".chars().next().unwrap()));
    }

    fn compare_game_state(gs1: GameState, gs2: GameState) -> bool {
        gs1.secret_word == gs2.secret_word &&
        gs1.guesses_remaining == gs2.guesses_remaining &&
        gs1.guessed_letters == gs2.guessed_letters

        // TODO Can use EQ trait???
    }
}
