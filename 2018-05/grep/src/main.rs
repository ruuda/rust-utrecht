extern crate regex;

use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let pattern = env::args().skip(1).next().expect("No pattern provided");
    let re = regex::Regex::new(&pattern).expect("Invalid regex");
    let stdin = io::stdin();
    // ANSI escape codes for bold red, and reset.
    let make_red = "\x1b[31;1;1m";
    let reset = "\x1b[0m";
    for line in stdin.lock().lines() {
        if let Ok(ln) = line {
            let mut k = 0;
            let mut highlighted = ln.clone();
            // Iterate over all matches, and insert turn-red code before the
            // start of a match, insert a reset code after the match.
            for m in re.find_iter(&ln) {
                highlighted.insert_str(m.start() + k, make_red);
                k += make_red.len();
                highlighted.insert_str(m.end() + k, reset);
                k += reset.len();
            }
            if k > 0 {
                // Print the line if there was a match.
                println!("{}", highlighted);
            }
        }
    }
}
