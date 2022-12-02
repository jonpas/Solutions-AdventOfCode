#!/usr/bin/env python3

with open("d08_input", "r", encoding="utf-8") as f:
    data = f.readlines()
    # data = ["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"]  # test

    data = [line.strip().split(" | ") for line in data]
    signals = [line[0].split() for line in data]
    outputs = [line[1].split() for line in data]

print(f"{signals=}\n{outputs=}")

sum_output = 0
for signal, output in zip(signals, outputs):
    digits = [""] * 10

    # Find unique lengths first
    for segments in signal:
        num_segments = len(segments)
        if num_segments == 2:
            digits[1] = segments
        elif num_segments == 3:
            digits[7] = segments
        elif num_segments == 4:
            digits[4] = segments
        elif num_segments == 7:
            digits[8] = segments

    # Find other lengths
    while digits.count("") > 0:
        for segments in signal:
            num_segments = len(segments)

            if num_segments == 5:
                if len(set(digits[1]) & set(segments)) == 2:
                    digits[3] = segments
                elif len(set(digits[4]) & set(segments)) == 3:
                    digits[5] = segments
                else:
                    digits[2] = segments
            elif num_segments == 6:
                if len(set(digits[4]) & set(segments)) == 4:
                    digits[9] = segments
                elif len(set(digits[1]) & set(segments)) == 2:
                    digits[0] = segments
                else:
                    digits[6] = segments

    print(f"{digits=}")

    digits = [set(digit) for digit in digits]

    value = ""
    for segment in output:
        value += str(digits.index(set(segment)))

    print(f"  {value=}")
    sum_output += int(value)

print(f"{sum_output=}")
