#!/usr/bin/env python3

import numpy as np

with open("d22_input", "r", encoding="utf-8") as f:
    data = f.read().splitlines()

steps = []
for line in data:
    state, coords = line.split()
    xstr, ystr, zstr = coords.split(",")
    x = tuple([int(x) for x in xstr.split("=")[1].split("..")])
    y = tuple([int(y) for y in ystr.split("=")[1].split("..")])
    z = tuple([int(z) for z in zstr.split("=")[1].split("..")])

    steps.append((state, x, y, z))

print(steps)

cuboid_size = 101
cuboid = np.full((cuboid_size, cuboid_size, cuboid_size), 0)

for step in steps:
    print(f"{step=}")
    state, (xs, xe), (ys, ye), (zs, ze) = step

    if xs < -50 or xe > 50 or ys < -50 or ye > 50 or zs < -50 or ze > 50:
        continue

    for x in range(xs, xe + 1):
        for y in range(ys, ye + 1):
            for z in range(zs, ze + 1):
                cuboid[x + 50][y + 50][z + 50] = 1 if state == "on" else 0

unique, counts = np.unique(cuboid, return_counts=True)
unique_counts = dict(zip(unique, counts))
count_on = unique_counts.get(1, 0)
print(f"{count_on=}")
