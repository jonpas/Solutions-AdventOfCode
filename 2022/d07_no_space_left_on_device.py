#!/usr/bin/env python3

from collections import defaultdict

with open("d07_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

dirsize = defaultdict(int)  # { "name": size }

cdir = ""
parents = []
for line in data:
    if line.startswith("$"):
        # Commands
        cmdarg = line[2:].split()
        cmd, arg = cmdarg[0], cmdarg[1] if len(cmdarg) > 1 else ""
        print(f"{cmd} {arg}")

        if cmd == "cd":
            if arg == "..":
                cdir = parents.pop()
            else:
                if arg == "/":
                    parents = []
                else:
                    parents.append(cdir)
                cdir = arg
            print(f"  {cdir=} <= {parents=}")
    else:
        # Output
        size, name = line.split()
        if size != "dir":
            cdir_unique = "".join(parents) + "/" + cdir
            dirsize[cdir_unique] += int(size)

            parents_add = parents.copy()
            while parents_add:
                p = parents_add.pop()
                p_unique = "".join(parents_add) + "/" + p
                dirsize[p_unique] += int(size)

# Filter directories bigger than 100000
sizesum = 0
for d in dirsize.values():
    if d <= 100000:
        sizesum += d

print(dirsize)
print(sizesum)
