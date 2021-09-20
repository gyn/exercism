fn spiral_index(x: i32, y: i32) -> i32 {
    let mut p: i32;

    if y * y >= x * x {
        p = 4 * y * y - y - x;
        if y < x {
            p -= 2 * (y - x);
        }
    } else {
        p = 4 * x * x - y - x;
        if y < x {
            p += 2 * (y - x);
        }
    }

    p
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result = Vec::new();

    if size == 0 {
        return result;
    }

    let width: i32 = size as i32;

    let sx = if width % 2 == 1 {
        width / 2
    } else {
        (1 - width) / 2
    };
    let ex = if width % 2 == 1 {
        (1 - width) / 2 - 1
    } else {
        width / 2 + 1
    };
    let dx = if width % 2 == 1 { -1 } else { 1 };
    let sy = if width % 2 == 1 {
        (1 - width) / 2
    } else {
        width / 2
    };
    let ey = if width % 2 == 1 {
        width / 2 + 1
    } else {
        (1 - width) / 2 - 1
    };
    let dy = if width % 2 == 1 { 1 } else { -1 };

    let mut y = sy;
    while y != ey {
        let mut v = Vec::new();

        let mut x = sx;
        while x != ex {
            let r = width * width - spiral_index(x, y);
            v.push(r as u32);
            x += dx;
        }

        result.push(v);
        y += dy;
    }

    result
}
