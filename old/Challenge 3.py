import binascii

b2hex = lambda b: binascii.hexlify(b)

hex2b = lambda s: bytes([int(s[i : i + 2], 16) for i in range(0, len(s), 2)])

singleCharXor = lambda s, c: bytes([s[i] ^ c for i in range(len(s))])

printable = lambda s: not (False in [31 < i < 127 or i == 10 or i == 13 for i in s])


def xorCipher(s):
    # fmt: off
    englishLetters = [
        8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966,
        0.153, 0.772, 4.025, 2.406, 6.749, 7.507, 1.929, 0.095, 5.987,
        6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.975, 0.074,
    ]  # fmt: on
    space = 18.31
    b = hex2b(s)
    bestMatch = ""
    bestScore = 0
    key = 0
    for i in range(255):
        currentStr = binascii.unhexlify(b2hex(singleCharXor(b, i)))
        if printable(currentStr):
            score = 0
            temp = currentStr.lower()
            for c in temp:
                if 96 < c < 123:
                    score += englishLetters[c - 97]
                elif c == 32:
                    score += space
            if score > bestScore:
                bestScore = score
                bestMatch = currentStr
                key = i
    return bestMatch, bestScore, key


data = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"

print(xorCipher(data)[0])
