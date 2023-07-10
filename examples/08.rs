use std::fs;

use cryptopals::*;

fn main() {
    let input = fs::read_to_string("input_data/8.txt").expect("couldn't read input file");
    for (i, line) in input.lines().map(from_hex).enumerate() {
        if detect_aes(&line) {
            println!("line {i} was AES encrypted: starts with {}", to_base64(&line[..8]))
        }
    }
}
