#!/usr/bin/env python3

from heapq import heappush, heappop
import numpy as np

risks = np.genfromtxt("d15_input", dtype=int, delimiter=1)
print(risks)


def get_neighbours(graph, center):
    x, y = center

    neighbours = []
    if x > 0:
        neighbours.append((x - 1, y))
    if y > 0:
        neighbours.append((x, y - 1))
    if x + 1 < graph.shape[1]:
        neighbours.append((x + 1, y))
    if y + 1 < graph.shape[0]:
        neighbours.append((x, y + 1))

    return neighbours


def dijkstra(graph, source, target=()):
    dist = np.full(graph.shape, np.inf)
    dist[source] = 0

    q = []
    heappush(q, source)

    while q:
        u = heappop(q)

        if u == target:
            break

        for v in get_neighbours(graph, u):
            alt = dist[u] + graph[v]
            if alt < dist[v]:
                dist[v] = alt
                if v not in q:
                    heappush(q, v)

    return dist


source = (0, 0)
target = (risks.shape[0] - 1, risks.shape[1] - 1)
dist = dijkstra(risks, (0, 0), target)
print(dist)

print(f"risk={dist[target]}")
