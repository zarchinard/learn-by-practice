from collections import defaultdict

lines = open("input.txt").readlines()
reactions = []
for r in lines:
    inputs, outputs = r.replace("\n", "").split(" => ")
    inputs = inputs.split(", ")
    outputs = outputs.split(", ")
    reaction = {"inputs": {}, "outputs": {}}
    for i in inputs:
        number, ingredient = i.split(" ")
        reaction["inputs"][ingredient] = int(number)
    for o in outputs:
        number, product = o.split(" ")
        reaction["outputs"][product] = int(number)
    reactions.append(reaction)


# def fuels(number):
#     ingredients = defaultdict(lambda: 0)
#     ingredients["FUEL"] = number
#     left = defaultdict(lambda: 0)
#     while True:
#         keys = [i for i in ingredients if ingredients[i] != 0]
#         if "ORE" in keys and len(keys) == 1:
#             break
#         k = keys[0] if keys[0] != "ORE" else keys[1]
#         v = ingredients[k]
#         if left[k] >= v:
#             left[k] = left[k] - v
#             ingredients[k] = 0
#             continue
#         for r in reactions:
#             if k in r["outputs"]:
#                 for i, vi in r["inputs"].items():
#                     ingredients[i] += vi
#                 for o, vo in r["outputs"].items():
#                     if o != k:
#                         left[o] = left[o] + vo
#                     else:
#                         left[o] += (vo - v) if vo > v else 0
#                         ingredients[o] = 0 if vo > v else v - vo
#                 break
#     return ingredients["ORE"]
def fuels(number):
    ingredients = defaultdict(lambda: 0)
    ingredients["FUEL"] = number
    left = defaultdict(lambda: 0)
    while True:
        keys = [i for i in ingredients if ingredients[i] != 0]
        if "ORE" in keys and len(keys) == 1:
            break
        k = keys[0] if keys[0] != "ORE" else keys[1]
        v = ingredients[k]
        if left[k] >= v:
            left[k] = left[k] - v
            ingredients[k] = 0
            continue
        for r in reactions:
            if k in r["outputs"]:
                m = max(v // r["outputs"][k], 1)
                for i, vi in r["inputs"].items():
                    ingredients[i] += m * vi
                for o, vo in r["outputs"].items():
                    if o != k:
                        left[o] += vo * m
                    else:
                        left[o] += (m * vo - v) if m * vo > v else 0
                        ingredients[o] = 0 if vo * m > v else v - m * vo
                break
    return ingredients["ORE"]


part1 = fuels(1)
print(part1)


start = 1000000000000 // part1
end = start
while fuels(end) < 1000000000000:
    end += start


def search(s, e):
    start = s
    end = e
    while end - start >= 2:
        mid = start + (end - start) // 2
        if fuels(mid) < 1000000000000:
            start = mid
        elif fuels(mid) > 1000000000000:
            end = mid
        else:
            return mid

    return start


print(search(start, end))

