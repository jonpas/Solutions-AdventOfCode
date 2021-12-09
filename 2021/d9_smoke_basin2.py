#!/usr/bin/env python3

import numpy as np

data = np.genfromtxt("d9_input", dtype=int, delimiter=1)

print(data)


def four_way(data, y, x, basin):
    height = data[y, x]

    if height < 9 and (y, x) not in basin:
        basin.append((y, x))

        if y > 0:
            four_way(data, y - 1, x, basin)
        if x > 0:
            four_way(data, y, x - 1, basin)
        if y + 1 < data.shape[0]:
            four_way(data, y + 1, x, basin)
        if x + 1 < data.shape[1]:
            four_way(data, y, x + 1, basin)


lows = []
basins = []
for y, x in np.ndindex(data.shape):
    height = data[y, x]

    # Is low point
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

        # Find basin
        basin = []  # coordinates
        four_way(data, y, x, basin)
        basins.append(len(basin))

print(f"{lows=}")
print(f"{basins=}")

basins.sort(reverse=True)

largest_mul = basins[0]
for basin_size in basins[1:3]:
    largest_mul *= basin_size

print(f"{largest_mul=}")
