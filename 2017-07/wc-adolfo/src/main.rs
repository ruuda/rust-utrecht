use std::error::Error;
use std::io::{self, BufRead};


fn main() {
    let wc = run().expect("Failed to run");
    println!("{}", wc);
}

fn run() -> Result<usize, Box<Error>> {
    // Lock stdin before we start reading
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut wc = 0;

    // Note: can you spot the performance pitfall of using stdin.lines()?
    for line in stdin.lines() {
        let line = line?;
        wc += line.split_whitespace().count();
    }

    Ok(wc)
}
