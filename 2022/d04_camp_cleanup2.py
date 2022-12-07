#!/usr/bin/env python3

with open("d04_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

overlapped_pairs = 0

for line in data:
    range1, range2 = line.split(",")
    range1 = [int(x) for x in range1.split("-")]
    range2 = [int(x) for x in range2.split("-")]

    sections1 = list(range(range1[0], range1[1] + 1))
    sections2 = list(range(range2[0], range2[1] + 1))
    # print(range1, range2, "->", sections1, sections2)

    overlap = set(sections1) & set(sections2)
    if overlap:
        overlapped_pairs += 1

print(overlapped_pairs)
