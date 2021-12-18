#!/usr/bin/env python3

with open("d17_input", "r", encoding="utf-8") as f:
    data = f.readline().strip()
    # data = "target area: x=20..30, y=-10..-5"  # test

limits = [limit.strip("xy=,").split("..") for limit in data.split()[2:]]
x_limits = (int(limits[0][0]), int(limits[0][1]))
y_limits = (int(limits[1][0]), int(limits[1][1]))
print(f"{x_limits=}, {y_limits=}")

vels = set()
for rx_vel in range(1, 500):
    for ry_vel in range(-500, 500):
        x_vel, y_vel = rx_vel, ry_vel

        x, y = 0, 0
        hit = False
        while x <= x_limits[1] and -1000 < y < 10000:
            if x_limits[0] <= x <= x_limits[1] and y_limits[0] <= y <= y_limits[1]:
                hit = True
                break

            x += x_vel
            y += y_vel
            if x_vel != 0:
                x_vel += (1 if x_vel < 0 else -1)
            y_vel -= 1

        if hit:
            print(f"hit: {rx_vel=}, {ry_vel=} - {x=}, {y=}")
            vels.add((rx_vel, ry_vel))

print(f"{len(vels)=}")
