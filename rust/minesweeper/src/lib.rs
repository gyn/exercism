#[inline]
fn neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if x != 0 && y != 0 {
        result.push((x - 1, y - 1));
    }

    if x != width - 1 && y != height - 1 {
        result.push((x + 1, y + 1));
    }

    if x != 0 && y != height - 1 {
        result.push((x - 1, y + 1));
    }

    if y != 0 && x != width - 1 {
        result.push((x + 1, y - 1));
    }

    if x != 0 {
        result.push((x - 1, y));
    }

    if y != 0 {
        result.push((x, y - 1));
    }

    if x != width - 1 {
        result.push((x + 1, y));
    }

    if y != height - 1 {
        result.push((x, y + 1));
    }

    result
}

pub fn annotate(text: &[&str]) -> Vec<String> {
    let height = text.len();
    if height == 0 {
        return Vec::new();
    }

    let width = text[0].len();
    if width == 0 {
        return vec![String::new()];
    }

    let mut v = Vec::new();

    for i in text {
        v.push(i.chars().collect::<Vec<_>>());
    }

    let mut result = Vec::new();

    for (y, vec) in v.iter().enumerate() {
        let mut s = String::new();

        for (x, &c) in vec.iter().enumerate() {
            if c == '*' {
                s.push('*');

                continue;
            }

            let r = neighbors(x, y, width, height)
                .iter()
                .filter(|&&(m, n)| v[n][m] == '*')
                .count();
            if r != 0 {
                s.push((r as u8 + b'0') as char);
            } else {
                s.push(' ');
            }
        }

        result.push(s);
    }

    result
}
