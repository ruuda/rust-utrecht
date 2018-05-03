extern crate regex;

use std::env;
use std::io;

fn main() {
    let pattern = env::args().skip(1).next().unwrap();
    let mut stdin = io::stdin();
    let mut line = String::new();
    loop {
        if let Ok(n) = stdin.read_line(&mut line) {
            print!("{}", line);
            if n == 0 {
                break
            }
            line.clear();
        }
    }
}
