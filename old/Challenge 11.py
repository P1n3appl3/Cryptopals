from Crypto.Cipher import AES
from Crypto.Util.Padding import pad
import random

encryptECB = lambda s, k: AES.new(k, AES.MODE_ECB).encrypt(s)

fixedXor = lambda a, b: bytes([a[i] ^ b[i] for i in range(len(a))])


def encryptCBC(s, k, iv):
    s = pad(s, 16)
    cipher = [iv]
    plain = [s[i : i + 16] for i in range(0, len(s), 16)]
    for p in plain:
        cipher.append(encryptECB(fixedXor(p, cipher[-1]), k))
    result = []
    for i in cipher[1:]:
        result += i
    return result


def detectCipher(s):
    temp = [s[i : i + 16] for i in range(0, len(s), 16)]
    for i in temp:
        if temp.count(i) > 1:
            return "ECB"
    return "CBC"


key = random.randbytes(16)
data = b"this is a test to see how well encryption works" * 20
enc = ""

data = (
    random.randbytes(random.randint(5, 10))
    + data
    + random.randbytes(random.randint(5, 10))
)

if random.random() < 0.5:
    enc = encryptECB(pad(data, 16), key)
    print("Actual: ECB")
else:
    enc = encryptCBC(data, key, random.randbytes(16))
    print("Actual: CBC")

print("Guess:", detectCipher(enc))
