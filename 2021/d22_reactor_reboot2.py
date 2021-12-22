#!/usr/bin/env python3

with open("d22_test2", "r", encoding="utf-8") as f:
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

on = set()

for step in steps:
    print(f"{step=}")
    state, (xs, xe), (ys, ye), (zs, ze) = step

    #if xs < -50 or xe > 50 or ys < -50 or ye > 50 or zs < -50 or ze > 50:
    #    continue

    for x in range(xs, xe + 1):
        for y in range(ys, ye + 1):
            for z in range(zs, ze + 1):
                if state == "on":
                    on.add((x, y, z))
                else:
                    on.discard((x, y, z))

count_on = len(on)
print(f"{count_on=}")
