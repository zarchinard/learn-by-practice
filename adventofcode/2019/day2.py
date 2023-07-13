f = open("input2.txt", "r")
contents = f.read()
data = list(map(int, contents.split(",")))


def program(data, noun, verb):
    r = data.copy()
    r[1] = noun
    r[2] = verb
    position = 0
    while r[position] != 99:
        if r[position] == 1:
            r[r[position + 3]] = r[r[position + 1]] + r[r[position + 2]]
        elif r[position] == 2:
            r[r[position + 3]] = r[r[position + 1]] * r[r[position + 2]]
        else:
            print("error")
            break
        position += 4
    return r[0]


print(program(data, 12, 2))  # answer1

for noun in range(0, 99):
    for verb in range(0, 99):
        if program(data, noun, verb) == 19690720:
            print(100 * noun + verb)  # answer2
            break

