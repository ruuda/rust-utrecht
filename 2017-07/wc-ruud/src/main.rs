use std::env::args;

extern crate filebuffer;
extern crate crossbeam;

fn main() {
    let fname = args().skip(1).next().unwrap();
    let file = filebuffer::FileBuffer::open(&fname).unwrap();
    let count = count_words_parallel(&file);

    // Mirror the output of `wc -w`.
    println!("{} {}", count, fname);
}

fn count_words_parallel(bytes: &[u8]) -> i64 {
    let (fst, snd) = bytes.split_at(bytes.len() / 2);
    let (b0, b1) = fst.split_at(fst.len() / 2);
    let (b2, b3) = snd.split_at(snd.len() / 2);

    crossbeam::scope(|scope| {
        let t1 = scope.spawn(|| count_words(b1));
        let t2 = scope.spawn(|| count_words(b2));
        let t3 = scope.spawn(|| count_words(b3));

        // Count words in the four slices individually.
        let (__, c0, w0) = count_words(b0);
        let (s1, c1, w1) = t1.join();
        let (s2, c2, w2) = t2.join();
        let (s3, c3, w3) = t3.join();

        // Account for words that end ad a slice boundary.
        let mut extra = 0;
        if w0 && !s1 { extra += 1; }
        if w1 && !s2 { extra += 1; }
        if w2 && !s3 { extra += 1; }
        if w3 { extra += 1; }

        (c0 + c1) + (c2 + c3) + extra
    })
}

fn count_words(bytes: &[u8]) -> (bool, i64, bool) {
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

    let start_in_word = match bytes[0] {
        b'\n' | b'\r' | b'\t' | b' ' => false,
        _ => true,
    };

    (start_in_word, count, in_word)
}
