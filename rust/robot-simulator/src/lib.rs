// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
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
            rd: match self.rd {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            rd: match self.rd {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        match self.rd {
            Direction::North => Robot{ry: self.ry + 1, ..self},
            Direction::East => Robot{rx: self.rx + 1, ..self},
            Direction::South => Robot{ry: self.ry - 1, ..self},
            Direction::West => Robot{rx: self.rx - 1, ..self},
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
