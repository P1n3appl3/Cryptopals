use cryptopals::*;

fn main() {
    let key = b"YELLOW SUBMARINE";
    let padded = pad_pkcs(key, 20);
    dbg!(padded);
}
