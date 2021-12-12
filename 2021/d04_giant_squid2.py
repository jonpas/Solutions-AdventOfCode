#!/usr/bin/env python3

import numpy as np


def color_marked(x):
    # Get marked numbers
    idx = np.nonzero(marked)
    marked_nums = np.unique(boards[idx])
    # Mark marked
    return f"\033[1;31m{x}\033[0m" if x in marked_nums else f"{x}"


with open("d4_input", "r", encoding="utf-8") as f:
    draws = [int(num) for num in f.readline().strip().split(",")]
    print(f"{draws=}")

    boards, board = [], []
    for line in f:
        if not line.strip():
            if board:
                boards.append(board)
                board = []
            continue

        board_line = [int(num) for num in line.strip().split()]
        board.append(board_line)

    boards.append(board)

boards = np.array(boards)
marked = np.full(boards.shape, False)

np.set_printoptions(formatter={"int": color_marked})

wins = np.full((len(boards),), False)
last = -1
for draw in draws:
    # Mark current draw
    marked[boards == draw] = True

    # Check if any full line or column
    for i, mark in enumerate(marked):
        if not wins[i]:
            full_rows = np.all(mark, axis=1)
            full_columns = np.all(mark, axis=0)
            if np.any(full_rows) or np.any(full_columns):
                wins[i] = True
                last = i

    if np.all(wins):
        break

print(boards)

if last != -1:
    idx = np.nonzero(marked[last] == False)  # noqa: E712
    sum_unmarked = np.sum(boards[last][idx])
    score = sum_unmarked * draw
    print(f"{last=}")
    print(f"{score=} ({sum_unmarked=}, {draw=})")
else:
    print("error: no last winner")
