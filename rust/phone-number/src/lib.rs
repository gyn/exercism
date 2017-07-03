#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn number<S>(phone: S) -> Option<String> where S: Into<String> {
    let result = phone
        .into()
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>();

    match result.len() {
        11 if result.starts_with('1') => Some(result[1..].to_string()),
        10 => Some(result),
        _ => None,
    }
}

pub fn area_code<S>(phone: S) -> Option<String> where S: Into<String> {
    number(phone).map(|r| r[..3].to_string())
}

pub fn pretty_print<S>(phone: S) -> String where S: Into<String> {
    number(phone).map_or("invalid".to_string(), |r| {
        format!("({}) {}-{}", &r[..3], &r[3..6], &r[6..])
    })
}
