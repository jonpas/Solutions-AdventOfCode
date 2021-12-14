#!/usr/bin/env python3

from collections import defaultdict

with open("d14_input", "r", encoding="utf-8") as f:
    template = f.readline().strip()
    next(f)
    pairs = f.readlines()
    pairs = [p.strip().split(" -> ") for p in pairs]
    pairs = {p[0]: p[1] for p in pairs}

print(f"{pairs=}")
print(f"step=0: {template}")

steps = 10

for step in range(1, steps + 1):
    polymer = template[0]
    for p0, p1 in zip(template, template[1:]):
        insert = pairs[f"{p0}{p1}"]
        polymer += f"{insert}{p1}"

    template = polymer
    print(f"{step=}: {template if len(template) < 50 else '-'} ({len(template)})")

counts = defaultdict(int)
for c in template:
    counts[c] += 1

print(f"{counts=}")
max_c = max(counts, key=counts.get)
min_c = min(counts, key=counts.get)
print(f"{max_c=}, {min_c=}")

result = counts[max_c] - counts[min_c]
print(f"{result=}")
