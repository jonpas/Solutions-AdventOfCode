#!/usr/bin/env python3

import numpy as np

with open("d7_input", "r", encoding="utf-8") as f:
    data = f.readline().strip().split(",")
    data = [int(crab) for crab in data]
    # data = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]  # test

crabs = np.array(data)
min_pos, max_pos = np.min(crabs), np.max(crabs)
print(f"{crabs=}, {min_pos=}, {max_pos=}")

costs = np.arange(0, max_pos + 1, dtype=int)
costs = np.cumsum(costs)
print(f"{costs=} ({len(costs)=})")

min_fuel_spent, end_pos = np.inf, -1
for pos in range(min_pos, max_pos + 1):
    target = np.full(crabs.shape, pos)
    diff = np.abs(crabs - target)
    diff_fuel = costs[diff]
    fuel_spent = np.sum(diff_fuel)
    # print(f"{pos=}\n  {target=}\n  {diff=}\n  {diff_fuel=}\n  {fuel_spent=}")

    if fuel_spent < min_fuel_spent:
        min_fuel_spent, end_pos = fuel_spent, pos

print(f"{min_fuel_spent=}")
