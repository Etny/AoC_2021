mod input;

use input::*;

fn main() {
    let mut fish = vec![0u64; 9];

    for fish_dec in INPUT.trim().split(',') {
        let fish_age = fish_dec.parse::<usize>().unwrap();
        fish[fish_age] += 1;
    }

    let days = 256;

    for _ in 0..days {
       
        let fish_loop = fish[0];

        fish = fish.into_iter().skip(1).collect();
        fish.push(fish_loop);
        fish[6] += fish_loop;
    }

    println!("{}", fish.into_iter().reduce(|acc, f| acc + f).unwrap());
}

