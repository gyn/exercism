use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}

impl Rectangle {
    fn new(x: usize, y: usize, dx: usize, dy: usize) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            dx: dx,
            dy: dy,
        }
    }

    fn segments(&self) -> [usize; 4] {
        [self.x, self.y, self.dx, self.dy]
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Rectangle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Rectangle) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}


#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[inline]
fn get_corners(board: &[char], width: usize) -> Vec<Point> {
    board
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == '+' {
            Some(Point {
                x: i % width,
                y: i / width,
            })
        } else {
            None
        })
        .collect()
}

#[inline]
fn get_rectangles(corners: &[Point]) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();

    for i in 0..corners.len() {
        for j in i..corners.len() {
            let pa = &corners[i];
            let pb = &corners[j];

            if pa.x == pb.x || pa.y == pb.y {
                continue;
            }

            if !corners.contains(&Point { x: pa.x, y: pb.y }) ||
                !corners.contains(&Point { x: pb.x, y: pa.y })
            {
                continue;
            }

            let (x, dx) = if pa.x > pb.x {
                (pb.x, pa.x - pb.x)
            } else {
                (pa.x, pb.x - pa.x)
            };

            rectangles.push(Rectangle::new(x, pa.y, dx, pb.y - pa.y));
        }
    }

    rectangles
}

#[inline]
fn is_rectangle(rectangle: &Rectangle, board: &[char], width: usize) -> bool {
    let r = (1..rectangle.dx).any(|i| {
        let m = rectangle.y * width + rectangle.x + i;
        let n = (rectangle.y + rectangle.dy) * width + rectangle.x + i;
        (board[m] != '-' && board[m] != '+') || (board[n] != '-' && board[n] != '+')
    });

    if r {
        return false;
    }

    let r = (1..rectangle.dy).any(|i| {
        let m = (rectangle.y + i) * width + rectangle.x;
        let n = (rectangle.y + i) * width + rectangle.x + rectangle.dx;
        (board[m] != '|' && board[m] != '+') || (board[n] != '|' && board[n] != '+')
    });

    if r {
        return false;
    }

    true
}

pub fn count(text: &[&str]) -> usize {
    let height = text.len();
    if height == 0 {
        return 0;
    }

    let width = text[0].len();
    if width < 2 {
        return 0;
    }

    let board = text.iter().flat_map(|s| s.chars()).collect::<Vec<_>>();

    let corners = get_corners(&board, width);

    let mut rectangles = get_rectangles(&corners);

    rectangles.sort();
    rectangles.dedup();
    rectangles
        .iter()
        .filter(|x| is_rectangle(x, &board, width))
        .count()
}
