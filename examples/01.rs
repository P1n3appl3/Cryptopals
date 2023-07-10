use cryptopals::*;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e\
206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = from_hex(input);
    println!("{}", std::str::from_utf8(&bytes).unwrap());
    let b64 = to_base64(&bytes);
    println!("{b64}");
}
