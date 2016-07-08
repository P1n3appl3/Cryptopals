from Crypto.Cipher import AES
import binascii

pad = lambda s, b: s + (b - len(s) % b) * chr(b - len(s) % b)

unpad = lambda s: s[:-ord(s[-1])]

encryptECB = lambda s, k: AES.new(k, AES.MODE_ECB).encrypt(pad(s, 16))

decryptECB = lambda s, k: unpad(AES.new(k, AES.MODE_ECB).decrypt(s))

ascii2b = lambda s: bytearray([ord(i) for i in s])

b2hex = lambda b: binascii.hexlify(b)

fixedXor = lambda a, b: bytearray([a[i] ^ b[i] for i in range(len(a))])

def encryptCBC(s, k, iv):
	cipher = [ascii2b(iv)]
	plain = [ascii2b(s[i:i + 16]) for i in range(0, len(s), 16)]
	for p in plain:
		cipher.append(encryptECB(b2hex(fixedXor(p, cipher[-1])), k))
	return b2hex(cipher[1:])

f = open("8.txt")
data = f.read()
f.close()

print pad(data, 20)