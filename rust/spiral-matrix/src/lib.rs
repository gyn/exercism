//
// This is smart but slow
//
//fn spiral(width: u32, height: u32, x: u32, y: u32) -> u32 {
//    if y == 0 {
//        x + 1
//    } else {
//        width + spiral(height - 1, width, y - 1, width - x - 1)
//    }
//}
//
//pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
//    (0..size)
//        .map(|y| (0..size).map(|x| spiral(size, size, x, y)).collect())
//        .collect()
//}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }

    let mut result = vec![vec![0; size as usize]; size as usize];

    let mut left = 0usize;
    let mut top = 0usize;
    let mut right = size as usize;
    let mut bottom = size as usize;

    let mut value = 1;

    while left < right && top < bottom {
        // fill top line
        for x in left..right {
            result[top][x] = value;

            value += 1;
        }
        top += 1;

        // fill right line
        for y in top..bottom {
            result[y][right - 1] = value;

            value += 1;
        }
        right -= 1;

        // fill bottom lime
        for x in (left..right).rev() {
            result[bottom - 1][x] = value;

            value += 1;
        }
        bottom -= 1;

        // fill left line
        for y in (top..bottom).rev() {
            result[y][left] = value;

            value += 1;
        }
        left += 1;
    }

    result
}
