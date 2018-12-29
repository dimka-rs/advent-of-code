#!/usr/bin/env python3

IN="./d1p1.in"

sum=0
with open(IN, 'r') as f:
    for l in f:
        sum += int(l)

print(sum)
