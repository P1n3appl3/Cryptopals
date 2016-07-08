import binascii

b2hex = lambda b: binascii.hexlify(b)

ascii2b = lambda s: bytearray([ord(i) for i in s])

fixedXor = lambda a, b: bytearray([a[i] ^ b[i] for i in range(len(a))])


def repeatingXor(s, k):
	while len(k) < len(s):
		k += k
	return b2hex(fixedXor(ascii2b(s), ascii2b(k)))


data = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
key = "ICE"

print repeatingXor(data, key)
