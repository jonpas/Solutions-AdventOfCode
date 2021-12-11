#!/usr/bin/env python3

with open("d10_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

scores = []

for line in data:
    c_open = []
    for i, c in enumerate(line):
        if c in "([{<":
            c_open.append(c)
        else:  # ")]}>"
            last_open = c_open.pop()

            should_close = ")"
            if last_open == "[":
                should_close = "]"
            elif last_open == "{":
                should_close = "}"
            elif last_open == "<":
                should_close = ">"

            # Discard corrupt line
            if c != should_close:
                c_open = []
                break

    # Autocomplete an incomplete line
    if c_open:
        to_close = ""
        score = 0
        for c in c_open[::-1]:
            score *= 5
            if c == "(":
                to_close += ")"
                score += 1
            elif c == "[":
                to_close += "]"
                score += 2
            elif c == "{":
                to_close += "}"
                score += 3
            elif c == "<":
                to_close += ">"
                score += 4

        scores.append(score)
        print(f"{line}\n  complete by adding {to_close} ({score=})")

scores.sort()
middle_score = scores[int(len(scores) / 2)]
print(f"{scores=}")
print(f"{middle_score=}")
