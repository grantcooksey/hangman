use std::io::{ self, Write };

static BAD_INPUT_MESSAGE: &'static str = "Bad input.  Must be a single letter a-z.";
static GUESS_MESSAGE: &'static str = "Enter your guess: ";

pub fn run() {
    let secret_word = generate_secret_word();
    let mut game_state = GameState::initialize(secret_word, 8);

    clear_screen();

    while ! game_state.has_won() && game_state.guesses_remaining > 0 {
        // Print the state of the hangman game
        println!("{}", game_state.report());
        print!("{}", GUESS_MESSAGE);
        io::stdout().flush().expect("Failed to flush");

        let guess: char = match get_guess() {
            Ok(guess) => guess,
            Err(e) => {
                clear_screen();
                println!("{}", e);
                continue;
            }
        };
        game_state = game_state.update(guess);

        clear_screen();
    }

    if game_state.has_won() {
        println!("Congrats, you won! The man didn't hang!");
    } else {
        println!("Hangman died :( Better luck next time!");
    }
}

fn clear_screen() {
    // erase text and move cursor to the home position, see VT100 terminal control
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);
}

fn get_guess() -> Result<char, &'static str> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Could not read line");
    parse_input(buffer)
}

fn parse_input(buffer: String) -> Result<char, &'static str> {
    let trimmed: &str = buffer.trim_right();
    if trimmed.len() != 1 || ! trimmed.chars().next().unwrap().is_ascii() {
        return Err(BAD_INPUT_MESSAGE)
    }

    Ok(trimmed.chars().next().unwrap().to_ascii_lowercase())
}

fn generate_secret_word() -> String {
    String::from("grantypanty")
}

#[derive(PartialEq, Debug)]
struct GameState {
    secret_word: String,
    matched_letters: Vec<bool>,
    guesses_remaining: i8,
    guessed_letters: Vec<char>,
}

impl GameState {
    fn initialize(secret_word: String, num_guesses: i8) -> GameState {
        if secret_word.len() < 5 || secret_word.len() > 30 {
            panic!("Cannot initialize GameState: Secret word len out of bounds");
        }

        let matched_letters = vec![false; secret_word.len()];

        GameState {
            secret_word: secret_word,
            matched_letters: matched_letters,
            guesses_remaining: num_guesses,
            guessed_letters: Vec::new()
        }
    }

    fn update(&self, guess: char) -> GameState {
        let guessed_letters = self.build_guessed_letters(&guess);
        let matched_letters = GameState::join(&self.matched_letters, self.match_letter(guess));
        
        GameState {
            secret_word: self.secret_word.clone(),
            matched_letters: matched_letters,
            guesses_remaining: self.guesses_remaining - 1,
            guessed_letters: guessed_letters
        }
    }

    fn build_guessed_letters(&self, guess: &char) -> Vec<char> {
        let mut guessed_letters = self.guessed_letters.clone();
        if ! self.guessed_letters.contains(guess) {
            guessed_letters.push(*guess);
        }

        guessed_letters
    }

    fn current_word(secret_word: &String, matched_letters: &Vec<bool>) -> String {
        secret_word.chars().zip(matched_letters.iter())
            .map(|zip| if *zip.1 { zip.0 } else { '_' } )
            .collect::<String>()
    }

    fn report(&self) -> String {
        let current_word_report = GameState::current_word(&self.secret_word, &self.matched_letters);
        let guesses_remaining_report = self.guesses_remaining.to_string();
        let guessed_letters = self.guessed_letters.iter().collect::<String>().replace("", " ");
        
        format!(
            "Current word: {}\nTries left: {}\nGuessed letters:{}", 
            current_word_report, 
            guesses_remaining_report, 
            guessed_letters
        )
    }

    fn has_won(&self) -> bool {
        match self.matched_letters.iter().position(|&x| ! x ) {
            Some(_x) => false,
            None => true
        }
    }

    fn match_letter(&self, letter: char) -> Vec<bool> {
        self.secret_word.chars().fold(
            Vec::new(),
            |mut acc, x| { 
                if x == letter { 
                    acc.push(true);
                } 
                else { 
                    acc.push(false) 
                } 
                
                acc 
            }
        )
    }

