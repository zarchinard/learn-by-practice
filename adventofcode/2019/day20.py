import re
import networkx as nx
from copy import copy

lines = open("input.txt").readlines()
plan = {}
maxY = len(lines) - 1
maxX = len(lines[0].replace("\n", "")) - 1
for y in range(len(lines)):
    line = lines[y]
    for x in range(len(line)):
        if line[x] == "." or re.match("^[A-Z]$", line[x]):
            plan[(x, y)] = line[x]


class Portal:
    def __init__(self, name, position, outside):
        self.name = name
        self.position = position
        self.outside = outside

    def __repr__(self):
        return self.name + " " + str(self.position) + " " + str(self.outside)

    def findEdges(self, portals, part2):
        edges = []

        def isAccessible(point, nodes, distances, d):
            accessible = False
            if point in plan:
                if plan[point] == ".":
                    accessible = True
            if accessible:
                if point not in distances:
                    distances[point] = d
                    if point not in nodes:
                        nodes.append(point)
                else:
                    if d < distances[point]:
                        distances[point] = d
                        if point not in nodes:
                            nodes.append(point)

        p = self.position
        distances = {p: 0}
        nodes = [p]
        while len(nodes) > 0:
            node = nodes.pop(0)
            x, y = node
            isAccessible((x + 1, y), nodes, distances, distances[node] + 1)
            isAccessible((x - 1, y), nodes, distances, distances[node] + 1)
            isAccessible((x, y + 1), nodes, distances, distances[node] + 1)
            isAccessible((x, y - 1), nodes, distances, distances[node] + 1)

        for portal in portals:
            if self.name == portal.name:
                if not part2:
                    edges.append((portal, 1))
            else:
                if portal.position in distances:
                    edges.append((portal, distances[portal.position]))
        return edges


def findPortals(plan):
    r = []
    p = copy(plan)

    for k, v in p.items():
        if re.match("^[A-Z]$", v):
            x, y = k
            if (x + 1, y) in p and re.match("^[A-Z]$", p[(x + 1, y)]):
                outside = False
                if x == 0 or x == maxX - 1:
                    outside = True
                name = p[(x, y)] + p[(x + 1, y)]
                p[(x, y)] = "?"
                p[(x + 1, y)] = "?"
                if (x + 2, y) in p and p[(x + 2, y)] == ".":
                    r.append(Portal(name, (x + 2, y), outside))
                else:
                    r.append(Portal(name, (x - 1, y), outside))
            else:
                outside = False
                if y == 0 or y == maxY - 1:
                    outside = True
                name = p[(x, y)] + p[(x, y + 1)]
                p[(x, y)] = "?"
                p[(x, y + 1)] = "?"
                if (x, y + 2) in p and p[(x, y + 2)] == ".":
                    r.append(Portal(name, (x, y + 2), outside))
                else:
                    r.append(Portal(name, (x, y - 1), outside))
    return r


portals = findPortals(plan)
portals.sort(key=lambda p: p.name)

G = nx.Graph()
G.add_nodes_from(portals)

for i in range(len(portals) - 1):
    initPortal = portals[i]
    edges = initPortal.findEdges(portals[i + 1 :], False)
    for portal, distance in edges:
        G.add_edge(initPortal, portal, length=distance)

part1 = nx.shortest_path_length(G, portals[0], portals[-1], weight="length")
print(part1)

G2 = nx.Graph()
G2.add_node((portals[0], 0))
nodes = [(portals[0], 0)]
while len(nodes) > 0 and not G2.has_node((portals[-1], 0)):
    node, level = nodes.pop(0)
    otherPortals = [n for n in portals if n != node]
    edges = node.findEdges(otherPortals, True)
    for portal, distance in edges:
        if level > 0 and (portal.name == "AA" or portal.name == "ZZ"):
            continue

        newPortal = None
        for p in portals:
            if p.name == portal.name and p != portal:
                newPortal = p
                break
        if newPortal == None:
            if portal.name == "AA":
                continue
            else:
                G2.add_node((portal, level))
                G2.add_edge((node, level), (portal, level), length=distance)
                continue

        newLevel = 0
        if portal.outside:
            newLevel = level - 1
        else:
            newLevel = level + 1
        if newLevel < 0 or newLevel > 30:
            continue

        if not G2.has_node((newPortal, newLevel)):
            G2.add_node((newPortal, newLevel))
            nodes.append((newPortal, newLevel))
        G2.add_edge((node, level), (newPortal, newLevel), length=distance + 1)

part2 = nx.shortest_path_length(G2, (portals[0], 0), (portals[-1], 0), weight="length")
print(part2)
