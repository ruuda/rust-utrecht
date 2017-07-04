extern crate rand;

use std::io::{self, BufWriter, Write};
use std::fs::File;
use rand::{Rng, ThreadRng};

fn main() {
    let mut rng = rand::thread_rng();

    // Open a file called large
    let mut file = BufWriter::new(File::create("large").expect("Failed to create file"));

    // Allocate a buffer that we will reuse when generating the words
    let mut buf = String::new();

    // Write ten million lines of text
    for _ in 0..10_000_000 {
        // Write between 5 and 15 words per line
        for _ in 0..rng.gen_range(5, 15) {
            write_random_word(&mut file, &mut rng, &mut buf).expect("Write failed");
        }
    }
}

fn write_random_word<W: Write>(w: &mut W, rng: &mut ThreadRng, buf: &mut String) -> io::Result<()> {
    buf.clear();

    // Write between 5 and 15 chars per word
    let word_len = rng.gen_range(5, 15);
    for c in rng.gen_ascii_chars().take(word_len) {
        buf.push(c);
    }

    write!(w, "{} ", buf)
}
