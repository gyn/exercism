#[derive(Debug)]
pub struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    pub fn build(t: [usize; 3]) -> Result<Triangle, String> {
        println!("{:?} {:?} {:?} {:?}", t, t[0], t[1], t[2]);

        if t[0] == 0 || t[1] == 0 || t[2] == 0 {
            return Err("Invalid side length".to_string());
        }

        if t[0] > t[1] + t[2] || t[1] > t[0] + t[2] || t[2] > t[0] + t[1] {
            return Err("Invalid length pair".to_string());
        }

        Ok(Triangle {
            a: t[0],
            b: t[1],
            c: t[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c
    }

    pub fn is_isosceles(&self) -> bool {
        (self.a == self.b && self.a != self.c) ||
        (self.b == self.c && self.b != self.a) ||
        (self.c == self.a && self.c != self.b)
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.c != self.a
    }
}
