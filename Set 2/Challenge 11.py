from Crypto.Cipher import AES
import random
import itertools

pad = lambda s, b: s + (b - len(s) % b) * chr(b - len(s) % b)

unpad = lambda s: s[:-ord(s[-1])]

encryptECB = lambda s, k: AES.new(k, AES.MODE_ECB).encrypt(s)

fixedXor = lambda a, b: bytearray([a[i] ^ b[i] for i in range(len(a))])

def encryptCBC(s, k, iv):
	s = pad(s, 16)
	cipher = [bytearray(iv)]
	plain = [bytearray(s[i:i + 16]) for i in range(0, len(s), 16)]
	for p in plain:
		cipher.append(bytearray(encryptECB(str(fixedXor(p, cipher[-1])), k)))
	result = []
	for i in cipher[1:]:
		result += i
	return str(bytearray(result))

def detectCipher(s):
	temp = [s[i:i + 16] for i in range(0, len(s), 16)]
	for i in temp:
		if temp.count(i) > 1:
			return "ECB"
	return "CBC"

key = ''.join([chr(random.randint(0,255)) for i in range(16)])
data = b'this is a test to see how well encryption works' * 20
enc = ''

if random.random() < .5:
	enc = encryptECB(pad(random.randint(5,10)*chr(random.randint(0,255))+data+random.randint(5,10)*chr(random.randint(0,255)), 16), key)
	print "Actual: ECB"
else:
	enc = encryptCBC(random.randint(5,10)*chr(random.randint(0,255))+data+random.randint(5,10)*chr(random.randint(0,255)), key, 16*chr(random.randint(0,255)))
	print "Actual: CBC"

print "Guess:", detectCipher(enc)