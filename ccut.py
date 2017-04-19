#!/bin/env python3

"""
A very simple Python implementation that is supposed to yield the same output as
the Rust program. But beware of the differences:

 * What happens when the wrong number of arguments is provided on the command
   line?
 * What happens when the column is not an integer? When it is negative?
 * What happens when a line contains less columns than requested?
 * What if the input file does not end in a newline?
 * How much memory does it consume? What if the input file contains a long line?

"""

import sys

column = int(sys.argv[1])
ifname = sys.argv[2]

with open(ifname, 'r') as f:
    for line in f:
        print(line.rstrip('\n').split(',')[column])
