// Csvtojson -- Basic example for the Rust Utrecht meetup.
// Written in 2017 by Ruud van Asseldonk
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty. You should have
// received a copy of the CC0 Public Domain Dedication along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.

use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::io;

fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        println!("Usage:\n\n  csvtojson <infile> <outfile>");
        return
    }

    // These unwraps are safe, because three arguments are given.
    args.next();
    let ifname = args.next().unwrap();
    let ofname = args.next().unwrap();

    let ifile = File::open(ifname).expect("Failed to open input file.");
    let ofile = File::create(ofname).expect("Failed to open output file.");
    process_file(ifile, ofile).expect("Failed to process file.");
}

fn process_file(mut input: File, raw_output: File) -> io::Result<()> {
    // Wrap the file in a `BufWriter`, so we don't need to do a syscall for
    // every write operation, but only when the 4 KiB buffer is full.
    let mut output = BufWriter::with_capacity(4096, raw_output);

    // Fill a 4 KiB input buffer once, and locate the first newline.
    let mut buffer = [0u8; 4096];
    let mut len = input.read(&mut buffer)?;
    let first_newline = find_byte(&buffer[..len], b'\n')
        .expect("Header row must fit in 4 KiB.");

    // Store the column headers, because we need to output them every time.
    let column_headers = parse_header(&buffer[..first_newline]);

    // Write the start of the json array.
    write!(output, "[")?;

    // The index of the current header.
    let mut h = 0;
    let mut needs_row_sep = true;
    let mut needs_header = true;
    let mut row_sep = "";

    loop {
        for byte in &buffer[first_newline + 1..len] {
            match *byte {
                b'\n' => {
                    // Note: the double } escapes the format string.
                    write!(output, "\"}}")?;
                    // We should only write row separators after the first row;
                    // initially it was empty.
                    row_sep = ",";
                    needs_row_sep = true;
                    needs_header = true;
                    h = 0;
                }
                b',' => {
                    write!(output, "\",")?;
                    h += 1;
                    if h >= column_headers.len() {
                        panic!("Data row contains more columns than header row.");
                    }
                    // We should only write row separators after the first row;
                    // initially it was empty.
                    row_sep = ",";
                    needs_header = true;
                }
                c => {
                    if needs_row_sep {
                        // Note: the double { escapes the format string.
                        write!(output, "{}{{", row_sep)?;
                        needs_row_sep = false;
                    }
                    if needs_header {
                        write!(output, "\"{}\":\"", column_headers[h])?;
                        needs_header = false;
                    }
                    // Do not use the `write!` macro to write the byte, as that
                    // would write it as digits. We could do proper decoding and
                    // work with strings, at the cost of validating UTF-8.
                    // Instead, just output the byte. Write a slice of length 1.
                    output.write(&[c])?;
                }

            }
        }

        // Refill the buffer with new data.
        len = input.read(&mut buffer)?;

        // When the number of bytes read is 0, we reached EOF.
        if len == 0 {
            break
        }
    }

    // If the csv file did not end in a newline, we must still close the last
    // field and the object.
    if !needs_row_sep {
        write!(output, "\"}}")?;
    }

    // CLose the json array.
    write!(output, "]")?;

    Ok(())
}

fn find_byte(buffer: &[u8], byte: u8) -> Option<usize> {
    for i in 0..buffer.len() {
        if buffer[i] == byte {
            return Some(i)
        }
    }
    None
}

fn parse_header(header_row: &[u8]) -> Vec<String> {
    header_row
        .split(|byte_ptr| *byte_ptr == b',')
        .map(|header| String::from_utf8_lossy(header).into_owned())
        .collect()
}
