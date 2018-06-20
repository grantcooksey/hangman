pub enum UserGuess {
    Valid(char),
    Invalid,
}

pub fn get_guess() -> UserGuess {
    UserGuess::Invalid
}
