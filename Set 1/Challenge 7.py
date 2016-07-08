from Crypto.Cipher import AES

unpad = lambda s: s[:-ord(s[-1])]

decryptECB = lambda s, k: unpad(AES.new(k, AES.MODE_ECB).decrypt(s))

f = open("7.txt")
data = f.read()
f.close()

key = "YELLOW SUBMARINE"

print decryptECB(data.decode("base64"), key)
