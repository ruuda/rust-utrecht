extern crate regex;

use std::env;
use std::io;

fn main() {
    let pattern = env::args().skip(1).next().expect("No pattern provided");
    let re = regex::Regex::new(&pattern).expect("Invalid regex");
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        if let Ok(n) = stdin.read_line(&mut line) {
            if re.is_match(&line) {
                print!("{}", line);
            }
            if n == 0 {
                break
            }
        }
    }
}
