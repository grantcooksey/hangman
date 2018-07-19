mod hangman;

fn main() {
    println!("Welcome to hangman!");
    match hangman::run() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Uh ok, something went wrong... {}", e)
    };
}
