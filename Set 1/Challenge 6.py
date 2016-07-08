import binascii

b642hex = lambda s: binascii.hexlify(s.decode("base64"))

ascii2b = lambda s: bytearray([ord(i) for i in s])

b2hex = lambda b: binascii.hexlify(b)

hex2b = lambda s: bytearray([int(s[i:i + 2], 16) for i in range(0, len(s), 2)])

fixedXor = lambda a, b: bytearray([a[i] ^ b[i] for i in range(len(a))])

singleCharXor = lambda s, c: bytearray([s[i] ^ c for i in range(len(s))])

printable = lambda s: not (False in [31 < ord(i) < 127 or ord(i) == 10 or ord(i) == 13 for i in s])


def xorCipher(s):
	englishLetters = [8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966, 0.153, 0.772, 4.025, 2.406, 6.749,
	                  7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.975, 0.074]
	space = 18.31
	b = hex2b(s)
	bestMatch = ''
	bestScore = 0
	key = 0
	for i in range(255):
		currentStr = binascii.unhexlify(b2hex(singleCharXor(b, i)))
		if printable(currentStr):
			score = 0
			temp = currentStr.lower()
			for c in temp:
				if 96 < ord(c) < 123:
					score += englishLetters[ord(c) - 97]
				elif ord(c) == 32:
					score += space
			if score > bestScore:
				bestScore = score
				bestMatch = currentStr
				key = i
	return bestMatch, bestScore, key


def decodeRepeatingXor(s, k):
	while len(k) < len(s):
		k += k
	return binascii.unhexlify(b2hex(fixedXor(hex2b(s), ascii2b(k))))


hamDist = lambda a, b: sum([bin(i).count('1') for i in fixedXor(a, b)])

f = open("6.txt")
data = hex2b(b642hex(f.read()))
f.close()
keySizes = []
for i in range(2, 40):
	dist = []
	for j in range(len(data) / 40):
		dist.append(hamDist(data[j * i:(j + 1) * i], data[(j + 1) * i:(j + 2) * i]) / float(i))
	avgDist = sum(dist) / len(dist)
	keySizes.append((avgDist, i))
keySizes.sort()

for n in range(3):
	key = []
	keySize = keySizes[n][1]
	blocks = [data[i::keySize] for i in range(keySize)]
	for b in blocks:
		key.append(xorCipher(b2hex(b))[2])
	key = ''.join([chr(i) for i in key])
	if printable(key[0]):
		print decodeRepeatingXor(b2hex(data), key)
