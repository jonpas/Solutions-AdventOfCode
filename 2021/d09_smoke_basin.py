#!/usr/bin/env python3

import numpy as np

data = np.genfromtxt("d9_input", dtype=int, delimiter=1)

print(data)

lows = []
for y, x in np.ndindex(data.shape):
    height = data[y, x]

    if y > 0 and data[y - 1, x] <= height:
        continue
    elif x > 0 and data[y, x - 1] <= height:
        continue
    elif y + 1 < data.shape[0] and data[y + 1, x] <= height:
        continue
    elif x + 1 < data.shape[1] and data[y, x + 1] <= height:
        continue
    else:
        lows.append(height)

print(f"{lows=}")
risk = sum(lows) + len(lows)  # 1 plus its height == sum + length
print(f"{risk=}")
