f = open("input6.txt", "r")
data = []
for line in f.readlines():
    data.append(line.replace("\n", ""))


class Tree:
    def __init__(self, value):
        self.parent = None
        self.children = []
        self.value = value
        self.level = None

    def setParent(self, parent):
        self.parent = parent
        parent.children.append(self)
        self.level = parent.level + 1


def buildTree(data):
    root = Tree("COM")
    root.level = 0
    nodes = [root]
    for o in data:
        parentValue, childValue = o.split(")")
        parent = [n for n in nodes if n.value == parentValue]
        if len(parent) == 0:
            data.append(o)
            continue
        parent = parent[0]
        child = Tree(childValue)
        child.setParent(parent)
        nodes.append(child)
    return nodes


tree = buildTree(data)
# Part 1
print(sum(map(lambda n: n.level, tree)))

# Part 2
def findParents(n):
    r = []
    i = n
    while i.parent != None:
        r.append(i.parent)
        i = i.parent
    return r


you = [n for n in tree if n.value == "YOU"][0]
youLevel = you.level
youParents = findParents(you)
san = [n for n in tree if n.value == "SAN"][0]
sanLevel = san.level
sanParents = findParents(san)
parentLevel = 0
for p in set(youParents).intersection(sanParents):
    if p.level > parentLevel:
        parentLevel = p.level
print(youLevel - parentLevel + sanLevel - parentLevel - 2)


with open("input6.txt", "r") as f:
    parents = dict(reversed(orbit.split(")")) for orbit in f.read().splitlines())
print(parents)
# Part 1:
# dist_to_root = lambda n: 1 + dist_to_root(parents[n]) if n in parents else 0
# print(sum(dist_to_root(n) for n in parents))

# Part 2:
# ancestors = (
#    lambda n: ancestors(parents[n]).union([parents[n]]) if n in parents else set()
# )
# print(len(ancestors("YOU") ^ ancestors("SAN")))

