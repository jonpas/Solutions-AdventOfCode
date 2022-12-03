#!/usr/bin/env python3

priorities = 0

with open("d03_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

for line in data:
    length = len(line)
    comp1 = line[:len(line) // 2]
    comp2 = line[len(line) // 2:]
    #print(f"{line} ({len(line)}) -> {comp1} ({len(comp1)}) & {comp2} ({len(comp2)})")

    in_both = set(comp1) & set(comp2)

    for letter in in_both:
        if letter.islower():
            priorities += ord(letter) - 96 # ord('a') -> 97, we want it to be 1
        else:
            priorities += ord(letter) - 38 # ord('A') -> 65, we want it to be 27

print(priorities)
