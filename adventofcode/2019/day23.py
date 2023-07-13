from collections import defaultdict

data = list(map(int, open("input.txt").readline().split(",")))

nat = None


class Computer:
    def __init__(self, data, address):
        r = defaultdict(lambda: 0)
        for key in range(len(data)):
            r[key] = data[key]
        self.r = r
        self.position = 0
        self.rbase = 0
        self.input = [address, -1]
        self.address = address
        self.output = []

    def run(self):
        def calculateParams(instruction, r, position, number):
            params = []
            for i in range(number):
                p = 0
                if instruction[-3 - i] == "0":
                    p = r[position + 1 + i]
                elif instruction[-3 - i] == "1":
                    p = position + 1 + i
                else:
                    p = r[position + 1 + i] + self.rbase
                params.append(p)
            return params

        while self.r[self.position] != 99:
            if len(self.output) == 3:
                global computers, nat
                a = self.output.pop(0)
                x = self.output.pop(0)
                y = self.output.pop(0)
                if a == 255:
                    if nat == None:
                        print(y)
                    nat = (x, y)
                else:
                    computers[a].input.extend([x, y])
                return "sent output"

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
                if len(self.input) == 0:
                    return "no more input"
                else:
                    self.r[params[0]] = self.input.pop(0)
                    self.position += 2
                    continue
            if instruction[-1] == "4":
                params = calculateParams(instruction, self.r, self.position, 1)
                self.output.append(self.r[params[0]])
                self.position += 2
                continue
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
            if instruction[-1] == "9":
                params = calculateParams(instruction, self.r, self.position, 1)
                self.rbase += self.r[params[0]]
                self.position += 2
                continue
            print("error")
            break


computers = []
returns = []
lasty = 0
for i in range(50):
    c = Computer(data, i)
    computers.append(c)
    returns.append("ready to start")
while True:
    for i in range(50):
        c = computers[i]
        returns[i] = c.run()
    if returns.count("no more input") == 50:
        x, y = nat
        computers[0].input.extend([x, y])
        if y == lasty:
            break
        else:
            lasty = y
print(lasty)
