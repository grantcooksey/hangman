use std::io;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub enum HangmanError {
    SecretWordLen,
    SecretWordFile, 
    BadInput,
    GeneralError(io::ErrorKind)
}

impl From<io::Error> for HangmanError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => HangmanError::SecretWordFile,
            e @ _ => HangmanError::GeneralError(e),
        }
    }
}

impl Display for HangmanError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            HangmanError::SecretWordLen => f.write_str("Cannot initialize GameState: Secret word len out of bounds"),
            HangmanError::SecretWordFile => f.write_str("Cannot find the secret word file"),
            HangmanError::BadInput => f.write_str("Bad input.  Must be a single letter a-z."),
            HangmanError::GeneralError(e) => {
                let error = io::Error::from(e);
                f.write_str(&format!("{}", error))
            }
        }
    }
}
