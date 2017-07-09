// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

pub struct Robot {
    rx: isize,
    ry: isize,
    rd: Direction,
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            rx: x,
            ry: y,
            rd: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            rx: self.rx,
            ry: self.ry,
            rd: self.rd.turn_right(),
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            rx: self.rx,
            ry: self.ry,
            rd: self.rd.turn_left(),
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.rd {
            Direction::North => (self.rx, self.ry + 1),
            Direction::East => (self.rx + 1, self.ry),
            Direction::South => (self.rx, self.ry - 1),
            Direction::West => (self.rx - 1, self.ry),
        };

        Robot {
            rx: x,
            ry: y,
            rd: self.rd,
        }
    }

    #[allow(unused_variables)]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut r = self;
        for c in instructions.chars() {
            r = match c {
                'L' => r.turn_left(),
                'R' => r.turn_right(),
                'A' => r.advance(),
                _ => r,
            };
        }

        r
    }

    pub fn position(&self) -> (isize, isize) {
        (self.rx, self.ry)
    }

    pub fn direction(&self) -> &Direction {
        &self.rd
    }
}
