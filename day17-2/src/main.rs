mod input;

use input::*;

fn main() {
    let ranges = INPUT.trim().replace("target area: ", "");

    let mut range_x = 0..=0;
    let mut range_y = 0..=0;

    for r in ranges.split(", ") {
        let parts = r.split('=').collect::<Vec<_>>();
        let nums = parts[1]
            .split("..")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();

        if parts[0] == "x" {
            range_x = nums[0].min(nums[1])..=nums[0].max(nums[1]);
        } else {
            range_y = nums[0].min(nums[1])..=nums[0].max(nums[1]);
        }
    }

    let range_x_vel = (*range_x.start() as f64).sqrt() as i32..=*range_x.end();
    let range_y_vel = (*range_y.start()..=(-*range_y.start() - 1)).collect::<Vec<_>>();

    let mut total = 0;

    for start_vel_x in range_x_vel {
        for start_vel_y in &range_y_vel {
            let (mut vel_x, mut vel_y) = (start_vel_x, *start_vel_y);
            let (mut x, mut y) = (0, 0);

            while x < *range_x.end() && y > *range_y.start() {
                x += vel_x;
                y += vel_y;

                vel_y -= 1;
                if vel_x > 0 {
                    vel_x -= 1;
                } else if vel_x < 0 {
                    vel_x += 1;
                }

                if x <= *range_x.end()
                && x >= *range_x.start()
                && y <= *range_y.end()
                && y >= *range_y.start()
                {
                    total += 1;
                    break;
                }
            }
        }
    }

    println!("{}", total);
}