    fn join(original_matches: &Vec<bool>, new_matches: Vec<bool>) -> Vec<bool> {
        original_matches.iter().zip(new_matches.iter()).fold(
            Vec::new(),
            |mut acc, (&original, &new)| {
                if original || new {
                    acc.push(true);
                } else {
                    acc.push(false);
                }

                acc
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_state_has_won() {
        let won_game = GameState {
            secret_word: String::new(),
            guesses_remaining: 0,
            matched_letters: vec![true, true, true],
            guessed_letters: Vec::new()
        };
        assert!(won_game.has_won());
    }

    #[test]
    fn game_state_has_not_won() {
        let won_game = GameState {
            secret_word: String::new(),
            guesses_remaining: 0,
            matched_letters: vec![true, true, false],
            guessed_letters: Vec::new()
        };
        assert!(! won_game.has_won());
    }

    #[test]
    fn game_state_has_won_empty() {
        let won_game = GameState {
            secret_word: String::new(),
            guesses_remaining: 0,
            matched_letters: vec![],
            guessed_letters: Vec::new()
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
            matched_letters: vec![false, false, false, false, false, false, false],
            guesses_remaining: guesses_remaining,
            guessed_letters: Vec::new()
        };

        let new_game_state = GameState::initialize("testing".to_string(), guesses_remaining);

        assert_eq!(new_game_state, expected_game_state);
    }

    #[test]
    fn update_valid_guess() {
        let state = GameState {
            secret_word: "testss".to_string(),
            guesses_remaining: 5,
            matched_letters: vec![false, true, false, false, false, false],
            guessed_letters: vec!['g', 'r']
        };

        let guess = 't';

        let expected_state = GameState {
            secret_word: "testss".to_string(),
            guesses_remaining: 4,
            matched_letters: vec![true, true, false, true, false, false],
            guessed_letters: vec!['g', 'r', 't']
        };

        assert_eq!(state.update(guess), expected_state);
    }

    #[test]
    fn update_invalid_guess() {
        let state = GameState {
            secret_word: "testss".to_string(),
            guesses_remaining: 3,
            matched_letters: vec![false, true, false, false, false, false],
            guessed_letters: vec!['g', 'r']
        };

        let guess = 'p';

        let expected_state = GameState {
            secret_word: "testss".to_string(),
            guesses_remaining: 2,
            matched_letters: vec![false, true, false, false, false, false],
            guessed_letters: vec!['g', 'r', guess]
        };

        assert_eq!(state.update(guess), expected_state);
    }

    #[test]
    fn build_guessed_letters_no_match() {
        let state = GameState {
            secret_word: "testss".to_string(),
            guesses_remaining: 0,
            matched_letters: vec![true, true, false, false, false],
            guessed_letters: vec!['g', 'r']
        };

        let expected_guessed_letters = vec!['g', 'r', 'p'];

        assert_eq!(state.build_guessed_letters(&'p'), expected_guessed_letters);
    }

    #[test]
    fn build_guessed_letters_duplicate_match() {
        let state = GameState {
            secret_word: "testss".to_string(),
            guesses_remaining: 0,
            matched_letters: vec![true, true, false, false, false],
            guessed_letters: vec!['g', 't']
        };

        let expected_guessed_letters = vec!['g', 't'];
        
        assert_eq!(state.build_guessed_letters(&'t'), expected_guessed_letters);
    }

    #[test]
    fn join_no_new_matches() {  
        let original_matches = vec![true, false, true];
        let new_matches = vec![false, false, false];
        let expected_matches = vec![true, false, true];

        assert_eq!(GameState::join(&original_matches, new_matches), expected_matches);
    }
// 
    #[test]
    fn join_some_matches() {
        let original_matches = vec![true, false, true];
        let new_matches = vec![false, true, false];
        let expected_matches = vec![true, true, true];

        assert_eq!(GameState::join(&original_matches, new_matches), expected_matches);
    }

    #[test]
    fn matches_multiple() {
        let game_state = GameState::initialize("testing".to_string(), 5);
        assert_eq!(vec![true, false, false, true, false, false, false], game_state.match_letter('t'));
    }

    #[test]
    fn matches_if_not_in() {
        let game_state = GameState::initialize("testing".to_string(), 5);
        let expected_matches: Vec<bool> = vec![false; 7];
        assert_eq!(expected_matches, game_state.match_letter('x'));
    }

    #[test]
    fn matches_single() {
        let game_state = GameState::initialize("testing".to_string(), 5);
        assert_eq!(vec![false, false, false, false, false, false, true], game_state.match_letter('g'));
    }

    #[test]
    fn current_word_multiple_matches() {
        let secret_word = "secretword".to_string();
        let matched_letters = vec![true, false, false, true, false, false, false, true, false, true];
        let current_word = GameState::current_word(&secret_word, &matched_letters);
        let expected_current_word = String::from("s__r___o_d");

        assert_eq!(current_word, expected_current_word);
    }

    #[test]
    fn current_word_no_matches() {
        let secret_word = "secret".to_string();
        let matched_letters = vec![false, false, false, false, false];
        let current_word = GameState::current_word(&secret_word, &matched_letters);
        let expected_current_word = String::from("_____");

        assert_eq!(current_word, expected_current_word);
    }

    #[test]
    fn current_word_single_matches() {
        let secret_word = "secret".to_string();
        let matched_letters = vec![false, true, false, false, false];
        let current_word = GameState::current_word(&secret_word, &matched_letters);
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
}
