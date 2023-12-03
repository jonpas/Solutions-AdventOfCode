#!/usr/bin/env python3

import ast
import itertools

with open("d13_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

right_order = []


def compare(i, p1, p2):
    for left, right in itertools.zip_longest(p1, p2, fillvalue=-1):
        print(f" {left} vs {right}")

        if isinstance(left, int) and isinstance(right, int):
            if left < right:
                right_order.append(i + 1)
                print("  right")
                return True
            elif left > right:
                print("  not")
                return True
        else:
            if isinstance(left, int):
                left = [left]
            elif isinstance(right, int):
                right = [right]

            if not left:
                right_order.append(i + 1)
                print("  right 2")
                return True
            elif not right:
                print("  not 2")
                return True

            if compare(i, left, right):
                return True

    return False


for i, (line1, line2) in enumerate(list(itertools.zip_longest(data[::3], data[1::3], fillvalue=-1))):
    p1 = ast.literal_eval(line1)
    p2 = ast.literal_eval(line2)
    print(f"{p1} vs {p2}")

    compare(i, p1, p2)
    print()

print(right_order)
print(sum(right_order))
