#!/usr/bin/env python3

with open("d01_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [int(line.strip()) for line in data]

increases = 0

# three-measurement windows
for prev_ms, ms in zip(zip(data, data[1:], data[2:]), zip(data[1:], data[2:], data[3:])):
    if sum(prev_ms) < sum(ms):
        increases += 1

print(increases)
