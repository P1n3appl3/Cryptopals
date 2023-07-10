use cryptopals::*;

fn main() {
    let a = from_hex("1c0111001f010100061a024b53535009181c");
    let b = from_hex("686974207468652062756c6c277320657965");
    let result: Vec<u8> = a.iter().zip(b).map(|(a, b)| a ^ b).collect();
    println!("{}", to_hex(&result));
    println!("{}", String::from_utf8_lossy(&result));
}
