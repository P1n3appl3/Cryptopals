from Crypto.Cipher import AES
import base64

unpad = lambda s: s[: -s[-1]]

decryptECB = lambda s, k: unpad(AES.new(k, AES.MODE_ECB).decrypt(s))

f = open("input_data/7.txt")
data = f.read()
f.close()

key = b"YELLOW SUBMARINE"

result = decryptECB(base64.b64decode(data), key)
print(decryptECB(base64.b64decode(data), key).decode())
