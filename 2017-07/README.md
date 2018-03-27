Rust Utrecht 2017-07 assignment
===============================

In this repository:

* `genwords`, which will put a ~1GiB text file in `./large`
  (just run `cargo run * --release`)
* `wc`, which is a reference implementation of `wc` in Rust (you can pipe the
  `large` file into this binary)

Note that the reference implementation focuses on clarity, not on efficency.
Here are some optimizations you could try:

* Avoid allocating a `String` upon reading each line
* Apply multithreading in a sensible way (maybe using the `rayon` crate?)
* Assume ASCII encoding (and fall back to UTF-8 if necessary)

Other possibilities:

* Allow reading from a file in addition to stdin
* Allow UTF-16 input, deal with Unicode spaces
