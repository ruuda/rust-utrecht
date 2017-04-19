#!/bin/env python3

with open('/tmp/large.csv', 'w') as f:
    f.write('integer,divisible_by_3,divisible_by_7,comment\n')
    for i in range(0, 1024 * 1024):
        div3 = 'yes' if i % 3 == 0 else 'no'
        div7 = 'yes' if i % 7 == 0 else 'no'
        # Occasionally make a line with a very long comment. This makes
        # line-by-line streaming infeasible, because lines can be very long.
        comment = ('very long comment' * 8192) if i % 239 == 39 else ''
        f.write('{},{},{},{}\n'.format(i, div3, div7, comment))
