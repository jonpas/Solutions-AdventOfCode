#!/usr/bin/env python3

with open("d16_input", "r", encoding="utf-8") as f:
    data = f.readline().strip()
    # data = "D2FE28"  # test literal value
    # data = "38006F45291200"  # test operator 0
    # data = "EE00D40C823060"  # test operator 1
    # data = "8A004A801A8002F478"  # test
    # data = "620080001611562C8802118E34"  # test
    # data = "C0015000016115A2E0802F182340"  # test
    # data = "A0016C880162017C3686B18A3D4780"  # test

print(data)

binary = "".join([bin(int(hexa, 16))[2:].zfill(4) for hexa in data])
print(binary)

sum_versions = 0


def packet(binary):
    version = int(binary[0:3], 2)
    type_id = int(binary[3:6], 2)
    print(f"{version=}, {type_id=}")

    global sum_versions
    sum_versions += version

    i = 6

    if type_id == 4:
        i += literal_value(binary[i:])
    else:
        i += operator(binary[i:])

    return i


def literal_value(binary):
    i, value = 0, ""

    while True:
        value += binary[i + 1:i + 5]
        i += 5
        if binary[i - 5] == "0":
            break

    print(f"literal {value} = {int(value, 2)}\n")
    return i


def operator(binary):
    i = 0

    length_type_id = int(binary[i])
    print(f"- operator {length_type_id}")
    if length_type_id == 0:
        # total length
        length = int(binary[i + 1:i + 16], 2)
        print(f"-- {length=}")

        i += 16
        last_i = i
        while i < last_i + length:
            i += packet(binary[i:])
    elif length_type_id == 1:
        # number of sub-packets
        length = int(binary[i + 1:i + 12], 2)
        print(f"-- {length=}")

        i += 12
        packets = 0
        while packets < length:
            i += packet(binary[i:])
            packets += 1
    else:
        print(f"    unknown {length_type_id=}")

    return i


packet(binary)

print(f"{sum_versions=}")
