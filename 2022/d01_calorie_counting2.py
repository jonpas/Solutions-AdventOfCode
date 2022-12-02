#!/usr/bin/env python3

cals = []
cur_cal = 0

with open("d01_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    for line in data:
        line = line.strip()
        if line:
            cur_cal += int(line)
        else:
            cals.append(cur_cal)
            cur_cal = 0

cals.sort(reverse=True)
sum_top_3 = sum(cals[:3])

print(sum_top_3)
