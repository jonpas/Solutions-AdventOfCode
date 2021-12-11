#!/usr/bin/env python3

with open("d10_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

score = 0

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

            if c != should_close:
                print(f"{line}\n  expected {should_close}, but found {c} instead")

                points = 3  # ")"
                if c == "]":
                    points = 57
                elif c == "}":
                    points = 1197
                elif c == ">":
                    points = 25137
                score += points

print(f"{score=}")
