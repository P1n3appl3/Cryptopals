use std::{mem, sync::OnceLock};

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

fn detect_block_size() -> usize {
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
            return i - start;
        }
        i += 1;
    }
}

fn main() {
    let block_size = dbg!(detect_block_size());
    assert!(detect_aes(&vec![b'A'; block_size * 3]));
    let mut decoded = Vec::new();
    for i in 1..100 {
        let skip_blocks = i / block_size;
        let idx = i % block_size;
        let one_short = &oracle(&vec![b'A'; block_size * skip_blocks + block_size - idx])
            [skip_blocks * block_size..(skip_blocks + 1) * block_size];
        for b in 0..=u8::MAX {
            let mut input = vec![b'A'; block_size];
            input[block_size - idx] = b;
            let output = oracle(&input);
            if &output[skip_blocks * block_size..(skip_blocks + 1) * block_size]
                == one_short
            {
                decoded.push(b);
                println!("first byte: {b:#0x}");
                break;
            }
        }
    }
    println!("{}", String::from_utf8_lossy(&decoded));
}
