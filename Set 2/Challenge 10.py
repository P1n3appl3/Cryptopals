from Crypto.Cipher import AES
import binascii

pad = lambda s, b: s + (b - len(s) % b) * chr(b - len(s) % b)

unpad = lambda s: s[:-ord(s[-1])]

encryptECB = lambda s, k: AES.new(k, AES.MODE_ECB).encrypt(s)

decryptECB = lambda s, k: AES.new(k, AES.MODE_ECB).decrypt(s)

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

def decryptCBC(s, k, iv):
	cipher = [s[i:i+16] for i in range(0, len(s), 16)]
	plain = fixedXor(bytearray(decryptECB(cipher[0], k)), bytearray(iv))
	for i in range(1, len(cipher)):
		plain += fixedXor(bytearray(decryptECB(cipher[i], k)), bytearray(cipher[i-1]))
	return unpad(str(plain))

f = open("10.txt")
data = f.read()
f.close()

print decryptCBC(data.decode("base64"), "YELLOW SUBMARINE", 16 * chr(0))
