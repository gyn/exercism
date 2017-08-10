#[derive(Debug)]
pub struct Brackets {
    string: String,
}

impl Brackets {
    pub fn from(text: &str) -> Self {
        Brackets {
            string: text.to_string(),
        }
    }

    pub fn are_balanced(&self) -> bool {
        let mut r = Vec::new();

        for c in self.string.chars() {
            match c {
                '[' | '{' | '(' => r.push(c),
                ']' | '}' | ')' => match (r.pop(), c) {
                    (Some('{'), '}') | (Some('['), ']') | (Some('('), ')') => {}
                    _ => return false,
                },
                _ => {}
            }
        }

        r.is_empty()
    }
}
