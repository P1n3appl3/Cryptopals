#![feature(lazy_cell)]

use std::{fs, sync::LazyLock};

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use bitvec::prelude::*;
use itertools::Itertools;

pub type AesBlock = [u8; 16];
pub const BLOCK_SIZE: usize = 16;

pub fn from_hex(hex: &str) -> Vec<u8> {
    assert!(hex.chars().all(|n| n.is_ascii_hexdigit()));
    assert!(hex.len() % 2 == 0);
    hex.as_bytes()
        .chunks_exact(2)
        .map(|s| std::str::from_utf8(s).unwrap())
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect()
}

const HEX: [u8; 16] = *b"0123456789abcdef";
pub fn to_hex(bytes: &[u8]) -> String {
    let bits = bytes.view_bits::<Msb0>();
    bits.chunks_exact(4).map(|chunk| HEX[chunk.load_be::<usize>()] as char).collect()
}

const B64: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub fn to_base64(bytes: &[u8]) -> String {
    let bits = bytes.view_bits::<Msb0>();
    let chunks = bits.chunks_exact(6);
    let rem = chunks.remainder();
    let mut s: String = chunks.map(|chunk| B64[chunk.load_be::<usize>()] as char).collect();
    if !rem.is_empty() {
        let mut last = bitvec![u8, Msb0;];
        last.extend(rem);
        while last.len() < 6 {
            last.push(false);
        }
        s.push(B64[last.load_be::<usize>()] as char)
    }
    s
}

fn b64_char(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => panic!("invalid b64 character '{}'", c as char),
    }
}
pub fn from_base64(s: &str) -> Vec<u8> {
    assert!(s.bytes().all(|n| B64.contains(&n) || n == b'=' || n.is_ascii_whitespace()));
    let mut bits = bitvec![u8, Msb0;];
    for b in s.bytes() {
        if b == b'=' || b.is_ascii_whitespace() {
            continue;
        }
        bits.extend(&BitArray::<_, Msb0>::new(b64_char(b))[2..])
    }
    bits.chunks_exact(8).map(|chunk| chunk.load_be::<u8>()).collect()
}

pub fn byte_frequency(bytes: &[u8]) -> [f32; 256] {
    let mut len = 0;
    let mut result = [0f32; 256];
    for &b in bytes {
        result[b as usize] += 1.0;
        len += 1;
    }
    for n in result.iter_mut() {
        *n /= len as f32
    }
    result
}

static EXPECTED_FREQ: LazyLock<[f32; 256]> = LazyLock::new(|| {
    byte_frequency(
        &fs::read(
            option_env!("SAMPLE_TEXT")
                .expect("set the env var 'SAMPLE_TEXT=path/to/sample/text.txt'"),
        )
        .expect("couldn't read sample text file"),
    )
});

pub fn break_xor(bytes: &[u8]) -> Vec<(f32, u8, Vec<u8>)> {
    let mut results = Vec::new();
    for i in 0..=u8::MAX {
        let result: Vec<u8> = bytes.iter().map(|b| b ^ i).collect();
        let freq = byte_frequency(result.as_slice());
        let distance: f32 =
            freq.iter().zip(*EXPECTED_FREQ).map(|(a, b)| (a - b).abs()).sum();
        results.push((distance, i, result))
    }
    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    results
}

pub fn show_byte_frequency(freq: [f32; 256]) {
    for b in b'!'..=b'~' {
        println!("'{}': {:.2}", b as char, freq[b as usize] * 10.0)
    }
}

pub fn avg_word_len(bytes: &[u8]) -> f32 {
    let lengths: Vec<usize> =
        bytes.split(u8::is_ascii_whitespace).map(<[u8]>::len).collect();
    lengths.iter().sum::<usize>() as f32 / lengths.len() as f32
}

pub fn xor_encrypt(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes.iter().zip(key.iter().cycle()).map(|(a, b)| a ^ b).collect()
}

pub fn hamming_dist(a: &[u8], b: &[u8]) -> u32 {
    a.iter().zip(b).map(|(a, b)| (a ^ b).count_ones()).sum()
}

