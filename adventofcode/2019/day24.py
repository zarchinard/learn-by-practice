from collections import defaultdict

inp = open("input.txt").readlines()

data = defaultdict(lambda: ".")
data2 = {}
for y in range(5):
    for x in range(5):
        if inp[y][x] == "#":
            data[(x, y)] = "#"
            data2[(x, y)] = "#"
        else:
            data2[(x, y)] = "."


def dataToString(d):
    l = []
    for y in range(5):
        for x in range(5):
            l.append(d[(x, y)])
    return "".join(l)


def nextData(d):
    r = defaultdict(lambda: ".")
    for y in range(5):
        for x in range(5):
            numberBugs = 0
            numberBugs += 1 if d[(x - 1, y)] == "#" else 0
            numberBugs += 1 if d[(x + 1, y)] == "#" else 0
            numberBugs += 1 if d[(x, y - 1)] == "#" else 0
            numberBugs += 1 if d[(x, y + 1)] == "#" else 0

            if d[(x, y)] == "#":
                r[(x, y)] = "#" if numberBugs == 1 else "."
            else:
                r[(x, y)] = "#" if numberBugs == 1 or numberBugs == 2 else "."
    return r


# Part 1
# layouts = [dataToString(data)]
# while True:
#     data = nextData(data)
#     newLayout = dataToString(data)
#     if newLayout in layouts:
#         total = 0
#         for i in range(25):
#             if newLayout[i] == "#":
#                 total += pow(2, i)
#         print(total)
#         break
#     else:
#         layouts.append(newLayout)


def nextData2(outside, d, inside):
    def numberBugs(x, y):
        r = 0
        if y == 0:
            if x == 0:
                r += 1 if outside[(2, 1)] == "#" else 0
                r += 1 if outside[(1, 2)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            elif x == 4:
                r += 1 if outside[(2, 1)] == "#" else 0
                r += 1 if outside[(3, 2)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            else:
                r += 1 if outside[(2, 1)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
        elif y == 4:
            if x == 0:
                r += 1 if outside[(1, 2)] == "#" else 0
                r += 1 if outside[(2, 3)] == "#" else 0
                r += 1 if d[(0, 3)] == "#" else 0
                r += 1 if d[(1, 4)] == "#" else 0
            elif x == 4:
                r += 1 if outside[(3, 2)] == "#" else 0
                r += 1 if outside[(2, 3)] == "#" else 0
                r += 1 if d[(4, 3)] == "#" else 0
                r += 1 if d[(3, 4)] == "#" else 0
            else:
                r += 1 if outside[(2, 3)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
        elif y == 1:
            if x == 0:
                r += 1 if outside[(1, 2)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            elif x == 4:
                r += 1 if outside[(3, 2)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            elif x == 2:
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                for i in range(5):
                    r += 1 if inside[(i, 0)] == "#" else 0
            else:
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
        elif y == 3:
            if x == 0:
                r += 1 if outside[(1, 2)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            elif x == 4:
                r += 1 if outside[(3, 2)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            elif x == 2:
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
                for i in range(5):
                    r += 1 if inside[(i, 4)] == "#" else 0
            else:
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
        else:
            if x == 0:
                r += 1 if outside[(1, 2)] == "#" else 0
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            elif x == 4:
                r += 1 if outside[(3, 2)] == "#" else 0
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
            elif x == 1:
                r += 1 if d[(x - 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
                for i in range(5):
                    r += 1 if inside[(0, i)] == "#" else 0
            else:
                r += 1 if d[(x + 1, y)] == "#" else 0
                r += 1 if d[(x, y - 1)] == "#" else 0
                r += 1 if d[(x, y + 1)] == "#" else 0
                for i in range(5):
                    r += 1 if inside[(4, i)] == "#" else 0

        return r

    r = {}
    for y in range(5):
        for x in range(5):
            if y == 2 and x == 2:
                r[(x, y)] = "?"
                continue
            nbBugs = numberBugs(x, y)

            if d[(x, y)] == "#":
                r[(x, y)] = "#" if nbBugs == 1 else "."
            else:
                r[(x, y)] = "#" if nbBugs == 1 or nbBugs == 2 else "."
    return r


def countBugs(d):
    r = 0
    for k, v in d.items():
        if v == "#":
            r += 1
    return r


layouts = [data2]
for m in range(200):
    newLayouts = []
    d = defaultdict(lambda: ".")
    for i in range(len(layouts)):
        if i == 0:
            if len(layouts) > 1:
                newLayouts.append(nextData2(d, layouts[i], layouts[i + 1]))
            else:
                newLayouts.append(nextData2(d, layouts[i], d))
        elif i == len(layouts) - 1:
            newLayouts.append(nextData2(layouts[i - 1], layouts[i], d))
        else:
            newLayouts.append(nextData2(layouts[i - 1], layouts[i], layouts[i + 1]))
    newBegin = nextData2(d, d, layouts[0])
    for k, v in newBegin.items():
        if v == "#":
            newLayouts.insert(0, newBegin)
            break
    newEnd = nextData2(layouts[-1], d, d)
    for k, v in newEnd.items():
        if v == "#":
            newLayouts.append(newEnd)
            break
    layouts = newLayouts

print(sum(list(map(countBugs, layouts))))

