mod input;
use std::{collections::HashSet, ops::Range};

use input::*;

fn main() {
    let mut reactor = HashSet::new();

    for line in INPUT.trim().lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let on = parts[0] == "on";

        let nums = parts[1].split(',').collect::<Vec<_>>();
        let range_x = parse_range(nums[0]);
        let range_y = parse_range(nums[1]);
        let range_z = parse_range(nums[2]);

        for x in range_x {
            for y in range_y.clone() {
                for z in range_z.clone() {
                    if on {
                        reactor.insert((x, y, z));
                    } else {
                        reactor.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    println!("{}", reactor.len());
}

fn parse_range(dec: &str) -> Range<i32> {
    let nums = dec[2..].split("..").collect::<Vec<_>>();

    let mut start: i32 = nums[0].parse().unwrap();
    let mut end: i32 = nums[1].parse().unwrap();

    if start.abs() > 50 {
        start = start.signum() * 50;
    }
    if end.abs() > 50 {
        end = end.signum() * 50;
    }

    if start == end {
        return 0..0;
    }

    start..(end + 1)
}
