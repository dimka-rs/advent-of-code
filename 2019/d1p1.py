#!/usr/bin/env python3
import math

IN="./d1p1.in"

sum=0
with open(IN, 'r') as f:
    for l in f:
        sum += math.floor(int(l)/3) - 2

print(sum)
