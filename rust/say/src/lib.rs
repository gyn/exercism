const TEN: u64 = 10;
const HUNDRED: u64 = 100;
const THOUSAND: u64 = 1000;

fn ntable(number: u64) -> &'static str {
    match number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => "",
    }
}

fn translate(number: u64) -> String {
    let mut result = String::new();

    let hundreds = number / HUNDRED;
    let tens = number % HUNDRED;

    if hundreds != 0 {
        result = ntable(hundreds).to_string() + " hundred";

        if tens == 0 {
            return result;
        } else {
            result += " ";
        }
    }

    let digit = tens % TEN;

    if tens <= 20 || digit == 0 {
        result += ntable(tens);
    } else {
        result += ntable(tens - digit);
        result += "-";
        result += ntable(digit);
    }

    result
}

static UINTS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(number: u64) -> String {
    if number < THOUSAND {
        return translate(number);
    }

    let mut f = number;
    let mut d = Vec::new();

    while f > 0 {
        let r = f % THOUSAND;

        d.push(r);

        f /= THOUSAND;
    }

    d.into_iter()
        .map(translate)
        .zip(UINTS.into_iter())
        .filter_map(|(v, u)| {
            if v == "zero" {
                return None;
            }

            if u.is_empty() {
                Some(v)
            } else {
                Some(v + " " + u)
            }
        })
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}
