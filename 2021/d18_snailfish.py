#!/usr/bin/env python3

import ast

data = []
with open("d18_test", "r", encoding="utf-8") as f:
    for line in f:
        sf = ast.literal_eval(line)
        data.append(sf)


def addition(left, right):
    return [left, right]


def reduce(pairs):
    last_num_i = []
    reduced = False
    for i, inest in enumerate(pairs):
        if reduced:
            break
        elif isinstance(inest, list):
            for j, jnest in enumerate(inest):
                if reduced:
                    break
                elif isinstance(jnest, list):
                    for k, knest in enumerate(jnest):
                        if reduced:
                            break
                        elif isinstance(knest, list):
                            for l, lnest in enumerate(knest):
                                if reduced:
                                    break
                                elif isinstance(lnest, list):
                                    print(f"fourth: {i}, {j}, {k}, {l}: {lnest}")
                                    explode(pairs, i, j, k, l, last_num_i)
                                    reduced = True
                                else:
                                    last_num_i = [i, j, k, l]
                        else:
                            last_num_i = [i, j, k]
                else:
                    last_num_i = [i, j]
        else:
            last_num_i = [i]


def explode(pairs, i, j, k, l, last_num_i):
    print(f"pre-explode: {pairs=}")
    left, right = pairs[i][j][k][l]
    if isinstance(right, list):
        left, right = right
    print(f"{left=}, {right=} | {last_num_i=}:")

    # Find next regular number
    #next_num_i = []
    next_done = False
    for i2, inest in enumerate(pairs[i:]):
        if next_done:
            break
        elif isinstance(inest, list):
            for j2, jnest in enumerate(inest[j:]):
                if next_done:
                    break
                elif isinstance(jnest, list):
                    for k2, knest in enumerate(jnest[k:]):
                        if next_done:
                            break
                        elif isinstance(knest, list):
                            for l2, lnest in enumerate(knest[l:]):
                                if next_done:
                                    break
                                elif isinstance(lnest, int):
                                    #print(i2, j2, k2, l2, pairs[i2][j2][k2][l2])
                                    pairs[i2][j2][k2][l2] += right
                                    #print(i2, j2, k2, l2, pairs[i2][j2][k2][l2])
                                    next_done = True
                        else:
                            #print(i2, j2, k2, pairs[i2][j2][k2])
                            pairs[i2][j2][k2] += right
                            #print(i2, j2, k2, pairs[i2][j2][k2])
                            next_done = True
                else:
                    #print(i2, j2, pairs[i2][j2])
                    pairs[i2][j2] += right
                    next_done = True
        else:
            #print(i2, pairs[i2])
            pairs[i2] += right
            next_done = True

    #print(i2, j2, k2, pairs[i2][j2][k2])
    #nxt_left, nxt_right = pairs[i2][j2][k2]
    #next_num_i = []
    #if isinstance(nxt_right, int):
    #    next_num_i = [i2, j2, k2]

    pairs[i][j][k][l] = 0
    print(f"mid-explode: {pairs=}")

    if last_num_i:
        #print(f"{pairs[last_num_i[0]][last_num_i[1]][last_num_i[2]][last_num_i[3]]}")
        pairs[last_num_i[0]][last_num_i[1]][last_num_i[2]][last_num_i[3]] += left
        #print(f"{pairs[last_num_i[0]][last_num_i[1]][last_num_i[2]][last_num_i[3]]}")
    #if next_num_i:
    #    pass
    #    #pairs[i][j][k][1] += right

    print(f"post-explode: {pairs=}")


def split():
    pass


# print(addition([1, 2], [[3, 4], 5]))  # test addition
# reduce([1, 2], [[3, 4], 5])  # test ???
reduce([[[[[9, 8], 1], 2], 3], 4])  # test explode
print()
reduce([7, [6, [5, [4, [3, 2]]]]])  # test explode
print()
reduce([[6, [5, [4, [3, 2]]]], 1])  # test explode
print()
reduce([[[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]])  # test explode
print()
reduce([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]])  # test explode

# for sf in data:
#     pass
#     #print(sf)
