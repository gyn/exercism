#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

fn matrix(length: usize) -> (usize, usize) {
    let s = (length as f64).sqrt().floor() as usize;

    if s * s == length {
        return (s, s);
    }

    if (s + 1) * s >= length {
        return (s + 1, s);
    }

    (s + 1, s + 1)
}

pub fn encrypt(text: &str) -> String {
    let v = text.chars()
        .filter_map(|x| match x {
            _ if x.is_ascii_alphabetic() => Some(x.to_ascii_lowercase()),
            _ => None,
        })
        .collect::<Vec<_>>();

    let (rows, columns) = matrix(v.len());

    let mut result = String::new();

    for i in 0..rows {
        for j in 0..columns {
            let p = j * rows + i;

            if p < v.len() {
                result.push(v[p]);
            }
        }

        if i != rows - 1 {
            result.push(' ');
        }
    }

    result
}
