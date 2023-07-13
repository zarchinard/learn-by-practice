import math


def dealIntoNewStack(deck):
    deck.reverse()
    return deck


def cut(deck, N):
    total = len(deck)
    n = N
    if n < 0:
        n = total + n
    newDeck = deck[n:]
    newDeck.extend(deck[0:n])
    return newDeck


def dealWithIncrement(deck, N):
    newDeck = deck[0:]
    for i in range(len(deck)):
        newDeck[i * N % len(deck)] = deck[i]
    return newDeck


def runInstruction(i, deck):
    if i.startswith("deal into new stack"):
        return dealIntoNewStack(deck)
    elif i.startswith("cut"):
        return cut(deck, int(i[4:]))
    elif i.startswith("deal with increment"):
        return dealWithIncrement(deck, int(i[20:]))
    else:
        print("error unknown instruction", i)


lines = open("input.txt").readlines()
instructions = []
for line in lines:
    instructions.append(line.replace("\n", ""))

# Part 1
# deck = list(range(10007))
# for i in instructions:
#     deck = runInstruction(i, deck)
# print(deck.index(2019))

# Part 2
totalCards = 119315717514047
instructionsTimes = 101741582076661
positionEnd = 2020
instructions.reverse()


def reverseDeal():
    return (-1, totalCards - 1)


def reverseCut(N):
    return (1, N)
    # return (1, -N)


def reverseIncrement(N):
    n = invmod(N)
    return (n, 0)
    # return (N, 0)


def reverseInstruction(i):
    if i.startswith("deal into new stack"):
        return reverseDeal()
    elif i.startswith("cut"):
        return reverseCut(int(i[4:]))
    elif i.startswith("deal with increment"):
        return reverseIncrement(int(i[20:]))
    else:
        print("error unknown instruction", i)


def bezout(p, q):  # fct récursive qui renvoie (g,x,y) tq ax+by=g (=pgcd(a,b))
    if p == 0:
        return (q, 0, 1)
    else:
        g, y, x = bezout(q % p, p)
        return (g, x - (q // p) * y, y)


def invmod(a):  # inverse modulaire de a modulo totalCards
    g, x, _y = bezout(a, totalCards)
    if g != 1:
        raise Exception("pas inversible")
    else:
        return x % totalCards


poly = (1, 0)
for instruction in instructions:
    newA, newB = reverseInstruction(instruction)
    a, b = poly
    # newA(ax+b)+newB=(anewA)x + (bnewA + newB)
    poly = (
        a * newA % totalCards,
        (b * newA % totalCards + newB % totalCards) % totalCards,
    )
a, b = poly


def run_many(a, b, e):
    if e == 1:
        return a, b
    elif e % 2 == 0:
        return run_many((a * a) % totalCards, (a * b + b) % totalCards, e / 2)
    else:
        c, d = run_many(a, b, e - 1)
        return (a * c) % totalCards, (a * d + b) % totalCards


finalPolyElegant = run_many(a, b, instructionsTimes)
a, b = finalPolyElegant
# x = (2020 - b) * invmod(a) % totalCards
# print(x)
print((a * 2020 + b) % totalCards)

