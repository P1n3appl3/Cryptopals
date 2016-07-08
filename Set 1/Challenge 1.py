import binascii

hex2b64 = lambda s: binascii.unhexlify(s).encode("base64")

data = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"

print hex2b64(data)
