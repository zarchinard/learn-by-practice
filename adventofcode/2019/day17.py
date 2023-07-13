from collections import defaultdict
import cmath

data = list(map(int, open("input.txt").readline().split(",")))


def program(data, inp):
    r = defaultdict(lambda: 0)
    for key in range(len(data)):
        r[key] = data[key]
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


text = ""
p = program(data, 0)
for v in p:
    text += str(chr(v))
lines = text.split("\n")
lenY = len(lines) - 2
lenX = len(lines[0])
plan = {}
for y in range(lenY):
    line = lines[y]
    for x in range(lenX):
        plan[complex(x, y)] = line[x]

# Part 1
intersections = []
for y in range(1, lenY - 1):
    for x in range(1, lenX - 1):
        if (
            plan[complex(x, y)] == "#"
            and plan[complex(x - 1, y)] == "#"
            and plan[complex(x + 1, y)] == "#"
            and plan[complex(x, y - 1)] == "#"
            and plan[complex(x, y + 1)] == "#"
        ):
            intersections.append(complex(x, y))
print(sum(list(map(lambda c: c.real * c.imag, intersections))))

# Part 2
instructions = []
position = 0
direction = 0
for k, v in plan.items():
    if v != "." and v != "#":
        position = k
        if v == "^":
            direction = complex(0, -1)
        elif v == "<":
            direction = -1
        elif v == ">":
            direction = 1
        else:
            direction = complex(0, 1)
while True:
    if position + direction in plan and plan[position + direction] == "#":
        if len(instructions) == 0:
            instructions.append(1)
        else:
            instructions[-1] += 1
        position += direction
    elif (
        position + direction * complex(0, -1) in plan
        and plan[position + direction * complex(0, -1)] == "#"
    ):
        instructions.append("L")
        instructions.append(0)
        direction *= complex(0, -1)
    elif (
        position + direction * complex(0, 1) in plan
        and plan[position + direction * complex(0, 1)] == "#"
    ):
        instructions.append("R")
        instructions.append(0)
        direction *= complex(0, 1)
    else:
        break
# found by hand with instructions
a = "R,6,L,10,R,8"
b = "R,8,R,12,L,8,L,8"
c = "L,10,R,6,R,6,L,8"
main = "A,B,A,B,C,A,B,C,A,C"


def convertToAscii(str):
    r = []
    for c in str:
        r.append(ord(c))
    r.append(10)
    return r


asciiMain = convertToAscii(main)
asciiA = convertToAscii(a)
asciiB = convertToAscii(b)
asciiC = convertToAscii(c)

inputs = []
inputs.extend(asciiMain)
inputs.extend(asciiA)
inputs.extend(asciiB)
inputs.extend(asciiC)
inputs.extend([ord("n"), 10])

inp = 0
output = 0


def program2(data):
    r = defaultdict(lambda: 0)
    for key in range(len(data)):
        r[key] = data[key]
    r[0] = 2
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
            inp = yield
            r[params[0]] = inp
            #  yield inp
            position += 2
            continue
        if instruction[-1] == "4":
            params = calculateParams(instruction, r, position, 1)
            position += 2
            global output
            output = r[params[0]]
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


p = program2(data)
next(p)
for i in inputs:
    try:
        p.send(i)
    except StopIteration:
        print(output)
