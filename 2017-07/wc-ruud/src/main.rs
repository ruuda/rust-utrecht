use std::env::args;

extern crate filebuffer;

fn main() {
    let fname = args().skip(1).next().unwrap();
    let file = filebuffer::FileBuffer::open(&fname).unwrap();
    let count = count_words(&file);

    // Mirror the output of `wc -w`.
    println!("{} {}", count, fname);
}

fn count_words(bytes: &[u8]) -> u64 {
    let mut in_word = false;
    let mut count = 0;
    for &b in bytes {
        if in_word {
            match b {
                b'\n' | b'\r' | b'\t' | b' ' => {
                    count += 1;
                    in_word = false;
                }
                _ => {}
            }
        } else {
            match b {
                b'\n' | b'\r' | b'\t' | b' ' => {}
                _ => { in_word = true; }
            }
        }
    }

    // Count the final word if we end in any.
    if in_word {
        count += 1;
    }

    count
}
