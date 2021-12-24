mod input;

use input::*;

fn main() {

    let unique_lenghts = vec![2, 4, 3, 7];
    let mut total = 0;
    
    for line in INPUT.trim().lines() {
        let output = line.split(" | ").skip(1).next().unwrap();
        
        for digit in output.split(' ') {
            if unique_lenghts.contains(&digit.len()) {
                total += 1;
            }
        }
    }

    println!("{}", total);

}
