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

impl Line {

    pub fn is_diagonal(&self) -> bool {
        self.p1.x != self.p2.x && self.p1.y != self.p2.y
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
