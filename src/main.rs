use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const BUF_SIZE: i32 = 1024;

fn main() -> std::io::Result<()> {
    
    let f = File::open("./resources/test.txt").expect("Could not find the file!");
    let reader = BufReader::new(f);
    
    let buf_counter = 0;
    let mut buf = String::new();
    for line in reader.lines() {
        if (buf_counter <= BUF_SIZE) {
            
        } else {

        }
        let line = line?;
        println!("line {}", line);
    }

    Ok(())
}
