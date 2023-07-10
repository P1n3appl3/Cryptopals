use std::fs;

use cryptopals::*;

fn main() {
    let input = fs::read_to_string("input_data/10.txt").expect("couldn't read input file");
    let mut bytes = from_base64(&input);
    let key = b"YELLOW SUBMARINE";
    let iv = [0; 16];
    cbc_decrypt(&mut bytes, *key, iv);
    println!("{}", String::from_utf8_lossy(&bytes));
    // cbc_encrypt(&mut bytes, *key, iv);
    // assert_eq!(from_base64(&input), bytes)
}
