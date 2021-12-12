#!/usr/bin/env python3

with open("d3_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

    datan = [[]] * len(data[0])
    for i in range(len(datan)):
        datan[i] = [b[i] for b in data]


def rating(least_common=False):
    keep = list(range(len(data)))
    for binary in datan:
        considered = [b for i, b in enumerate(binary) if i in keep]
        zeroes = considered.count("0")
        ones = considered.count("1")

        less_ones = "1" if least_common else "0"
        more_ones = "0" if least_common else "1"

        rem = [j for j, b in enumerate(binary) if (zeroes <= ones and b == less_ones) or (zeroes > ones and b == more_ones)]
        keep = [k for k in keep if k not in rem]

        if len(keep) <= 1:
            final = data[keep[0]]
            return final


o2 = rating(least_common=False)
co2 = rating(least_common=True)

print(f"{o2=} ({int(o2, 2)=}), {co2=} ({int(co2, 2)=})")

life_support = int(o2, 2) * int(co2, 2)
print(f"{life_support=}")
