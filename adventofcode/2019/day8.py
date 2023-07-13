data = open("input8.txt", "r").readline()
wide = 25
tall = 6

layers = []
for i in range(len(data) // (wide * tall)):
    layers.append(data[i * (wide * tall) : (i + 1) * (wide * tall)])

# Part 1
sortedLayers = layers.copy()
sortedLayers.sort(key=lambda l: l.count("0"))
layerFewZero = sortedLayers[0]
print(layerFewZero.count("1") * layerFewZero.count("2"))

# Part 2
layer = list(layers.pop(0))
for l in layers:
    for i in range(len(l)):
        if l[i] != "2" and layer[i] == "2":
            layer[i] = l[i]
layer = "".join(layer).replace("0", " ")
for i in range(tall):
    print(layer[i * wide : (i + 1) * wide])
