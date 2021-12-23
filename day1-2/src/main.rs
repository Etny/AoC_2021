mod input;

use input::*;

fn main() {
    let output = 
        get_numbers()
        .windows(3)
        .zip(
            get_numbers()
            .windows(3)
            .skip(1)
        )
        .filter(
            |(a, b)| 
            b.iter().sum::<i32>() > a.iter().sum()
        )
        .count();
    
    println!("{}", output);
}
