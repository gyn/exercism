pub fn reply(greeting: &str) -> &str {
    let mut contains_upper = false;
    let mut contains_lower = false;
    let mut last_nonspace = ' ';

    for c in greeting.chars() {
        if c.is_whitespace() {
            continue;
        }

        if c.is_alphabetic() {
            if c.is_uppercase() {
                contains_upper = true;
            } else {
                contains_lower = true;
            }
        }

        last_nonspace = c;
    }

    let is_shouting = contains_upper && !contains_lower;
    let is_asking = last_nonspace == '?';
    let is_addressing = last_nonspace == ' ';

    if is_shouting {
        return "Whoa, chill out!";
    }

    if is_asking {
        return "Sure.";
    }

    if is_addressing {
        return "Fine. Be that way!";
    }

    "Whatever."
}
