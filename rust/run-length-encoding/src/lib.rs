pub fn encode(text: &str) -> String {

    let mut count = 0;
    let mut result = String::new();
    let mut iter = text.chars().peekable();

    while let Some(c) = iter.next() {
        count += 1;

        if iter.peek() != Some(&c) {
            if count > 1 {
                result.push_str(&count.to_string());
            }

            result.push(c);

            count = 0;
        }
    }

    result
}

pub fn decode(text: &str) -> String {
    let mut count = 0;
    let mut result = String::new();

    for c in text.chars() {
        if let Some(v) = c.to_digit(10) {
            count = count * 10 + v as usize;

            continue;
        }

        if count < 2 {
            result.push(c);
        } else {
            let s = std::iter::repeat(c).take(count).collect::<String>();

            result.push_str(&s);
        }

        count = 0;
    }

    result
}
