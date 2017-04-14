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
use std::io::{Read, Write};
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

fn process_file<I: Read, O: Write>(input: I, output: O) -> io::Result<()> {
    Ok(())
}
