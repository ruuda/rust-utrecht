extern crate regex;

use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let pattern = env::args().skip(1).next().expect("No pattern provided");
    let re = regex::Regex::new(&pattern).expect("Invalid regex");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(ln) = line {
            if re.is_match(&ln) {
                println!("{}", ln);
            }
        }
    }
}
