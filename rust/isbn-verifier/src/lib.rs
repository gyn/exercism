#![feature(ascii_ctype)]

pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut iter = isbn.chars().peekable();

    if let Some(&c) = iter.peek() {
        if !c.is_ascii_digit() {
            return false;
        }
    }

    let mut index = 0;
    let mut sum = 0;
    let mut acc = 0;
    const X_POSTION: usize = 9;
    const MAX_LENGTH: usize = 10;

    while let Some(&c) = iter.peek() {
        match c {
            '-' => {
                iter.next();
            }
            _ if c.is_ascii_digit() => {
                index += 1;
                acc += c as usize - '0' as usize;
                sum += acc;

                iter.next();
            }
            'X' if index == X_POSTION => {
                index += 1;
                acc += 10;
                sum += acc;

                iter.next();
            }
            _ => return false,
        }

        if index > MAX_LENGTH {
            return false;
        }
    }

    sum % 11 == 0
}
