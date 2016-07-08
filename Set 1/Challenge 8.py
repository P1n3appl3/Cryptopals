import itertools
from Crypto.Cipher import AES

unpad = lambda s: s[:-ord(s[-1])]

decryptECB = lambda s, k: unpad(AES.new(k, AES.MODE_ECB).decrypt(s))

f = open("8.txt")
data = f.read().splitlines()
f.close()
for d in data:
	temp = [d[i:i + 32] for i in range(0, len(d), 32)]
	for i in itertools.combinations(temp, 2):
		if i[0] == i[1]:
			print d
			break
