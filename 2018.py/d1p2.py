#!/usr/bin/env python3

IN = "./d1p1.in"

sum = 0
freqs = dict()

def parse(file):
    global sum
    global freqs
    with open(file, 'r') as f:
        for l in f:
            sum += int(l)
            #print(sum)
            k = str(sum)

            if k in freqs:
                print(k)
                exit(0)
            else:
                freqs[k] = 1





cnt = 0
while True:
    cnt += 1
    print("Pass ", cnt)
    parse(IN)
