from collections import defaultdict

data = list(map(int, open("input.txt").readline().split(",")))


def program(data):
    r = defaultdict(lambda: 0)
    for key in range(len(data)):
        r[key] = data[key]
    position = 0
    rbase = 0
    inputs = []
    outputs = []

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
            if len(inputs) > 0:
                r[params[0]] = inputs.pop(0)
                position += 2
                continue
            else:
                message = list(map(chr, outputs))
                print("".join(message))
                outputs = []
                c = input()
                inputs = commandeToAscii(c)
                r[params[0]] = inputs.pop(0)
                position += 2
                continue

        if instruction[-1] == "4":
            params = calculateParams(instruction, r, position, 1)
            outputs.append(r[params[0]])
            position += 2
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
    message = list(map(chr, outputs))
    print("".join(message))


def commandeToAscii(s):
    r = list(map(ord, list(s)))
    r.append(10)
    return r


program(data)
