#!/usr/bin/env python3

with open("d13_input", "r", encoding="utf-8") as f:
    data = f.readlines()

    dots, folds, past_empty = [], [], False
    for line in data:
        strip = line.strip()
        if line.split():
            if past_empty:
                fold_axis, fold_coord = strip.split()[2].split("=")
                folds.append((fold_axis, int(fold_coord)))
            else:
                dots.append([int(x) for x in strip.split(",")])
        else:
            past_empty = True

print(f"{dots=}")

for fold_axis, fold_coord in folds[:1]:
    print(f"fold_{fold_axis}={fold_coord}")

    for dot in dots:
        i = 0 if fold_axis == "x" else 1

        if dot[i] > fold_coord:
            dot[i] = fold_coord - (dot[i] - fold_coord)

dots_set = set([tuple(d) for d in dots])
print(f"{len(dots_set)=}")
