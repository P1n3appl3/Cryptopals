use std::fs;

use cryptopals::*;

fn main() {
    let input = fs::read_to_string("input_data/7.txt").expect("couldn't read input file");
    let mut bytes = from_base64(&input);
    let key = b"YELLOW SUBMARINE";
    ecb_decrypt(&mut bytes, *key);
    println!("{}", String::from_utf8_lossy(&bytes));
    // ecb_encrypt(&mut bytes, *key);
    // assert_eq!(from_base64(&input), bytes)
}
