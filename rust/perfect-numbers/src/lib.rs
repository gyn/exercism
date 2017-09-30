#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn classify(number: u64) -> Result<Classification, &'static str> {
    if number < 1 {
        return Err("Number must be positive");
    }

    let sum: u64 = (1..number / 2 + 1).filter(|x| number % x == 0).sum();

    if sum == number {
        Ok(Classification::Perfect)
    } else if sum > number {
        Ok(Classification::Abundant)
    } else {
        Ok(Classification::Deficient)
    }
}
