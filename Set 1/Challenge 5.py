import binascii

b2hex = lambda b: binascii.hexlify(b)

fixedXor = lambda a, b: bytearray([a[i] ^ b[i] for i in range(len(a))])


def repeatingXor(s, k):
	while len(k) < len(s):
		k += k
	return b2hex(fixedXor(bytearray(s), bytearray(k)))


data = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
key = "ICE"

print repeatingXor(data, key)
