
mod input;

use std::collections::HashMap;

use input::*;


fn main() {
    let mut pop_count: HashMap<usize, u32> = HashMap::new();

    for line in INPUT.trim().lines() {
        for (e, c) in line.chars().rev().enumerate() {
            if c == '0' { continue; }

            let count = pop_count.entry(e).or_insert(0);
            *count += 1;
        }
    }

    let target = (INPUT.trim().lines().count() / 2) as u32;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for index in pop_count.keys() {
        match pop_count[index] < target {
            true => epsilon |= 1 << index,
            false => gamma |= 1 << index
        }
    }


    println!("{}", gamma * epsilon);
}
