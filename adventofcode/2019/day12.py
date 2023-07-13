import re
import cmath


class Moon:
    def __init__(self, coord):
        self.x, self.y, self.z = coord
        self.vx, self.vy, self.vz = [0, 0, 0]

    def gravity(self, moon):
        if moon.x > self.x:
            self.vx += 1
        if moon.x < self.x:
            self.vx -= 1
        if moon.y > self.y:
            self.vy += 1
        if moon.y < self.y:
            self.vy -= 1
        if moon.z > self.z:
            self.vz += 1
        if moon.z < self.z:
            self.vz -= 1

    def velocity(self):
        self.x += self.vx
        self.y += self.vy
        self.z += self.vz

    def energy(self):
        return (abs(self.x) + abs(self.y) + abs(self.z)) * (
            abs(self.vx) + abs(self.vy) + abs(self.vz)
        )


def getInput():
    moons = []
    lines = open("input.txt").readlines()
    for line in lines:
        result = (re.findall(r"<x=(.*?), y=(.*?), z=(.*?)>", line))[0]
        result = list(map(int, result))
        moons.append(Moon(result))
    return moons


def step(moons):
    for m in moons:
        for n in moons:
            m.gravity(n)
    for m in moons:
        m.velocity()


def part1():
    moons1 = getInput()
    for _t in range(0, 1000):
        step(moons1)
    print(sum(list(map(lambda m: m.energy(), moons1))))


part1()


def part2():
    def lcm(x, y):
        from math import gcd

        return x * y // gcd(x, y)

    def stateX(moons):
        r = []
        for m in moons:
            r.append(complex(m.x, m.vx))
        return r

    def stateY(moons):
        r = []
        for m in moons:
            r.append(complex(m.y, m.vy))
        return r

    def stateZ(moons):
        r = []
        for m in moons:
            r.append(complex(m.z, m.vz))
        return r

    moons = getInput()
    initialState = stateX(moons)
    step(moons)
    px = 1
    while stateX(moons) != initialState:
        step(moons)
        px += 1

    moons = getInput()
    initialState = stateY(moons)
    step(moons)
    py = 1
    while stateY(moons) != initialState:
        step(moons)
        py += 1

    moons = getInput()
    initialState = stateZ(moons)
    step(moons)
    pz = 1
    while stateZ(moons) != initialState:
        step(moons)
        pz += 1

    print(lcm(pz, lcm(px, py)))


part2()
