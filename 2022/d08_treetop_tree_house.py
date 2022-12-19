#!/usr/bin/env python3

import numpy as np

forest = np.genfromtxt("d08_input", dtype=int, delimiter=1)
print(forest)

visible = 0
for y, x in np.ndindex(forest.shape):
    tree = forest[y, x]
    print(y, x, "=>", tree)

    row1 = list(forest[y, :x])
    row2 = list(forest[y, x + 1:])
    col1 = list(forest[:y, x])
    col2 = list(forest[y + 1:, x])
    print(f"Row: {row1}, {row2} | Col: {col1}, {col2}")

    vis_row1 = all(tree > c for c in row1)
    vis_row2 = all(tree > c for c in row2)
    vis_col1 = all(tree > c for c in col1)
    vis_col2 = all(tree > c for c in col2)
    vis = vis_row1 or vis_row2 or vis_col1 or vis_col2
    print(f"Visibility: {vis_row1}, {vis_row2}, {vis_col1}, {vis_col2} => {vis}")

    if vis:
        visible += 1

print(visible)
