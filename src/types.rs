#[derive(Clone, Debug, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
