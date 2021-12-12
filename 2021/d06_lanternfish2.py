#!/usr/bin/env python3

import numpy as np

MAX_TIMER = 8

with open("d6_input", "r", encoding="utf-8") as f:
    data = f.readline().strip().split(",")
    data = [int(fish) for fish in data]
    # data = [3, 4, 3, 1, 2]  # test

    # Reorganize data into timers with each timer having amount of fish (static size array)
    fish = [0] * (MAX_TIMER + 1)
    fish = [data.count(i) for i, f in enumerate(fish)]

fish = np.array(fish)

days = 256

print(f"Initial state: {fish}")
for day in range(1, days + 1):
    new = 0

    fish = np.roll(fish, -1)
    fish[6] += fish[-1]

    print(f"After {day} days: {fish}")

print(f"Total fish: {sum(fish)}")
