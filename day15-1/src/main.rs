mod input;
mod risk_map;
use input::*;
use risk_map::*;

fn main() {
    let map = RiskMap::new(INPUT.trim().lines());

    println!("{}", map.dijkstra());
}
