import itertools

f = open("input_data/8.txt")
data = f.read().splitlines()
f.close()
for d in data:
    temp = [d[i : i + 32] for i in range(0, len(d), 32)]
    for i in itertools.combinations(temp, 2):
        if i[0] == i[1]:
            print(d)
            break
