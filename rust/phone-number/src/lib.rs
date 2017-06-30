#![feature(ascii_ctype)]
use std::ascii::AsciiExt;


fn check_countrycode(mut phone: Vec<u8>) -> Option<String> {
    println!("{:?}", phone);

    if phone[0] != b'1' {
        return None;
    }

    String::from_utf8(phone.drain(1..).collect()).ok()
}

pub fn number(phone: &str) -> Option<String> {
    let result = phone
        .chars()
        .filter(|x| x.is_ascii_digit())
        .map(|x| x as u8)
        .collect::<Vec<u8>>();

    match result.len() {
        11 => check_countrycode(result),
        10 => String::from_utf8(result).ok(),
        _ => None,
    }
}

pub fn area_code(phone: &str) -> Option<String> {
    number(phone).map(|r| r[..3].to_string())
}

pub fn pretty_print(phone: &str) -> String {
    let v = number(phone);

    if v.is_none() {
        return "invalid".to_string();
    }

    let r = v.unwrap();

    format!("({}) {}-{}", &r[..3], &r[3..6], &r[6..])
}
