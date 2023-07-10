use cryptopals::*;

fn main() {
    let input = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let key = "ICE";
    println!("{}", to_hex(&xor_encrypt(input.as_bytes(), key.as_bytes())));
}
