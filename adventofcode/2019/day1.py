import math

f = open("input1.txt", "r")
lines = f.readlines()

total1 = 0
total2 = 0


def calculateFuel(x):
    return math.floor(int(x) / 3) - 2


for x in lines:
    total1 += calculateFuel(x)
    y = calculateFuel(x)
    while y > 0:
        total2 += y
        y = calculateFuel(y)

print(total1)
print(total2)
