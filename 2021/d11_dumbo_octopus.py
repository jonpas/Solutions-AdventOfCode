#!/usr/bin/env python3

import numpy as np

data = np.genfromtxt("d11_input", dtype=int, delimiter=1)
print(data)

steps = 100
# steps = 2  # test


def adjacent(data, y, x, flashed):
    if (y, x) not in flashed:
        if data[y, x] > 9:
            flashed.append((y, x))
            data[y, x] = 0

            if y > 0:
                adjacent(data, y - 1, x, flashed)
            if x > 0:
                adjacent(data, y, x - 1, flashed)
            if y + 1 < data.shape[0]:
                adjacent(data, y + 1, x, flashed)
            if x + 1 < data.shape[1]:
                adjacent(data, y, x + 1, flashed)
            if y > 0 and x > 0:
                adjacent(data, y - 1, x - 1, flashed)
            if y > 0 and x + 1 < data.shape[1]:
                adjacent(data, y - 1, x + 1, flashed)
            if x > 0 and y + 1 < data.shape[0]:
                adjacent(data, y + 1, x - 1, flashed)
            if y + 1 < data.shape[0] and x + 1 < data.shape[1]:
                adjacent(data, y + 1, x + 1, flashed)
        else:
            data[y, x] += 1
            if data[y, x] > 9:
                adjacent(data, y, x, flashed)


flashes = 0
for step in range(1, steps + 1):
    print(f"{step=}")

    data += 1

    flashed = []

    flashing = np.nonzero(data > 9)
    if flashing[0].size > 0:
        flashing = np.stack((flashing[0], flashing[1]), axis=-1)

        for y, x in flashing:
            adjacent(data, y, x, flashed)

    flashes += len(flashed)
    print(f"{data}\n")

print(f"{flashes=}")
