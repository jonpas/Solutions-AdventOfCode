#!/usr/bin/env python3

with open("d06_input", "r", encoding="utf-8") as f:
    data = f.read().strip()

print(data)

for i in range(len(data) - 13):
    seq = data[i:i + 14]

    # marker found
    if len(set(seq)) == 14:
        print(i + 14)
        break
