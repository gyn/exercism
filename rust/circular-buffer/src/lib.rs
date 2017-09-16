use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            buffer: VecDeque::new(),
            size: size,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.buffer.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn write(&mut self, v: T) -> Result<usize, Error> {
        if self.buffer.len() == self.size {
            return Err(Error::FullBuffer);
        }

        self.buffer.push_back(v);

        Ok(self.buffer.len())
    }

    pub fn overwrite(&mut self, v: T) {
        if self.buffer.len() == self.size {
            self.buffer.pop_front().unwrap();
        }

        self.buffer.push_back(v);
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}
