use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const BUF_SIZE: i32 = 1024;

fn main() -> std::io::Result<()> {
    
    let f = File::open("./resources/test.tx").expect("Could not find the file!");
    let mut reader = BufReader::new(f);
    
    for line in reader.lines() {
        let line = line?;
        println!("line {}", line);
    }

    Ok(())
}
