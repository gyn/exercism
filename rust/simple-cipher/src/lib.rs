extern crate rand;

use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    let mut v: Vec<u8> = Vec::new();
    for ch in key.chars() {
        if !ch.is_ascii_lowercase() {
            return None;
        }

        v.push(ch as u8);
    }

    let result = s
        .chars()
        .enumerate()
        .map(|(i, ch)| ((ch as u8 + v[i % v.len()] - 2 * b'a') % 26 + b'a') as char)
        .collect::<String>();

    Some(result)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    let mut v: Vec<u8> = Vec::new();
    for ch in key.chars() {
        if !ch.is_ascii_lowercase() {
            return None;
        }

        v.push(ch as u8);
    }

    let result = s
        .chars()
        .enumerate()
        .map(|(i, ch)| ((26 + ch as u8 - v[i % v.len()]) % 26 + b'a') as char)
        .collect::<String>();

    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = std::iter::repeat_with(|| rand::thread_rng().gen_range(b'a', b'z') as char)
        .take(std::cmp::max(100, s.len()))
        .collect::<String>();

    let result = encode(&key, s).unwrap();

    (key, result)
}
