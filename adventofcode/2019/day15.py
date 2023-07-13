from collections import defaultdict
from copy import copy
import cmath
import math

data = open("input.txt").readline().split(",")
r = defaultdict(lambda: 0)
for i in range(len(data)):
    r[i] = int(data[i])


class Droid:
    def __init__(self, coord, r):
        self.coord = coord
        self.r = r
        self.input = 0
        self.position = 0
        self.rbase = 0

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
                self.r[params[0]] = self.input
                self.position += 2
                continue
            if instruction[-1] == "4":
                params = calculateParams(instruction, self.r, self.position, 1)
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
            if instruction[-1] == "9":
                params = calculateParams(instruction, self.r, self.position, 1)
                self.rbase += self.r[params[0]]
                self.position += 2
                continue
            print("error")
            break


ship = {}
ship[0] = "."
bots = []
bots.append(Droid(0, r))

while len(bots) > 0:
    b = bots.pop(0)
    directions = [1, complex(0, 1), complex(0, -1), -1]

    def copyAndGo(bot, direction):
        diff = directions[direction % 4]
        bcopy = Droid(bot.coord, copy(bot.r))
        bcopy.position = bot.position
        bcopy.rbase = bot.rbase
        bcopy.input = direction
        d = bcopy.run()
        if d == 0:
            ship[bot.coord + diff] = "#"
        elif d == 1:
            ship[bot.coord + diff] = "."
            bcopy.coord = b.coord + diff
        else:
            ship[bot.coord + diff] = "O"
            bcopy.coord = bot.coord + diff
        bots.append(bcopy)

    direction = None
    if b.coord + complex(0, 1) not in ship:
        if direction == None:
            direction = 1
        else:
            copyAndGo(b, direction)
    if b.coord + complex(0, -1) not in ship:
        if direction == None:
            direction = 2
        else:
            copyAndGo(b, 2)
    if b.coord - 1 not in ship:
        if direction == None:
            direction = 3
        else:
            copyAndGo(b, 3)
    if b.coord + 1 not in ship:
        if direction == None:
            direction = 4
        else:
            copyAndGo(b, 4)
    if direction != None:
        b.input = direction
        diff = directions[direction % 4]
        d = b.run()
        if d == 0:
            ship[b.coord + diff] = "#"
        elif d == 1:
            ship[b.coord + diff] = "."
            b.coord = b.coord + diff
        else:
            ship[b.coord + diff] = "O"
            b.coord = b.coord + diff
        bots.append(b)
po = 0
for k, v in ship.items():
    if v == "O":
        po = k
        break


def calculateDistances(initialPosition):
    distances = {}
    for k, _v in ship.items():
        distances[k] = math.inf
    distances[initialPosition] = 0

    nodes = [initialPosition]
    while len(nodes) > 0:
        n = nodes.pop(0)
        d = distances[n] + 1
        if d < distances[n + complex(0, 1)] and ship[n + complex(0, 1)] != "#":
            distances[n + complex(0, 1)] = d
            nodes.append(n + complex(0, 1))
        if d < distances[n + complex(0, -1)] and ship[n + complex(0, -1)] != "#":
            distances[n + complex(0, -1)] = d
            nodes.append(n + complex(0, -1))
        if d < distances[n + 1] and ship[n + 1] != "#":
            distances[n + 1] = d
            nodes.append(n + 1)
        if d < distances[n - 1] and ship[n - 1] != "#":
            distances[n - 1] = d
            nodes.append(n - 1)
    return distances


print(calculateDistances(0)[po])
maxTime = 0
for k, v in calculateDistances(po).items():
    if v == math.inf:
        continue
    maxTime = v if v > maxTime else maxTime
print(maxTime)
