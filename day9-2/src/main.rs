mod heightmap;
mod input;

use heightmap::*;
use input::*;

fn main() {
    let heightmap = HeightMap::new(INPUT.trim().lines().collect());
    let output = heightmap.get_basins().reduce(|acc, f| acc*f).unwrap();

    println!("{}", output);
}
