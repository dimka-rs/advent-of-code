#!/usr/bin/env python3
import math

IN="./d2p1.in"

def oper(op, arg1, arg2):
    if op == '+':
        return arg1 + arg2
    elif op == '*':
        return arg1 * arg2

    print("wrong op: %c" % op)

cmd = []
with open(IN, 'r') as f:
    for l in f:
        cmd += l.split(",")

for i in range (len(cmd)):
    cmd[i] = int(cmd[i])

pc = 0
while True:
    op = '!'
    if cmd[pc] == 1:
        op = '+'
    elif cmd[pc] == 2:
        op = '*'
    elif cmd[pc] == 99:
        print("exit at position %d of %d" % (pc, len(cmd)))
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

print("Result: %d" % cmd[0])
