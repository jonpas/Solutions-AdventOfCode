#!/usr/bin/env python3

with open("d10_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

x = 1
line, op, val = 0, None, 0

for cycle in range(240):
    if cycle in [40, 80, 120, 160, 200, 240]:
        print()

    if cycle % 40 in [x - 1, x, x + 1]:
        print("#", end="")
    else:
        print(".", end="")

    if op is None:
        instr = data[line]
        if instr != "noop":
            op, val = instr.split()
        line += 1
    else:
        x += int(val)
        op = None

print()
