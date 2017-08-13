pub trait LuhnTrait {
    fn valid_luhn(&self) -> bool;
}

impl<U> LuhnTrait for U
where
    U: ToString,
{
    fn valid_luhn(&self) -> bool {
        is_valid(&self.to_string())
    }
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
