input = open("input.txt").readline()
data = list(map(int, list(input)))
basePattern = [0, 1, 0, -1]


def part1(d):
    data = d
    n = len(data)
    for _phase in range(100):
        newData = []
        for i in range(n):
            r = 0
            for j in range(i, n):
                r += data[j] * basePattern[(j + 1) // (i + 1) % len(basePattern)]
            newData.append(int(str(r)[-1]))
        data = newData
    print("".join(list(map(str, data)))[0:8])


# part1(data)


def part2(d):
    data = []
    for _ in range(10000):
        data.extend(d)
    n = len(data)
    start = int("".join(list(map(str, data)))[0:7])
    if start < n // 2:
        print("not handled")
        return None

    for _phase in range(100):
        newData = {}
        r = 0
        for i in reversed(range(start, n)):
            r += data[i]
            newData[i] = int(str(r)[-1])
        data = newData

    r = []
    for i in range(8):
        r.append(str(data[start + i]))
    print("".join(r))


part2(data)
