mod input;
use input::*;

fn main() {
    let ranges = INPUT.trim().replace("target area: ", "");

    let mut range_y = (0, 0);

    for r in ranges.split(", ") {
        let parts = r.split('=').collect::<Vec<_>>();

        if parts[0] == "y" {
            let nums = parts[1]
                .split("..")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>();
            range_y = (nums[0].min(nums[1]), nums[0].max(nums[1]));
        }
    }

    if range_y.0 > 0 {
        //Unnecessary for the specified input, but I wanted to include it nonetheless
        let range = (range_y.0..range_y.1).collect::<Vec<_>>();
        let mut acc = 0;
        for i in 0.. {
            acc += i;
            if acc > range_y.1 {
                println!("No possible y-velocity");
                break;
            } else if range.contains(&acc) {
                println!("{}", i);
                break;
            }
        }
    } else {
        println!("{}", (0..=(-range_y.0 - 1)).sum::<i32>());
    }
}
