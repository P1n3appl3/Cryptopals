use std::fs;

use itertools::Itertools;

use cryptopals::*;

fn main() {
    let input = fs::read_to_string("input_data/8.txt").expect("couldn't read input file");
    for (i, line) in input.lines().map(from_hex).enumerate() {
        for pair in line.chunks(16).combinations(2) {
            let &[a, b] = pair.as_slice() else {unreachable!()};
            if a == b {
                println!("Match in line {i}: starts with {}", to_base64(&line[..8]))
            }
        }
    }
}
