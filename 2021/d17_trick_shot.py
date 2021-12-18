#!/usr/bin/env python3

with open("d17_input", "r", encoding="utf-8") as f:
    data = f.readline().strip()
    # data = "target area: x=20..30, y=-10..-5"  # test

limits = [limit.strip("xy=,").split("..") for limit in data.split()[2:]]
x_limits = (int(limits[0][0]), int(limits[0][1]))
y_limits = (int(limits[1][0]), int(limits[1][1]))
print(f"{x_limits=}, {y_limits=}")

rmax_y = 0
for rx_vel in range(1, 100):
    for ry_vel in range(5, 500):
        x_vel, y_vel = rx_vel, ry_vel

        x, y = 0, 0
        max_y = 0
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
            max_y = max(max_y, y)

        if hit:
            print(f"hit: {rx_vel=}, {ry_vel=} - {x=}, {y=} (max: {max_y=})")
            rmax_y = max(rmax_y, max_y)

print(f"max: {rmax_y=}")
