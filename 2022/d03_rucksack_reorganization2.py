#!/usr/bin/env python3

priorities = 0

with open("d03_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]

for ruck1, ruck2, ruck3 in zip(data[0::3], data[1::3], data[2::3]):
    in_all = set(ruck1) & set(ruck2) & set(ruck3)

    for letter in in_all:
        if letter.islower():
            priorities += ord(letter) - 96 # ord('a') -> 97, we want it to be 1
        else:
            priorities += ord(letter) - 38 # ord('A') -> 65, we want it to be 27

print(priorities)
