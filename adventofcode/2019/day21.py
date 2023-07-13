from collections import defaultdict

data = list(map(int, open("input.txt").readline().split(",")))


def program(data, inputs):
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
            r[params[0]] = inputs.pop(0)
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


def runBot(instructions):

    inputs = []
    for i in instructions:
        inputs.append(ord(i))
    p = program(data, inputs)
    outputs = []
    while True:
        try:
            outputs.append(next(p))
        except StopIteration:
            break
    result = []
    for o in outputs:
        try:
            result.append(chr(o))
        except ValueError:
            print(o)
            return None
    print("".join(result))


# Part 1
runBot("NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n")

# Part 2
runBot(
    "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nNOT E T\nAND H T\nOR E T\nAND T J\nRUN\n"
)
