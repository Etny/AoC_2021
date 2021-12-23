use std::fmt::Display;



#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

pub struct Line {
    pub p1: Point,
    pub p2: Point
}

impl From<&str> for Point {
    fn from(dec: &str) -> Self {
        let mut nums = dec.split(',');
        let x = nums.next().unwrap().parse().unwrap();
        let y = nums.next().unwrap().parse().unwrap();

        Point{ x, y }
    }
}

impl From<&str> for Line {
    fn from(dec: &str) -> Self {
        let mut points = dec.split(" -> ");

        let p1 = points.next().unwrap().into(); 
        let p2 = points.next().unwrap().into();
        
        Line { p1, p2 }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.p1, self.p2)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}