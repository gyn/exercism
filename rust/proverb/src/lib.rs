pub fn build_proverb(text: Vec<&str>) -> String {
    let mut result = String::new();
    let mut iter = text.iter().peekable();

    let last = if text.len() < 3 { "" } else { "horseshoe " };

    while let Some(c) = iter.next() {
        if let Some(n) = iter.peek() {
            result += &format!("For want of a {} the {} was lost.\n", c, n);
        } else {
            result += &format!("And all for the want of a {}nail.", last);
        }
    }

    result
}
