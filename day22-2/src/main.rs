mod input;
use input::*;

fn main() {
    let mut chunks: Vec<Cube> = vec![];

    for line in INPUT.trim().lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let on = parts[0] == "on";

        let nums = parts[1].split(',').collect::<Vec<_>>();
        let x = parse_range(nums[0]);
        let y = parse_range(nums[1]);
        let z = parse_range(nums[2]);

        let cube = Cube::new([x.0, y.0, z.0], [x.1, y.1, z.1]);

        for c in &mut chunks {
            c.check_overlap(&cube);
        }

        if on {
            chunks.push(cube);
        }
    }

    println!("{}", chunks.into_iter().map(|c| c.volume()).sum::<i64>());
}

fn parse_range(dec: &str) -> (i32, i32) {
    let nums = dec[2..].split("..").collect::<Vec<_>>();

    let start: i32 = nums[0].parse().unwrap();
    let end: i32 = nums[1].parse().unwrap();

    (start.min(end), start.max(end))
}

#[derive(Clone)]
struct Cube {
    min: [i32; 3],
    max: [i32; 3],
    removed: Vec<Cube>,
}

impl Cube {
    fn new(min: [i32; 3], max: [i32; 3]) -> Self {
        Self {
            min,
            max,
            removed: vec![],
        }
    }

    fn volume(&self) -> i64 {
        let mut vol = 1;

        for i in 0..3 {
            vol *= (self.max[i] - self.min[i]) as i64 + 1;
        }

        for c in &self.removed {
            vol -= c.volume();
        }

        vol
    }

    fn check_overlap(&mut self, other: &Cube) {
        if let Some(remove) = self.overlapping(other) {
            let to_remove = self.removed.clone();
            self.removed.clear();

            for mut r in to_remove {
                r.check_overlap(&remove);
                if r.volume() > 0 {
                    self.removed.push(r);
                }
            }

            self.removed.push(remove);
        }
    }

    #[rustfmt::skip]
    fn overlapping(&self, other: &Cube) -> Option<Cube> {

        if  !(self.min[0] <= other.max[0] && self.max[0] >= other.min[0]) ||
            !(self.min[1] <= other.max[1] && self.max[1] >= other.min[1]) ||
            !(self.min[2] <= other.max[2] && self.max[2] >= other.min[2])
        {
            return None;
        }

        let mut min = [0, 0, 0];
        let mut max = [0, 0, 0];

        for i in 0..3 {
            min[i] = other.min[i].max(self.min[i]);
            max[i] = self.max[i].min(other.max[i]);
        }

        Some(Self { min, max, removed: vec![] })
    }
}
