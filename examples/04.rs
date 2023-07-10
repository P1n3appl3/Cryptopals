use std::fs;

use cryptopals::*;

fn main() {
    let input = fs::read_to_string("input_data/4.txt").expect("couldn't read input file");
    let mut results = Vec::new();
    for line in input.lines().map(from_hex) {
        results.extend(break_xor(&line));
    }
    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("{}", String::from_utf8_lossy(&results.first().unwrap().2));
}
