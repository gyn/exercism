pub struct Luhn {
    result: Option<String>,
}

#[inline]
fn is_valid(id: &str) -> bool {
    let mut index: u32 = 0;
    let mut sum: u32 = 0;

    for c in id.chars().rev() {
        if c.is_whitespace() {
            continue;
        }

        let mut v = match c.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        if index & 1 != 0 {
            v *= 2;

            if v > 9 {
                v -= 9;
            }
        }

        index += 1;
        sum += v;
    }

    index >= 2 && sum % 10 == 0
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
