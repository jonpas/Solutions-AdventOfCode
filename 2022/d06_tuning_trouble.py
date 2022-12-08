#!/usr/bin/env python3

with open("d06_input", "r", encoding="utf-8") as f:
    data = f.read().strip()

print(data)

for i in range(len(data) - 3):
    seq = data[i:i + 4]

    # marker found
    if len(set(seq)) == 4:
        print(i + 4)
        break
