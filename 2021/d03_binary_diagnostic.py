#!/usr/bin/env python3

with open("d03_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [list(line.strip()) for line in data]

    datan = [[]] * len(data[0])
    for i in range(len(datan)):
        datan[i] = [b[i] for b in data]

gamma, epsilon = "", ""

for binary in datan:
    zeroes = binary.count("0")
    ones = binary.count("1")
    gamma += "0" if zeroes >= ones else "1"
    epsilon += "0" if zeroes < ones else "1"


print(f"{gamma=} ({int(gamma, 2)=}), {epsilon=} ({int(epsilon, 2)=})")

power = int(gamma, 2) * int(epsilon, 2)
print(f"{power=}")
