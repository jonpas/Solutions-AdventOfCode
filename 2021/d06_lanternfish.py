#!/usr/bin/env python3

with open("d06_input", "r", encoding="utf-8") as f:
    data = f.readline().strip().split(",")
    data = [int(fish) for fish in data]
    # data = [3, 4, 3, 1, 2]  # test

days = 80

print(f"Initial state: {data}")
for day in range(1, days + 1):
    new = 0
    for i, fish in enumerate(data):
        if fish == 0:
            data[i] = 6
            new += 1  # we don't want to include it in the current loop
        else:
            data[i] -= 1

    data.extend([8] * new)

    print(f"After {day} days: {data}")

print(f"Total fish: {len(data)}")
