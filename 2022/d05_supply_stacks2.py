#!/usr/bin/env python3

with open("d05_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.rstrip() for line in data]

stacks = []

for line in data:
    line_strip = line.strip()
    if line_strip.startswith("["):
        # Stacks
        line_split = line.replace("[", "").replace("]", "").replace("    ", " ").split(" ")
        stacks.append(line_split)
        print(stacks)
    elif line_strip.startswith("1"):
        # Stack numbering
        n_stacks = int(line_strip[-1])
        print(f"{n_stacks=}")

        # Extend smaller stacks for transpose to not cut data
        for stack in stacks:
            stack.extend([''] * (n_stacks - len(stack)))

        # Transpose
        stacks = list(map(list, zip(*stacks)))
        print(f"start = {stacks}")

        # Reverse each stack so pop() uses the correct end
        # and clear empty elements
        for stack in stacks:
            stack.reverse()
            stack[:] = [x for x in stack if x]

        print(f"ready = {stacks}")
    elif line_strip.startswith("move"):
        # Moves
        move = line_strip.split()
        mamount, mfrom, mto = int(move[1]), int(move[3]) - 1, int(move[5]) - 1

        crates = stacks[mfrom][-mamount:]
        stacks[mfrom][:] = stacks[mfrom][:-mamount]
        stacks[mto].extend(crates)

        print(f"move = {stacks}")

tops = [stack[-1] for stack in stacks]
print("".join(tops))
