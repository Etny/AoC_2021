mod input;

use input::*;

fn main() {

    let mut pos = (0, 0);

    for line in INPUT.lines() {
        let mut iter = line.trim().split(" ");
        let phrase = iter.next().unwrap();
        let amount = iter.next().unwrap().parse::<i32>().unwrap();

        match phrase {
            "forward" => pos = (pos.0 + amount, pos.1),
            "down" => pos = (pos.0, pos.1 + amount),
            "up" => pos = (pos.0, pos.1 - amount),
            _ => panic!("Uh oh...")
        }
    }

    println!("{}", pos.0 * pos.1);

}
