mod input;

use input::*;

fn main() {
    let mut fish = Vec::<u32>::new();

    for fish_dec in INPUT.trim().split(',') {
        fish.push(fish_dec.parse().unwrap());
    }

    let days = 80;

    for _ in 0..days {
        let mut spawn = 0;
        fish = fish.into_iter().map(|f| {
            if f == 0 {
                spawn += 1;
                6
            } else {
                f - 1
            }
        }).collect();
        fish.append(&mut vec![8u32; spawn]);

    }

    println!("{}", fish.len());
}

