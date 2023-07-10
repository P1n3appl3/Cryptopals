use cryptopals::*;

fn main() {
    let key = b"YELLOW SUBMARINE";
    let padded = padded(key, 20);
    dbg!(padded);
}
