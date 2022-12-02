#!/usr/bin/env python3

with open("d01_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [int(line.strip()) for line in data]

increases = 0
for prev, m in zip(data, data[1:]):
    if prev < m:
        increases += 1

print(increases)
