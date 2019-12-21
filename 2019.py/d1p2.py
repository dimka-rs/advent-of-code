#!/usr/bin/env python3
import math

IN="./d1p1.in"

def get_fuel(mass):
    return math.floor(int(mass)/3) - 2

sum = 0
s = 0
with open(IN, 'r') as f:
    for l in f:
        s += 1
        mass = int(l)
        fuel = get_fuel(mass)
        add_fuel = get_fuel(fuel)
        while (add_fuel > 0):
            fuel += add_fuel
            add_fuel = get_fuel(add_fuel)

        sum += fuel

print(sum)
