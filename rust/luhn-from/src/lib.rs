pub struct Luhn {
    result: Option<String>,
}

fn is_valid(id: &str) -> bool {
    if id.len() < 2 {
        return false;
    }

    let mut index: u32 = 0;
    let mut sum: u32 = 0;

    for c in id.chars().rev() {
        if c.is_whitespace() {
            continue;
        }

        let mut v: u32;

        if let Some(d) = c.to_digit(10) {
            v = d;
        } else {
            return false;
        }

        if index % 2 != 0 {
            v *= 2;

            if v > 9 {
                v -= 9;
            }
        }

        index += 1;
        sum += v;
    }

    if index < 2 {
        return false;
    }

    (sum % 10) == 0
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(id: T) -> Self {
        let string = id.to_string();

        if is_valid(&string) {
            return Luhn {
                result: Some(string),
            };
        }

        Luhn { result: None }
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.result.is_some()
    }
}
