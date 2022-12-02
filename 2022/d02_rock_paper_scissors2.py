#!/usr/bin/env python3

# A = Rock, B = Paper, C = Scissors
move_logic = {
    "A X": "C", "A Y": "A", "A Z": "B",
    "B X": "A", "B Y": "B", "B Z": "C",
    "C X": "B", "C Y": "C", "C Z": "A",
}
scoring_shape = {'A': 1, 'B': 2, 'C': 3}
# X = lose, Y = draw, Z = win
scoring_result = {"X": 0, "Y": 3, "Z": 6}

score = 0

with open("d02_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

for line in data:
    _, strategy = line.split()

    move = move_logic[line]
    score += scoring_shape[move] + scoring_result[strategy]

print(score)
