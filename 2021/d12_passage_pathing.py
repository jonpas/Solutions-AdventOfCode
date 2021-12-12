#!/usr/bin/env python3

from collections import defaultdict

with open("d12_input", "r", encoding="utf-8") as f:
    data = f.readlines()

graph = defaultdict(list)
for line in data:
    begin, stop = line.strip().split("-")
    graph[begin].append(stop)
    graph[stop].append(begin)

print(f"{graph=}")


def walk(graph, paths, path, cons):
    for con in cons:
        if con == "end":
            paths.append(path + ["end"])  # need a copy anyways
        elif con != "start" and (con.isupper() or con not in path):
            path.append(con)
            walk(graph, paths, path, graph[con])
            path.pop()


paths = []
for con in graph["start"]:
    walk(graph, paths, ["start", con], graph[con])

for path in paths:
    print(f"{path=}")

print(f"{len(paths)=}")
