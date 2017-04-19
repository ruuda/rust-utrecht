// Ccut -- Basic example for the Rust Utrecht meetup.
// Written in 2017 by Ruud van Asseldonk
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty. You should have
// received a copy of the CC0 Public Domain Dedication along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.

// This file implements a cut-like program that outputs a given (0-based) column
// of a file of comma-separated values. Exercises left to the reader:
//
//  * Allow writing to a file in addition to stdout.
//  * Allow reading from stdin in addition to a file.
//  * Allow the user to specify the separator.
//  * Allow the user to select a contiguous range of columns.
//  * Allow the user to select an arbitrary subset of columns.
//  * Speed up the program using the memchr crate.
//  * Allow using non-ASCII separators, assuming UTF-8 input.
//    The application should deal with code points spanning the buffer boundary.
//  * Allow using UTF-16 input.

use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::io;
use std::str::FromStr;

fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        println!("Usage:\n\n  ccut <column> <infile>");
        return
    }

    // These unwraps are safe, because three arguments are given. (The first one
    // is the name of the program.)
    args.next();
    let colstr = args.next().unwrap();
    let ifname = args.next().unwrap();

    let column = u32::from_str(&colstr)
        .expect("Column must be a non-negative integer.");

    let ifile = File::open(ifname).expect("Failed to open input file.");
    process_file(column, ifile).expect("Failed to process file.");
}

fn process_file(column: u32, mut input: File) -> io::Result<()> {
    // Take a lock on stdout, such that write calls don't need to lock while the
    // `handle` is in scope. This program is not multithreaded anyway.
    let stdout = io::stdout();
    let handle = stdout.lock();

    // Wrap the sink in a `BufWriter`, so we don't need to do a syscall for
    // every write operation, but only when the 4 KiB buffer is full. The 4 KiB
    // buffer is allocated on the heap.
    let mut output = BufWriter::with_capacity(4096, handle);

    // Allocate a 4 KiB input buffer on the stack.
    let mut buffer = [0u8; 4096];

    let mut current_column = 0;
    let mut should_echo = column == current_column;

    // Keep track of the first byte of the desired column.
    let mut from = 0;

    loop {
        let len = input.read(&mut buffer)?;

        // When `read()` returns 0, we have reached EOF.
        if len == 0 {
            break
        }

        for i in 0..len {
            match buffer[i] {
                b',' => {
                    if should_echo {
                        output.write(&buffer[from..i])?;
                    }

                    current_column += 1;
                    should_echo = column == current_column;
                    from = i + 1;
                }
                b'\n' => {
                    if should_echo {
                        output.write(&buffer[from..i])?;
                    }

                    current_column = 0;
                    should_echo = column == current_column;
                    from = i + 1;

                    write!(output, "\n")?;
                }
                _ => {}
            }
        }

        // We might have reached the end of the buffer inside a column that
        // should be printed.
        if should_echo {
            output.write(&buffer[from..len])?;
        }
        from = 0;
    }

    Ok(())
}
