f = open("input5.txt", "r")
contents = f.read()
data = list(map(int, contents.split(",")))


def program(data, input):
    r = data.copy()
    position = 0

    def calculateParams(instruction, r, position, number):
        params = []
        for i in range(number):
            params.append(
                r[position + 1 + i]
                if instruction[-3 - i] == "0"
                else (position + 1 + i)
            )
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
            r[params[0]] = input
            position += 2
            continue
        if instruction[-1] == "4":
            params = calculateParams(instruction, r, position, 1)
            print("output " + str(r[params[0]]))
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
        print("error")
        break


# Answer 1
# program(data, 1)

program(data, 5)

