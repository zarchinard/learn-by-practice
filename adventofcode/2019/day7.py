from itertools import permutations

f = open("input7.txt", "r")
contents = f.read()
data = list(map(int, contents.split(",")))


class Amplifier:
    def __init__(self, data, phase, inp):
        self.r = data.copy()
        self.phase = phase
        self.inp = inp
        self.position = 0
        self.notPhased = True

    def run(self):
        def calculateParams(instruction, r, position, number):
            params = []
            for i in range(number):
                params.append(
                    r[position + 1 + i]
                    if instruction[-3 - i] == "0"
                    else (position + 1 + i)
                )
            return params

        while self.r[self.position] != 99:
            instruction = "0000" + str(self.r[self.position])
            if instruction[-1] == "1":
                params = calculateParams(instruction, self.r, self.position, 3)
                self.r[params[2]] = self.r[params[0]] + self.r[params[1]]
                self.position += 4
                continue
            if instruction[-1] == "2":
                params = calculateParams(instruction, self.r, self.position, 3)
                self.r[params[2]] = self.r[params[0]] * self.r[params[1]]
                self.position += 4
                continue
            if instruction[-1] == "3":
                params = calculateParams(instruction, self.r, self.position, 1)
                if self.notPhased:
                    self.r[params[0]] = self.phase
                    self.notPhased = False
                else:
                    self.r[params[0]] = self.inp
                self.position += 2
                continue
            if instruction[-1] == "4":
                params = calculateParams(instruction, self.r, self.position, 1)
                # print("output " + str(r[params[0]]))
                self.position += 2
                return self.r[params[0]]
            if instruction[-1] == "5":
                params = calculateParams(instruction, self.r, self.position, 2)
                if self.r[params[0]] != 0:
                    self.position = self.r[params[1]]
                else:
                    self.position += 3
                continue
            if instruction[-1] == "6":
                params = calculateParams(instruction, self.r, self.position, 2)
                if self.r[params[0]] == 0:
                    self.position = self.r[params[1]]
                else:
                    self.position += 3
                continue
            if instruction[-1] == "7":
                params = calculateParams(instruction, self.r, self.position, 3)
                self.r[params[2]] = 1 if self.r[params[0]] < self.r[params[1]] else 0
                self.position += 4
                continue
            if instruction[-1] == "8":
                params = calculateParams(instruction, self.r, self.position, 3)
                self.r[params[2]] = 1 if self.r[params[0]] == self.r[params[1]] else 0
                self.position += 4
                continue
            print("error")
            break
        return "END"


# Part 1
part1 = 0
for combi in permutations(range(5), 5):
    inp = 0
    for x in combi:
        inp = Amplifier(data, x, inp).run()
    part1 = inp if inp > part1 else part1
print(part1)

# Part 2
part2 = 0
for combi in permutations(range(5, 10), 5):
    inp = 0
    amplifiers = []
    for x in combi:
        amplifiers.append(Amplifier(data, x, inp))
        inp = amplifiers[-1].run()
    output = inp
    while inp != "END":
        for amp in amplifiers:
            amp.inp = inp
            inp = amp.run()
            if inp == "END":
                break
        output = inp if inp != "END" else output
    part2 = output if output > part2 else part2
print(part2)
