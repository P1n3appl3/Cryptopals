use std::{collections::HashMap, sync::OnceLock};

use rand::prelude::*;

use cryptopals::*;

fn parse_kv(bytes: &[u8]) -> HashMap<String, String> {
    let s = std::str::from_utf8(bytes).expect("invalid utf8");
    let mut result = HashMap::new();
    for pair in s.split('&') {
        let Some((k, v)) = pair.split_once('=') else { panic!("missing '='") };
        result.insert(k.to_owned(), v.to_owned());
    }
    result
}

fn profile_for(email: &[u8]) -> Vec<u8> {
    let sanitized_email: String = email
        .iter()
        .filter_map(|&c| {
            let c = c as char;
            (c != '=' && c != '&').then_some(c)
        })
        .collect();
    let uid: usize = thread_rng().gen_range(0..1000);
    format!("email={sanitized_email}&uid={uid}&role=user").into_bytes()
}

static KEY: OnceLock<AesBlock> = OnceLock::new();

fn encrypt(profile: &[u8]) -> Vec<u8> {
    let key = *KEY.get_or_init(random);
    let mut data = Vec::from(profile);
    pad(&mut data, BLOCK_SIZE);
    ecb_encrypt(&mut data, key);
    data
}

fn decrypt(bytes: &[u8]) -> Vec<u8> {
    let key = *KEY.get_or_init(random);
    let mut data = Vec::from(bytes);
    ecb_decrypt(&mut data, key);
    unpad(&data).to_vec()
}

fn main() {
    let profile = profile_for(b"foo@bar.com");
    pprint(&profile);
    assert_eq!(profile, decrypt(&encrypt(&profile)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_parsing() {
        let input = "foo=bar&baz=qux&zap=zazzle";
        let expected = [("foo", "bar"), ("baz", "qux"), ("zap", "zazzle")]
            .into_iter()
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect();
        assert_eq!(parse_kv(input), expected);
    }
}
