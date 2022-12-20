#!/usr/bin/env python3

with open("d10_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

signal_sum, x = 0, 1
line, op, val = 0, None, 0

for cycle in range(1, 221):
    if cycle in [20, 60, 100, 140, 180, 220]:
        signal = cycle * x
        signal_sum += signal
        print(f" {signal} => {signal_sum}")

    if op is None:
        instr = data[line]
        if instr != "noop":
            op, val = instr.split()
        line += 1
    else:
        x += int(val)
        op = None

    print(f"({cycle}) X = {x}")

print(signal_sum)
