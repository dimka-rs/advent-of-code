#!/usr/bin/env python3
import math

INTEST="./d1.in.test"
IN="./d1.in"


def d1p1(input_file):
    sum=0
    with open(input_file, 'r') as f:
        for l in f:
            first = None
            last = None
            for c in l:
                if c.isdigit():
                    if first == None:
                        # it is a first digit, so assign both vars
                        first = c
                        last = c
                    else:
                        # not a first digit, only assign last
                        last = c
            # Now we have two digits, make number and add to sum
            sum += int(first)*10 + int(last)

    return sum

print(d1p1(INTEST))

print(d1p1(IN))
