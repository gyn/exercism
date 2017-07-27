#[derive(Debug)]
pub struct Brackets {
    brackets: String,
}

impl Brackets {
    pub fn from(text: &str) -> Self {
        Brackets {
            brackets: text.chars()
                .filter(|&x| {
                    x == '[' || x == ']' || x == '{' || x == '}' || x == '(' || x == ')'
                })
                .collect(),
        }
    }

    pub fn are_balanced(&self) -> bool {
        let mut r = Vec::new();

        for c in self.brackets.chars() {
            match c {
                '[' | '{' | '(' => r.push(c),
                ')' => if let Some(l) = r.pop() {
                    if l != '(' {
                        return false;
                    }
                } else {
                    return false;
                },
                ']' | '}' => if let Some(l) = r.pop() {
                    if c as i32 - l as i32 != 2 {
                        return false;
                    }
                } else {
                    return false;
                },
                _ => return false,
            }
        }

        r.is_empty()
    }
}
