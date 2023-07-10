use cryptopals::*;

fn main() {
    let input =
        from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let results = break_xor(&input);

    // for best in &results[..5] {
    //     println!(
    //         r#"key {:0x}: "{}" (similarity: {})"#,
    //         best.1,
    //         String::from_utf8_lossy(&best.2),
    //         best.0
    //     )
    // }
    println!("{}", String::from_utf8_lossy(&results.first().unwrap().2));
}
