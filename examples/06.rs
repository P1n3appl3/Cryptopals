use std::fs;

use cryptopals::*;

fn main() {
    let input = fs::read_to_string("input_data/6.txt").expect("couldn't read input file");
    let bytes = from_base64(&input);
    let mut keysizes = (2..=40)
        .map(|k| {
            (
                bytes
                    .chunks_exact(k)
                    .zip(bytes.chunks_exact(k).skip(1))
                    .map(|(a, b)| hamming_dist(a, b) as f32 / k as f32)
                    .sum::<f32>()
                    / (bytes.len() / k) as f32,
                k,
            )
        })
        .collect::<Vec<(f32, usize)>>();
    keysizes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // for keysize in &keysizes[..5] {
    let keysize = keysizes.first().unwrap();
    println!("keysize: {} (similarity: {})", keysize.1, keysize.0);
    let mut blocks = vec![Vec::new(); keysize.1];
    for i in 0..keysize.1 {
        for b in bytes.chunks_exact(keysize.1) {
            blocks[i].push(b[i]);
        }
    }
    let mut key = Vec::new();
    for block in blocks {
        let results = break_xor(&block);
        key.push(results.first().unwrap().1);
    }
    println!("Key: {}", String::from_utf8_lossy(&key));
    println!("decrypted:\n{}", String::from_utf8_lossy(&xor_encrypt(&bytes, &key)));
    println!();
    // }
}
