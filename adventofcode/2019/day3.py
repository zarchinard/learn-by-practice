import cmath

f = open("input3.txt", "r")
i1 = f.readline().replace("\n", "").split(",")
i2 = f.readline().split(",")


def wirePoints(instruction):
    r = [complex(0, 0)]
    for i in instruction:
        e = complex(1, 0)
        if i[0] == "U":
            e = complex(0, 1)
        elif i[0] == "D":
            e = complex(0, -1)
        elif i[0] == "L":
            e = complex(-1, 0)
        for _n in range(int(i[1:])):
            r.append(r[-1] + e)
    return r[1:]


def manD(c):
    return abs(c.real) + abs(c.imag)


w1 = wirePoints(i1)
w2 = wirePoints(i2)
intersections = set(w1).intersection(w2)

print(min(map(manD, intersections)))  # Answer 1


def timeD(w1, w2):
    def r(i):
        return w1.index(i) + w2.index(i) + 2

    return r


print(min(map(timeD(w1, w2), intersections)))  # Answer 2

