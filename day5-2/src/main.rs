mod input;
mod line;

use input::*;
use line::*;
use std::cmp;

fn main() {

    let mut lines = Vec::<Line>::new();
    
    for line_dec in INPUT.trim().lines() {
       lines.push(line_dec.into()); 
    }

    let width = lines.iter().fold(0, |max, l| cmp::max(max, cmp::max(l.p1.x, l.p2.x))) as usize + 1;
    let height = lines.iter().fold(0, |max, l| cmp::max(max, cmp::max(l.p1.y, l.p2.y))) as usize + 1;

    let mut map = vec![vec![0u32; height]; width];

    for line in &lines {
        apply_line(line, &mut map);
    }

    let danger_count = map.iter().flatten().filter(|p| **p > 1).count();

    println!("{}", danger_count);

}

fn apply_line(line: &Line, map: &mut Vec<Vec<u32>>) {
    let mut current = line.p1;

    loop {
        map[current.x as usize][current.y as usize] += 1;

        if current == line.p2 { break; }

        current.x = match current.x.cmp(&line.p2.x) {
            cmp::Ordering::Greater => current.x - 1,
            cmp::Ordering::Less => current.x + 1,
            _ => current.x
        };

        current.y = match current.y.cmp(&line.p2.y) {
            cmp::Ordering::Greater => current.y - 1,
            cmp::Ordering::Less => current.y + 1,
            _ => current.y
        };
    }
}
