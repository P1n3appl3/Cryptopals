hex2b = lambda s: bytes(int(s[i : i + 2], 16) for i in range(0, len(s), 2))

fixedXor = lambda a, b: bytes([a[i] ^ b[i] for i in range(len(a))])

a = "1c0111001f010100061a024b53535009181c"
b = "686974207468652062756c6c277320657965"

print(fixedXor(hex2b(a), hex2b(b)))
