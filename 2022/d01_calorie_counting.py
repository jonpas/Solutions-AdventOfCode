#!/usr/bin/env python3

max_cal = 0
cur_cal = 0

with open("d01_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    for line in data:
        line = line.strip()
        if line:
            cur_cal += int(line)
            if cur_cal > max_cal:
                max_cal = cur_cal
        else:
            cur_cal = 0

print(max_cal)
