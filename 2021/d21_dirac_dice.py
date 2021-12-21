#!/usr/bin/env python3

with open("d21_input", "r", encoding="utf-8") as f:
    pos1 = int(f.readline().strip().split()[4])
    pos2 = int(f.readline().strip().split()[4])
    # pos1, pos2 = 4, 8  # test

print(f"{pos1=}, {pos2=}")

score1, score2 = 0, 0
die = 1

turn = True
rolls = 0
while score1 < 1000 and score2 < 1000:
    move = 0
    for _ in range(3):
        print(f"{die} ", end="")
        move += die
        die = (die + 1) if die < 100 else 1
        rolls += 1

    if turn:
        pos1 = (pos1 + move) % 10
        if pos1 == 0:
            pos1 = 10
        score1 += pos1
        print(f"=> {move} => {pos1=} ({score1=})")
    else:
        pos2 = (pos2 + move) % 10
        if pos2 == 0:
            pos2 = 10
        score2 += pos2
        print(f"=> {move} => {pos2=} ({score2=})")

    turn = not turn

losing_score = min(score1, score2)
final = losing_score * rolls
print(f"{final=} ({losing_score=})")
