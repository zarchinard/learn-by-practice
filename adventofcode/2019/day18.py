import cmath
import math
import re
from copy import copy

lines = open("input.txt").readlines()
plan = {}
for y in range(len(lines)):
    line = lines[y]
    for x in range(len(line)):
        if line[x] != "#" and line[x] != "\n":
            plan[complex(x, y)] = line[x]
start = 0
for k, v in plan.items():
    if v == "@":
        start = k


class Node:
    def __init__(self, position, keys, steps):
        self.position = position
        self.keys = keys
        self.steps = steps

    def __repr__(self):
        return (
            "position : "
            + str(self.position)
            + " steps : "
            + str(self.steps)
            + " keys : "
            + str(self.keys)
        )

    def __str__(self):
        return (
            "position : "
            + str(self.position)
            + " steps : "
            + str(self.steps)
            + " keys : "
            + str(self.keys)
        )

    def findDirectKeys(self, bot):
        myPlan = copy(plan)
        for k, v in myPlan.items():
            if v.lower() in self.keys or v == "@":
                myPlan[k] = "."

        def isAccessible(point, nodes, distances, d):
            accessible = False
            shouldStop = False
            if point in myPlan:
                v = myPlan[point]
                if v == "." or re.match("^[a-z]$", v):
                    accessible = True
                if re.match("^[a-z]$", v):
                    shouldStop = True
            if accessible:
                if point not in distances:
                    distances[point] = d
                    if point not in nodes and not shouldStop:
                        nodes.append(point)
                else:
                    if d < distances[point]:
                        distances[point] = d
                        if point not in nodes and not shouldStop:
                            nodes.append(point)

        p = self.position
        if isinstance(self.position, list):
            p = self.position[bot]
        distances = {p: 0}
        nodes = [p]
        while len(nodes) > 0:
            node = nodes.pop(0)
            isAccessible(node + 1, nodes, distances, distances[node] + 1)
            isAccessible(node - 1, nodes, distances, distances[node] + 1)
            isAccessible(node + complex(0, 1), nodes, distances, distances[node] + 1)
            isAccessible(node + complex(0, -1), nodes, distances, distances[node] + 1)
        r = {}
        for k, v in distances.items():
            if re.match("^[a-z]$", myPlan[k]):
                r[k] = v
        return r


def part1():
    numberKeys = 0
    for _k, v in plan.items():
        if re.match("^[a-z]", v):
            numberKeys += 1
    node = Node(start, [], 0)
    nodes = [node]
    for _i in range(numberKeys):
        nodesPlusOne = []
        for node in nodes:
            distances = node.findDirectKeys(0)
            for p, distance in distances.items():
                keys = [plan[p]]
                keys.extend(node.keys)
                keys.sort()
                nodesPlusOne.append(Node(p, keys, node.steps + distance))
        for i in range(len(nodesPlusOne)):
            for j in range(i + 1, len(nodesPlusOne)):
                ni = nodesPlusOne[i]
                nj = nodesPlusOne[j]
                if ni.position == nj.position and ni.keys == nj.keys:
                    if ni.steps <= nj.steps:
                        nj.steps = math.inf
                    else:
                        ni.steps = math.inf
        nodesPlusOne = [n for n in nodesPlusOne if n.steps < math.inf]
        nodes = nodesPlusOne
    print(min(set(map(lambda x: x.steps, nodes))))


# part1()


def part2():
    numberKeys = 0
    for _k, v in plan.items():
        if re.match("^[a-z]", v):
            numberKeys += 1
    plan[start] = "#"
    plan[start + 1] = "#"
    plan[start - 1] = "#"
    plan[start + complex(0, 1)] = "#"
    plan[start - complex(0, -1)] = "#"
    bots = [
        start - 1 + complex(0, -1),
        start + 1 + complex(0, -1),
        start - 1 + complex(0, 1),
        start + 1 + complex(0, 1),
    ]
    node = Node(bots, [], 0)
    nodes = [node]
    for _i in range(numberKeys):
        print("iteration", _i)
        nodesPlusOne = []
        for node in nodes:
            for robot in range(4):
                distances = node.findDirectKeys(robot)
                for p, distance in distances.items():
                    keys = [plan[p]]
                    keys.extend(node.keys)
                    keys.sort()
                    bots = []
                    bots.extend(node.position)
                    bots[robot] = p
                    newNode = Node(bots, keys, node.steps + distance)
                    added = False
                    for i in range(len(nodesPlusOne)):
                        ni = nodesPlusOne[i]
                        if ni.keys == keys and ni.position == bots:
                            added = True
                            if ni.steps > newNode.steps:
                                nodesPlusOne[i] = newNode
                            break
                    if not added:
                        nodesPlusOne.append(newNode)
        print(len(nodesPlusOne))
        nodes = nodesPlusOne
    print(min(set(map(lambda x: x.steps, nodes))))


part2()
