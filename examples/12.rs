use std::sync::OnceLock;

use kdam::tqdm;
use rand::prelude::*;

use cryptopals::*;

fn oracle(bytes: &[u8]) -> Vec<u8> {
    static KEY: OnceLock<AesBlock> = OnceLock::new();
    let key = *KEY.get_or_init(random);
    let secret = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
                  aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
                  dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
                  YnkK";
    let mut data = Vec::from(bytes);
    data.extend(from_base64(secret));
    pad(&mut data, BLOCK_SIZE);
    ecb_encrypt(&mut data, key);
    data
}

fn len_and_block_size() -> (usize, usize) {
    let mut i = 2;
    let prev_len = oracle(b"A").len();
    let first_len;
    let start = loop {
        let input = vec![b'A'; i];
        let len = oracle(&input).len();
        if len != prev_len {
            first_len = len;
            break i;
        }
        i += 1;
    };
    loop {
        let input = vec![b'A'; i];
        let len = oracle(&input).len();
        if len != first_len {
            let block_size = i - start;
            return (oracle(&[]).len() - start, block_size);
        }
        i += 1;
    }
}

fn main() {
    let (secret_size, block_size) = len_and_block_size();
    assert!(detect_aes(&vec![b'A'; block_size * 3]));
    let mut decoded = vec![b'A'; dbg!(block_size)];
    let mut probe = decoded.clone();
    for n in tqdm!(0..dbg!(secret_size)) {
        let b = n / block_size;
        let i = n % block_size;
        if i == 0 {
            probe = decoded.rchunks(block_size).next().unwrap().to_vec();
        }
        let one_short = &oracle(&probe[i + 1..])[b * block_size..(b + 1) * block_size];
        for byte in 0..=u8::MAX {
            let mut input = decoded.rchunks(block_size - 1).next().unwrap().to_vec();
            input.push(byte);
            if &oracle(&input)[..block_size] == one_short {
                decoded.push(byte);
                break;
            }
        }
        if decoded.len() != n + block_size + 1 {
            panic!("didn't add a byte");
        }
    }
    println!("{}", String::from_utf8_lossy(&decoded[block_size..]));
}