pub fn detect_aes(bytes: &[u8]) -> bool {
    for pair in bytes.chunks(BLOCK_SIZE).combinations(2) {
        let &[a, b] = pair.as_slice() else {unreachable!()};
        if a == b {
            return true;
        }
    }
    false
}

pub fn padded(bytes: &[u8], len: usize) -> Vec<u8> {
    let mut v = Vec::from(bytes);
    pad(&mut v, len);
    v
}

pub fn pad(bytes: &mut Vec<u8>, blocksize: usize) {
    let needed = blocksize - bytes.len() % blocksize;
    assert!(needed <= u8::MAX as usize);
    bytes.extend(std::iter::repeat(needed as u8).take(needed))
}

pub fn unpad(bytes: &[u8]) -> &[u8] {
    let Some(&padding_bytes) = bytes.last() else {return bytes};
    if bytes[bytes.len() - padding_bytes as usize..].iter().all(|&b| b == padding_bytes) {
        &bytes[..bytes.len() - padding_bytes as usize]
    } else {
        bytes
    }
}

pub fn ecb_decrypt(bytes: &mut [u8], key: AesBlock) {
    let cipher = Aes128::new(&GenericArray::from(key));
    for chunk in bytes.chunks_mut(BLOCK_SIZE) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.decrypt_block(block);
    }
}

pub fn ecb_encrypt(bytes: &mut [u8], key: AesBlock) {
    let cipher = Aes128::new(&GenericArray::from(key));
    for chunk in bytes.chunks_mut(BLOCK_SIZE) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block(block);
    }
}

pub fn cbc_decrypt(bytes: &mut [u8], key: AesBlock, iv: AesBlock) {
    let cipher = Aes128::new(&GenericArray::from(key));
    let mut prev = iv;
    for chunk in bytes.chunks_mut(BLOCK_SIZE) {
        let temp: [u8; BLOCK_SIZE] = chunk.try_into().unwrap();
        let block = GenericArray::from_mut_slice(chunk);
        cipher.decrypt_block(block);
        chunk.iter_mut().zip(prev).for_each(|(a, b)| *a ^= b);
        prev = temp;
    }
}

pub fn cbc_encrypt(bytes: &mut [u8], key: AesBlock, iv: AesBlock) {
    let cipher = Aes128::new(&GenericArray::from(key));
    let mut prev = iv;
    for chunk in bytes.chunks_mut(BLOCK_SIZE) {
        chunk.iter_mut().zip(prev).for_each(|(a, b)| *a ^= b);
        let block = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block(block);
        prev = chunk.try_into().unwrap();
    }
}

pub fn pprint(bytes: &[u8]) {
    let s: String = bytes
        .iter()
        .map(|b| match b {
            b if b.is_ascii_alphanumeric() || b.is_ascii_punctuation() => {
                (*b as char).to_string()
            }
            b'\n' => "⮒".to_owned(),
            b' ' => "·".to_owned(),
            b => format!("\\{{{b:0x}}}"),
        })
        .collect();
    println!("{} : {} bytes", s, bytes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        assert_eq!(from_hex("abad1dea"), [0xab, 0xad, 0x1d, 0xea]);
        assert_eq!(to_hex(&from_hex("abad1dea")), "abad1dea");
    }

    #[test]
    fn test_b64() {
        assert_eq!(to_base64(&[1, 2, 3]), "AQID");
        assert_eq!(from_base64(&to_base64(&[1, 2, 3])), [1, 2, 3]);
        assert_eq!(to_base64(b"j"), "ag");
        assert_eq!(from_base64(&to_base64(b"j")), b"j");
    }

    #[test]
    fn test_hamming() {
        assert_eq!(hamming_dist(b"this is a test", b"wokka wokka!!!"), 37);
    }

    #[test]
    fn test_unpad() {
        let bytes = [0, 0, 0, 0, 0, 0, 5];
        assert_eq!(unpad(&bytes), [0, 0]);
    }
}
