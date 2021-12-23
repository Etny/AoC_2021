mod input;

use input::*;

fn main() {
    let mut crabs = vec![];

    for crab_dec in INPUT.trim().split(',') {
        crabs.push(crab_dec.parse::<i32>().unwrap());
    }

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut answer = -1;

    for i in min..=max {
        let cost = crabs.iter().fold(0, |acc, c| acc + (*c-i).abs());
        if answer < 1 || cost < answer { 
            answer = cost; 
        }
    }

    println!("{}",  answer);
}
