use rand::prelude::*;

use cryptopals::*;

fn oracle(bytes: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();
    (0..thread_rng().gen_range(5..=10)).for_each(|_| data.push(random()));
    data.extend(bytes);
    (0..thread_rng().gen_range(5..=10)).for_each(|_| data.push(random()));
    pad_for_aes(&mut data);
    let mut key = [0; 16];
    thread_rng().fill(&mut key);
    if random() {
        ecb_encrypt(&mut data, key);
        println!("Actual: ECB");
    } else {
        let mut iv = [0; 16];
        thread_rng().fill(&mut iv);
        cbc_encrypt(&mut data, key, iv);
        println!("Actual: CBC");
    }
    data
}

fn main() {
    let bytes = b"0123456789abcdef0123456789abcdef0123456789abcdef";
    for _ in 0..10 {
        let data = oracle(bytes);
        println!(
            "Guess: {}",
            if (0..16).filter(|i| data[11 + i] == data[11 + i + 16]).count() == 11 {
                "ECB"
            } else {
                "CBC"
            }
        );
        println!("Data: {}\n", to_base64(&data));
    }
}
