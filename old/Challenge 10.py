from Crypto.Cipher import AES
import base64

pad = lambda s, b: s + (b - len(s) % b) * chr(b - len(s) % b)

unpad = lambda s: s[: -s[-1]]

encryptECB = lambda s, k: AES.new(k, AES.MODE_ECB).encrypt(s)

decryptECB = lambda s, k: AES.new(k, AES.MODE_ECB).decrypt(s)

fixedXor = lambda a, b: bytes([a[i] ^ b[i] for i in range(len(a))])


def encryptCBC(s, k, iv):
    s = pad(s, 16)
    cipher = [bytes(iv)]
    plain = [bytes(s[i : i + 16]) for i in range(0, len(s), 16)]
    for p in plain:
        cipher.append(bytes(encryptECB(fixedXor(p, cipher[-1]), k)))
    result = []
    for i in cipher[1:]:
        result += i
    return bytes(result)


def decryptCBC(s, k, iv):
    cipher = [s[i : i + 16] for i in range(0, len(s), 16)]
    plain = fixedXor(bytes(decryptECB(cipher[0], k)), bytes(iv))
    for i in range(1, len(cipher)):
        plain += fixedXor(bytes(decryptECB(cipher[i], k)), bytes(cipher[i - 1]))
    return unpad(plain)


f = open("input_data/10.txt")
data = f.read()
f.close()

print(decryptCBC(base64.b64decode(data), b"YELLOW SUBMARINE", 16 * b"0").decode())
