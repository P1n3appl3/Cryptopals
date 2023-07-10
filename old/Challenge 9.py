pad = lambda s, b: s + (b - len(s) % b) * chr(b - len(s) % b)

data = "YELLOW SUBMARINE"

print(pad(data, 20))
