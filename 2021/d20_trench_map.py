#!/usr/bin/env python3

steps = 2  # part 1
# steps = 50  # part 2

with open("d20_input", "r", encoding="utf-8") as f:
    algorithm = f.readline().strip()
    next(f)
    image = f.read().splitlines()


def show(image):
    for ximage in image:
        print(ximage)
    print()


def pad(image, amount=2, pad_with="."):
    for n in range(amount):
        for x in range(len(image)):
            image[x] = pad_with + image[x] + pad_with
        image.insert(0, pad_with * len(image[0]))
        image.append(pad_with * len(image[0]))


def unpad(image, amount=1):
    for n in range(amount):
        image[:] = image[1:-1]
        for x in range(len(image)):
            image[x] = image[x][1:-1]


def enhance(image):
    image_new = image.copy()
    for x in range(1, len(image) - 1):
        for y in range(1, len(image[0]) - 1):
            area = image[x - 1][y - 1:y + 2] + image[x][y - 1:y + 2] + image[x + 1][y - 1:y + 2]

            area = area.replace(".", "0").replace("#", "1")
            areaint = int(area, 2)

            xy_out = algorithm[areaint]
            image_new[x] = image_new[x][:y] + xy_out + image_new[x][y + 1:]
            # print(f"{x=}, {y=}: {image[x][y]} -> {xy_out}")

    return image_new


def enhance_n(image, amount):
    for n in range(amount):
        pad(image, 2, "#" if n % 2 and algorithm[0] == "#" else ".")
        image = enhance(image)
        unpad(image, 1)
    return image


print(f"{algorithm=}\n")
show(image)

print("enhance:")
image = enhance_n(image, steps)
show(image)

lit = 0
for ximage in image:
    lit += ximage.count("#")

print(f"{lit=}")
