mod input;

use input::*;

fn main() {
    let increases = get_numbers().windows(2).filter(|w| w[1] > w[0]).count();
    println!("{}", increases);
}
