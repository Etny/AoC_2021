mod input;

use input::*;

fn main() {

    let mut horz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in INPUT.lines() {
        let mut iter = line.trim().split(" ");
        let phrase = iter.next().unwrap();
        let amount = iter.next().unwrap().parse::<i32>().unwrap();

        match phrase {
            "down" => aim += amount,
            "up" => aim -= amount,
            "forward" => {
                horz += amount;
                depth += aim * amount;
            }
            _ => panic!("Uh oh...")
        }
    }

    println!("{}", horz * depth);

}
