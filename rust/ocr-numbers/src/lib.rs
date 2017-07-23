#![feature(str_checked_slicing)]

const DIGITS: [&str; 10] = [
    " _ | ||_|   ", // 0
    "     |  |   ", // 1
    " _  _||_    ", // 2
    " _  _| _|   ", // 3
    "   |_|  |   ", // 4
    " _ |_  _|   ", // 5
    " _ |_ |_|   ", // 6
    " _   |  |   ", // 7
    " _ |_||_|   ", // 8
    " _ |_| _|   ", // 9
];

const DIGITS_ROWS: usize = 3;
const DIGITS_LINES: usize = 4;

pub fn convert(input: &str) -> Result<String, String> {
    let v = input.split(|x| x == '\n').collect::<Vec<_>>();

    let lines = v.len();
    if lines == 0 || lines % DIGITS_LINES != 0 {
        return Err("Invalid lines".to_string());
    }

    let rows = v[0].len();
    if rows == 0 || rows % DIGITS_ROWS != 0 {
        return Err(format!("Invalid {} row at line 1", rows));
    }
    if v.iter().any(|x| x.len() != rows) {
        return Err("Invalid rows".to_string());
    }

    let mut result = String::new();
    for i in 0..lines / DIGITS_LINES {
        for j in 0..rows / DIGITS_ROWS {
            let row = j * DIGITS_ROWS;
            let line = i * DIGITS_LINES;

            let number = format!(
                "{}{}{}{}",
                v[line].get(row..row + DIGITS_ROWS).unwrap(),
                v[line + 1].get(row..row + DIGITS_ROWS).unwrap(),
                v[line + 2].get(row..row + DIGITS_ROWS).unwrap(),
                v[line + 3].get(row..row + DIGITS_ROWS).unwrap()
            );

            let index: u8 = DIGITS
                .iter()
                .position(|&x| x == number)
                .map_or_else(|| b'?', |v| v as u8 + b'0');

            result.push(index as char);
        }

        result.push(',');
    }

    result.pop();

    Ok(result)
}
