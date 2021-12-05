#!/usr/bin/env python3

import numpy as np

with open("d5_input", "r", encoding="utf-8") as f:
    data, max_x, max_y = [], 0, 0
    for line in f:
        split_line = line.strip().split(" -> ")
        x1, y1 = split_line[0].split(",")
        x1, y1 = int(x1), int(y1)
        x2, y2 = split_line[1].split(",")
        x2, y2 = int(x2), int(y2)
        data.append((x1, y1, x2, y2))
        max_x, max_y = max(max_x, x1, x2), max(max_y, y1, y2)


diagram = np.zeros((max_x + 1, max_y + 1))
for x1, y1, x2, y2 in data:
    used_x, used_y = [], []
    xoffset = 1 if x1 <= x2 else -1
    yoffset = 1 if y1 <= y2 else -1
    for x in range(x1, x2 + xoffset, xoffset):
        for y in range(y1, y2 + yoffset, yoffset):
            if (x not in used_x and y not in used_y) or x1 == x2 or y1 == y2:
                used_x.append(x)
                used_y.append(y)
                diagram[x, y] += 1

print(diagram.T)

overlap = diagram[diagram > 1].size
print(f"{overlap=}")
