#!/usr/bin/env python3

from collections import defaultdict

with open("d14_input", "r", encoding="utf-8") as f:
    template = f.readline().strip()
    next(f)
    pairs = f.readlines()
    pairs = [p.strip().split(" -> ") for p in pairs]
    pairs = {p[0]: p[1] for p in pairs}

print(f"{pairs=}")
print(f"{template=}")

steps = 40

ps = defaultdict(int)
for p0, p1 in zip(template, template[1:]):
    ps[f"{p0}{p1}"] += 1

print(f"step=0: {ps}")

for step in range(1, steps + 1):
    for pair, count in list(ps.items()):
        insert = pairs[pair]
        np0 = f"{pair[0]}{insert}"
        np1 = f"{insert}{pair[1]}"

        ps[pair] -= count
        if ps[pair] == 0:
            del ps[pair]

        ps[np0] += count
        ps[np1] += count

    count = sum(ps.values()) + 1
    print(f"{step=}: {ps} ({count})")

counts = defaultdict(int)
for pair, count in ps.items():
    for p in pair:
        counts[p] += count

for el, count in counts.items():
    counts[el] = -(-count // 2)

print(f"{counts=}")
max_c = max(counts, key=counts.get)
min_c = min(counts, key=counts.get)
print(f"{max_c=}, {min_c=}")

result = counts[max_c] - counts[min_c]
print(f"{result=}")
