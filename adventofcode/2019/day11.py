import cmath
import math
from collections import defaultdict

data = list(map(int, open("input.txt").readline().split(",")))

inp = 0


def program(data):
    r = defaultdict(lambda: 0)
    for i in range(len(data)):
        r[i] = data[i]
    position = 0
    rbase = 0

    def calculateParams(instruction, r, position, number):
        params = []
        for i in range(number):
            p = 0
            if instruction[-3 - i] == "0":
                p = r[position + 1 + i]
            elif instruction[-3 - i] == "1":
                p = position + 1 + i
            else:
                p = r[position + 1 + i] + rbase
            params.append(p)
        return params

    while r[position] != 99:
        instruction = "0000" + str(r[position])
        if instruction[-1] == "1":
            params = calculateParams(instruction, r, position, 3)
            r[params[2]] = r[params[0]] + r[params[1]]
            position += 4
            continue
        if instruction[-1] == "2":
            params = calculateParams(instruction, r, position, 3)
            r[params[2]] = r[params[0]] * r[params[1]]
            position += 4
            continue
        if instruction[-1] == "3":
            params = calculateParams(instruction, r, position, 1)
            r[params[0]] = inp
            position += 2
            continue
        if instruction[-1] == "4":
            params = calculateParams(instruction, r, position, 1)
            position += 2
            yield r[params[0]]
            continue
        if instruction[-1] == "5":
            params = calculateParams(instruction, r, position, 2)
            if r[params[0]] != 0:
                position = r[params[1]]
            else:
                position += 3
            continue
        if instruction[-1] == "6":
            params = calculateParams(instruction, r, position, 2)
            if r[params[0]] == 0:
                position = r[params[1]]
            else:
                position += 3
            continue
        if instruction[-1] == "7":
            params = calculateParams(instruction, r, position, 3)
            r[params[2]] = 1 if r[params[0]] < r[params[1]] else 0
            position += 4
            continue
        if instruction[-1] == "8":
            params = calculateParams(instruction, r, position, 3)
            r[params[2]] = 1 if r[params[0]] == r[params[1]] else 0
            position += 4
            continue
        if instruction[-1] == "9":
            params = calculateParams(instruction, r, position, 1)
            rbase += r[params[0]]
            position += 2
            continue
        print("error")
        break


p = program(data)

panel = defaultdict(lambda: 0)
######## Part 2
panel[0] = 1
########
direction = complex(0, -1)
position = 0

positions = []
colors = []

try:
    while True:
        inp = panel[position]
        positions.append(position)
        colors.append(next(p))
        panel[position] = colors[-1]
        if next(p) == 0:
            direction = direction * complex(0, -1)
        else:
            direction = direction * complex(0, 1)
        position = position + direction
except StopIteration:
    pass

# Part 1
# print(len(set(positions)))

# Part 2
whites = [k for k in panel if panel[k] == 1]
x = list(map(lambda c: int(c.real), whites))
y = list(map(lambda c: int(c.imag), whites))

for j in range(min(y), max(y) + 1):
    line = []
    for i in range(min(x), max(x) + 1):
        if complex(i, j) in whites:
            line.append("#")
        else:
            line.append(" ")
    print("".join(line))
