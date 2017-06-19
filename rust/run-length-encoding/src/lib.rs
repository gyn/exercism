pub fn encode(text: &str) -> String {
    if text.is_empty() {
        return "".to_string();
    }

    let mut last = text.chars().nth(0).unwrap();
    let mut count = 0;

    let mut result = String::new();

    for c in text.chars() {
        if c == last {
            count += 1;

            continue;
        }

        if count != 1 {
            result.push_str(&count.to_string());
        }

        result.push(last);

        last = c;
        count = 1;
    }

    // deal with the last one
    if count != 1 {
        result.push_str(&count.to_string());
    }
    result.push(last);

    result
}

pub fn decode(text: &str) -> String {
    let mut result = String::new();

    let mut count = 0;

    for c in text.chars() {
        if !c.is_numeric() {
            if count < 2 {
                result.push(c);
            } else {
                result.push_str(&std::iter::repeat(c).take(count).collect::<String>());
            }

            count = 0;

            continue;
        }

        count = count * 10 + c.to_digit(10).unwrap() as usize;
    }

    result
}
