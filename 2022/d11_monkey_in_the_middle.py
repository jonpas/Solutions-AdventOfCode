#!/usr/bin/env python3

with open("d11_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    data = [line.strip() for line in data]


class Monkey:
    def __init__(self, mid):
        self.id = mid
        self.items = []  # array of item IDs: [79, 98, ...]
        self.operation = ()  # tuple of operation and number: ("*", 19), ("+", 6), ...
        self.test = 0  # divisibility test: number
        self.target_true = 0  # throw target if test true: number
        self.target_false = 0  # throw target if test false: number

        self.inspected = 0

    def __repr__(self):
        return f"Monkey({self.id}, items={self.items}, op={self.operation}, test={self.test} => [{self.target_true}, {self.target_false}])"


monkeys = []

for line in data:
    if line.startswith("Monkey"):
        mid = line.split()[1][:-1]
        monkeys.append(Monkey(mid))
    elif line.startswith("Starting"):
        items = line.split(":")[1].split(",")
        items = [int(item.strip()) for item in items]
        monkeys[-1].items = items
    elif line.startswith("Operation"):
        operation = line.split("= old ")[1].split()
        operation = (operation[0], "old" if operation[1] == "old" else int(operation[1]))
        monkeys[-1].operation = operation
    elif line.startswith("Test"):
        test = int(line.split("divisible by ")[1])
        monkeys[-1].test = test
    elif line.startswith("If true"):
        target_true = int(line.split("to monkey ")[1])
        monkeys[-1].target_true = target_true
    elif line.startswith("If false"):
        target_false = int(line.split("to monkey ")[1])
        monkeys[-1].target_false = target_false

print(monkeys)

for rnd in range(1, 21):
    for monkey in monkeys:
        print(f"Monkey {monkey.id}")

        for item in monkey.items:
            print(f" Inspect {item}")

            op, val = monkey.operation
            if val == "old":
                val = item

            if op == "+":
                item += val
            elif op == "*":
                item *= val
            else:
                print("  Unknown operation!")
                exit(1)
            print(f"  Worry {monkey.operation} -> {item}")

            item //= 3
            print(f"  Worry // 3 -> {item}")

            if item % monkey.test == 0:
                print(f"  Test True => {item} thrown to monkey {monkey.target_true}")
                monkeys[monkey.target_true].items.append(item)  # monkeys are sorted by id
            else:
                print(f"  Test True => {item} thrown to monkey {monkey.target_false}")
                monkeys[monkey.target_false].items.append(item)  # monkeys are sorted by id

            monkey.inspected += 1

        monkey.items = []

    print(f"\nAfter round {rnd}")
    for monkey in monkeys:
        print(f"Monkey {monkey.id}: {monkey.items}")

print()
for monkey in monkeys:
    print(f"Monkey {monkey.id} inspected {monkey.inspected} items")

monkeys.sort(key=lambda x: x.inspected, reverse=True)
monkey_biz = monkeys[0].inspected * monkeys[1].inspected
print(monkey_biz)
