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

# Free up just enough space
unused_space = 70_000_000 - dirsize["//"]
to_delete = 30_000_000 - unused_space
print(f"{unused_space=}, {to_delete=}")

dirsize = dict(sorted(dirsize.items(), key=lambda item: item[1]))
print(dirsize)

for d in dirsize.values():
    if d >= to_delete:
        print(d)
        break
