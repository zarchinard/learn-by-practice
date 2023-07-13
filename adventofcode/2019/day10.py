import math
import cmath

asteroids = []
lines = open("input.txt", "r").readlines()
ySize = len(lines)
xSize = len(lines[0])
for y in range(len(lines)):
    line = lines[y]
    for x in range(len(line)):
        if line[x] == "#":
            asteroids.append(complex(x, y))


def isBetween(a, b, c):
    return cmath.phase(b - a) == cmath.phase(c - a) and abs(c - a) <= abs(b - a)


part1 = 0
target = None
for a in asteroids:
    number = len(
        set(map(lambda c: cmath.phase(c), [(b - a) for b in asteroids if b != a]))
    )
    if number > part1:
        target = a
        part1 = number

print(part1)

diffs = [(a - target) for a in asteroids if a != target]

angles = list(set(map(cmath.phase, diffs)))
angles.sort()
for a in angles:
    if a >= -math.pi / 2:
        cursor = angles.index(a)
        break
for i in range(200):
    inAngles = [d for d in diffs if cmath.phase(d) == angles[cursor]]
    if len(inAngles) > 0:
        inAngles.sort(key=abs)
        if i == 199:
            n = inAngles[0] + target
            print(n.real * 100 + n.imag)
        diffs.remove(inAngles[0])
    cursor = (cursor + 1) % len(angles)

