mod input;
mod heightmap;

use input::*;
use heightmap::*;

fn main() {

    let heightmap = HeightMap::new(INPUT.trim().lines().collect());
    let output: u32 = heightmap.get_low_points().into_iter().map(|h| (h+1) as u32).sum();

    println!("{}", output);

}
