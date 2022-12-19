#!/usr/bin/env python3

import numpy as np

forest = np.genfromtxt("d08_input", dtype=int, delimiter=1)
print(forest)


def get_vis(tree, vis_check):
    vis = 0
    for c in vis_check:
        vis += 1
        if c >= tree:
            break
    return vis


max_scenic = 0
for y, x in np.ndindex(forest.shape):
    tree = forest[y, x]
    print(y, x, "=>", tree)

    row1 = list(forest[y, :x])
    row2 = list(forest[y, x + 1:])
    col1 = list(forest[:y, x])
    col2 = list(forest[y + 1:, x])
    print(f"Row: {row1}, {row2} | Col: {col1}, {col2}")

    vis_row1 = get_vis(tree, reversed(row1))
    vis_row2 = get_vis(tree, row2)
    vis_col1 = get_vis(tree, reversed(col1))
    vis_col2 = get_vis(tree, col2)

    scenic = vis_row1 * vis_row2 * vis_col1 * vis_col2
    print(f"Visibility: {vis_row1}, {vis_row2}, {vis_col1}, {vis_col2} => Scenic: {scenic}")

    max_scenic = max(max_scenic, scenic)

print(max_scenic)
