#!/usr/bin/env python3

with open("d09_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip().split() for line in data]

visited = set([(0, 0)])  # tail
head, tail = [0, 0], [0, 0]

for direction, steps in data:
    steps = int(steps)
    print(direction, steps)

    for step in range(steps):
        print(step)

        head_prev = head.copy()
        if direction == "U":
            head[1] += 1
        elif direction == "R":
            head[0] += 1
        elif direction == "D":
            head[1] -= 1
        elif direction == "L":
            head[0] -= 1

        dist_x = head[0] - tail[0]
        dist_y = head[1] - tail[1]
        dist = abs(dist_x) + abs(dist_y)
        print(f"{dist_x=}, {dist_y=} ({dist=})")

        if dist > 1:
            if dist > 2:
                print("move diagonally")
                tail = head_prev
            elif abs(dist_x) > 1:
                print("move on x")
                tail[0] += dist_x
                tail[0] += -1 if dist_x > 0 else 1
            elif abs(dist_y) > 1:
                print("move on y")
                tail[1] += dist_y
                tail[1] += -1 if dist_y > 0 else 1

        visited.add(tuple(tail))
        print(f"{head=}, {tail=}")

    print()

print(visited)
print(len(visited))
