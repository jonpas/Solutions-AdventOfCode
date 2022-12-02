#!/usr/bin/env python3

# A/X = Rock, B/Y = Paper, C/Z = Scissors
outcome_logic = {
    "A X": "draw", "A Y": "win", "A Z": "lose",
    "B X": "lose", "B Y": "draw", "B Z": "win",
    "C X": "win", "C Y": "lose", "C Z": "draw",
}
scoring_shape = {'X': 1, 'Y': 2, 'Z': 3}
scoring_result = {"lose": 0, "draw": 3, "win": 6}

score = 0

with open("d02_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

for line in data:
    _, player = line.split()

    outcome = outcome_logic[line]
    score += scoring_shape[player] + scoring_result[outcome]

print(score)
