mod grid;
mod input;

use grid::OctoGrid;
use input::*;

fn main() {
    let mut grid = OctoGrid::new(INPUT.trim().lines());

    let output = (0..).skip_while(|_| !grid.update()).next().unwrap() + 1;

    println!("{}", output);
}
