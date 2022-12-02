#!/usr/bin/env python3

with open("d02_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip().split() for line in data]
    data = [(move, int(val)) for move, val in data]

position, depth, aim = 0, 0, 0

for move, val in data:
    if move == "forward":
        position += val
        depth += aim * val
    elif move == "down":
        aim += val
    elif move == "up":
        aim -= val
    else:
        print(f"error: {move} {val}")

print(f"{position=}, {depth=}")

multiplied = position * depth
print(f"{multiplied=}")
