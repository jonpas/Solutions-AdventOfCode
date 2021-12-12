#!/usr/bin/env python3

with open("d2_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip().split() for line in data]
    data = [(move, int(val)) for move, val in data]

position, depth = 0, 0

for move, val in data:
    if move == "forward":
        position += val
    elif move == "down":
        depth += val
    elif move == "up":
        depth -= val
    else:
        print(f"error: {move} {val}")

print(f"{position=}, {depth=}")

multiplied = position * depth
print(f"{multiplied=}")
