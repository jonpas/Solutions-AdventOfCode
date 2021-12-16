#!/usr/bin/env python3

from functools import reduce

with open("d16_input", "r", encoding="utf-8") as f:
    data = f.readline().strip()
    # data = "C200B40A82"  # test sum
    # data = "04005AC33890"  # test product
    # data = "880086C3E88112"  # test minimum
    # data = "CE00C43D881120"  # test maximum
    # data = "D8005AC2A8F0"  # test less than
    # data = "F600BC2D8F"  # test greater than
    # data = "9C005AC2F8F0"  # test equal to
    # data = "9C0141080250320F1802104A08"  # test

print(data)

binary = "".join([bin(int(hexa, 16))[2:].zfill(4) for hexa in data])
print(binary)


def packet(binary):
    version = int(binary[0:3], 2)
    type_id = int(binary[3:6], 2)
    print(f"{version=}, {type_id=}")

    i, value = 6, 0

    if type_id == 4:
        value, j = literal_value(binary[i:])
    else:
        values, j = operator(binary[i:])
        if type_id == 0:
            value = sum(values)
        elif type_id == 1:
            value = reduce(lambda x, y: x * y, values)
        elif type_id == 2:
            value = min(values)
        elif type_id == 3:
            value = max(values)
        elif type_id == 5:
            value = int(values[0] > values[1])
        elif type_id == 6:
            value = int(values[0] < values[1])
        elif type_id == 7:
            value = int(values[0] == values[1])
        else:
            print(f"unknown {type_id=}")
    i += j

    return value, i


def literal_value(binary):
    i, bvalue = 0, ""

    while True:
        bvalue += binary[i + 1:i + 5]
        i += 5
        if binary[i - 5] == "0":
            break

    value = int(bvalue, 2)
    print(f"literal {bvalue} = {value}\n")
    return value, i


def operator(binary):
    i, values = 0, []

    length_type_id = int(binary[i])
    print(f"- operator {length_type_id}")
    if length_type_id == 0:
        # total length
        length = int(binary[i + 1:i + 16], 2)
        print(f"-- {length=}")

        i += 16
        last_i = i
        while i < last_i + length:
            value, j = packet(binary[i:])
            values.append(value)
            i += j
    elif length_type_id == 1:
        # number of sub-packets
        length = int(binary[i + 1:i + 12], 2)
        print(f"-- {length=}")

        i += 12
        packets = 0
        while packets < length:
            value, j = packet(binary[i:])
            values.append(value)
            i += j
            packets += 1
    else:
        print(f"-- unknown {length_type_id=}")

    return values, i


value, _ = packet(binary)
print(f"{value=}")
