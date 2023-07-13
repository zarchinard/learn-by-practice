from collections import defaultdict
import cmath

data = list(map(int, open("input.txt").readline().split(",")))


class Game:
    def program(self):
        r = defaultdict(lambda: 0)
        for i in range(len(self.data)):
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
                r[params[0]] = self.inp
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

    def __init__(self, inp, data):
        self.inp = inp
        self.data = data


def part1():
    game = defaultdict(lambda: 0)
    p = Game(0, data)
    p = p.program()
    while True:
        try:
            c = complex(next(p), next(p))
            game[c] = next(p)
        except StopIteration:
            break

    part1 = 0
    for _k, v in game.items():
        part1 += 1 if v == 2 else 0

    print(part1)


part1()


def part2():
    game = defaultdict(lambda: 0)
    g = Game(0, data)
    g.data[0] = 2
    p = g.program()

    def positionByValue(value):
        for k, v in game.items():
            if v == value:
                return k
        return None

    blocksLeft = 333
    while True:
        try:
            x = next(p)
            y = next(p)
            v = next(p)
            if x == -1 and y == 0:
                if blocksLeft == 0:
                    print(v)
                    break
            else:
                if game[complex(x, y)] == 2 and v == 0:
                    blocksLeft -= 1
                game[complex(x, y)] = v
                ball = positionByValue(4)
                paddle = positionByValue(3)
                if ball != None and paddle != None:
                    ball = ball.real
                    paddle = paddle.real
                    if ball < paddle:
                        g.inp = -1
                    elif ball > paddle:
                        g.inp = 1
                    else:
                        g.inp = 0
        except StopIteration:
            break


part2()
