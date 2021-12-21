#!/usr/bin/env python3

import itertools
from functools import cache

with open("d21_input", "r", encoding="utf-8") as f:
    pos1 = int(f.readline().strip().split()[4])
    pos2 = int(f.readline().strip().split()[4])
    # pos1, pos2 = 4, 8  # test


print(f"{pos1=}, {pos2=}")

rolls = [sum(x) for x in itertools.product([1, 2, 3], repeat=3)]
max_score = 21


@cache
def do_turn(pos, score, move):
    pos = (pos + move) % 10
    if pos == 0:
        pos = 10
    score += pos
    return pos, score


@cache
def roll(turn, pos1, pos2, score1=0, score2=0):
    wins1, wins2 = 0, 0
    for move in rolls:
        rpos1, rpos2, rscore1, rscore2 = pos1, pos2, score1, score2
        if turn:
            rpos1, rscore1 = do_turn(pos1, score1, move)
        else:
            rpos2, rscore2 = do_turn(pos2, score2, move)

        if rscore1 >= max_score:
            wins1 += 1
        elif rscore2 >= max_score:
            wins2 += 1
        else:
            wins = roll(not turn, rpos1, rpos2, rscore1, rscore2)
            wins1 += wins[0]
            wins2 += wins[1]

    return wins1, wins2


wins1, wins2 = roll(True, pos1, pos2)
print(f"{wins1=}, {wins2=}")
final = max(wins1, wins2)
print(f"{final=}")
