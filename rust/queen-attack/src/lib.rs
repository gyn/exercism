#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}


impl ChessPosition {
    pub fn new(m: i32, n: i32) -> Result<ChessPosition, &'static str> {
        if m < 0 || m > 7 {
            return Result::Err("Invalid value for m");
        }

        if n < 0 || n > 7 {
            return Result::Err("Invalid value for n");
        }

        Result::Ok(ChessPosition{x:m, y:n})
    }
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl Queen {
    pub fn new(x: ChessPosition) -> Self {
        Queen{pos: x}
    }

    pub fn can_attack(&self, q: &Queen) -> bool {
        let xdelta = self.pos.x - q.pos.x;
        let ydelta = self.pos.y - q.pos.y;

        xdelta == 0 || ydelta == 0 || xdelta.abs() == ydelta.abs()
    }
}
