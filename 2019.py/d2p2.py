#!/usr/bin/env python3
import math

IN="./d2p1.in"
MAGIC=19690720

def oper(op, arg1, arg2):
    if op == '+':
        return arg1 + arg2
    elif op == '*':
        return arg1 * arg2

    print("wrong op: %c" % op)


def got_magic(pc, arg1, arg2):
    print("got %d at step: %d" % (MAGIC, pc))
    print("arg1: %d" % arg1)
    print("arg2: %d" % arg2)
    print("100 * verb + noun = % d" % (100 * arg1 + arg2))
    exit()


def run(cmd):
    pc = 0
    while True:
        op = '!'
        if cmd[pc] == 1:
            op = '+'
        elif cmd[pc] == 2:
            op = '*'
        elif cmd[pc] == 99:
            if cmd[0] == MAGIC:
                got_magic(pc, cmd[1], cmd[2])
            break
        else:
            print("invalid opcode %d at position %d" % (cmd[pc], pc))
            exit()

        arg1 = cmd[cmd[pc + 1]]
        arg2 = cmd[cmd[pc + 2]]
        res_id = cmd[pc + 3]
        res = oper(op, arg1, arg2)
        cmd[res_id] = res
        pc += 4


''' START '''

cmd = []
with open(IN, 'r') as f:
    for l in f:
        cmd += l.split(",")

for i in range (len(cmd)):
    cmd[i] = int(cmd[i])

for noun in range(100):
    for verb in range(100):
        cmd[1] = noun
        cmd[2] = verb
        run(cmd[:]) # pass a copy
