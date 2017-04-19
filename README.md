# Rust Utrecht 2017-04 assignment

In this repository:

 * `gencsv.py`, which will put a 2.1 GiB csv file in `/tmp/large.csv`.
 * `src/main.rs`, a Rust implementation of a streaming cut-like program.
 * `ccut.py`, a Python implementation that produces the same output.

Compare the wall clock elapsed time and max resident set size of both programs:

    ./gencsv.py
    cargo build --release
    /bin/time -v target/release/ccut 3 /tmp/large.csv > /dev/null
    /bin/time -v ./ccut.py 3 /tmp/large.csv > /dev/null

Note that this uses GNU Time in `/bin/time`, not the shell builtin.
