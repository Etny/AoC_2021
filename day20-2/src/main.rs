mod input;
mod grid;

use grid::*;
use input::*;

//Really happy with this one!
fn main() {

    let mut alg = vec![];

    for c in INPUT_ALG.chars().filter(|c| *c == '#' || *c == '.') {
        alg.push(if c == '#' { 1u8 } else { 0u8 });
    }

    let mut width = 0;
    let mut data = vec![];

    for line in INPUT_DATA.trim().lines() {
        width = line.len();

        for c in line.chars().filter(|c| *c == '#' || *c == '.') {
            data.push(if c == '#' { 1u8 } else { 0u8 });
        }
    }

    let mut grid = Grid::new(data, width, 0);

    for _ in 0..50 {
        grid = grid.next(&alg);
    }

    println!("{}", grid.pop_count());

}
