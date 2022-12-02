#!/usr/bin/env python3

with open("d08_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    # data = ["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"]  # test

    data = [line.strip().split(" | ") for line in data]
    signals = [line[0].split() for line in data]
    outputs = [line[1].split() for line in data]

print(f"{signals=}\n{outputs=}")

digits_1478 = 0
for output in outputs:
    for segments in output:
        num_segments = len(segments)
        if num_segments in [2, 4, 3, 7]:
            digits_1478 += 1

print(f"{digits_1478=}")
