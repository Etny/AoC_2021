mod grid;
mod input;

use grid::OctoGrid;
use input::*;

fn main() {
    let mut grid = OctoGrid::new(INPUT.trim().lines());

    let output: u32 = (0..100).map(|_| grid.update()).sum();

    println!("{}", output);
}
