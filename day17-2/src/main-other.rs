mod input;

use std::collections::{HashMap, HashSet};

use input::*;

//In memory of this horrible, terrible attempted solution. 
fn main() {
    let ranges = INPUT.trim().replace("target area: ", "");

    let mut range_x = 0..=0;
    let mut range_y = 0..=0;

    for r in ranges.split(", ") {
        let parts = r.split('=').collect::<Vec<_>>();
        let nums = parts[1].split("..").map(|n| n.parse().unwrap()).collect::<Vec<i32>>();

        if parts[0] == "x" {
            range_x = nums[0].min(nums[1])..=nums[0].max(nums[1]);
        } else {
            range_y = nums[0].min(nums[1])..=nums[0].max(nums[1]);
        }
    }

    let mut y_steps = HashMap::new();
    // let mut acc_sum_cache = HashMap::new();


    for y in range_y {
        if y >= 0 { panic!() }

        y_steps.entry(1).or_insert(HashSet::new()).insert(y);
        y_steps.entry(2 * - y).or_insert(HashSet::new()).insert((-y) - 1);

        
        // if let Some((steps, vel)) = steps_to_pos_neg(y) {
        //     // println!("Result for {}: {}, {}", y, steps, vel);
        //     y_steps.entry(steps).or_insert(HashSet::new()).insert(vel);

        //     if vel <= -2 {
        //         if let Some(steps_mirror) = steps_to_pos((-vel) - 1) {
        //             // println!("Got mirror steps {} for y {} with vel {} (new vel {})", steps_mirror, y, vel, (-vel) -1);
        //             y_steps.entry(steps_mirror * 2 + 1 + steps).or_insert(HashSet::new()).insert(steps_mirror);
        //         }
        //     }
        // }


        for i in 0..=(-y/2) {
            for j in 1..i {
                if (j..=i).sum::<i32>() == -y {
                    println!("Found {} (j: {}) for y {} with steps {}", i, j, y, i-j + 1);
            
                    y_steps.entry(i-j + 1).or_insert(HashSet::new()).insert(-j);
                    if j == 1 {
                        y_steps.entry(i-j + 2).or_insert(HashSet::new()).insert(0);
                    }
                }
            }
        }


    }

    let mut poss = HashSet::new();

    for x in range_x {
        if let Some(steps) = steps_to_pos(x) {
            for key in y_steps.keys().filter(|k| *k >= &steps) {
                for y in &y_steps[key] {
                    poss.insert((steps, *y));
                }
            }
        }

        for i in 0..=(x/2) {
            for j in 1..i {
                if (j..=i).sum::<i32>() == x {
                    // println!("Found {} for x {} with steps {}", i, x, i-j + 1);
                    if y_steps.contains_key(&(i-j + 1)) {
                        for y in &y_steps[&(i-j + 1)] {
                            poss.insert((i, *y));
                        }
                    }
                }
            }
        }

        if y_steps.contains_key(&1) {
            for y in &y_steps[&1] {
                poss.insert((x, *y));
            }
        }


        if let Some((steps, bad_vel)) = steps_to_pos_neg(-x) {
            let vel = (-bad_vel) + (steps - 1);

            if y_steps.contains_key(&steps) {
                for y in &y_steps[&steps] {
                    poss.insert((vel, *y));
                }
            }
        }
    }

    // println!("{:?}", y_steps);
    // println!("{}", poss.len());
    let mut vec = poss.into_iter().collect::<Vec<_>>();
    vec.sort();
    // println!("{:?}", vec);

    let mut test = Vec::new();

    for i in TEST.split_ascii_whitespace() {
        let parts = i.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
        test.push((parts[0], parts[1]))
    }

    for i in test.iter().filter(|n| !vec.contains(n)) {
        println!("{:?}", i);
    }
}

fn steps_to_pos_neg(pos: i32) -> Option<(i32, i32)> {
    let mut pos_acc = -pos;

    let mut vel = -pos / 2;
    if pos % 2 == -1 { vel += 1; }


    for i in 0.. {
        pos_acc -= vel;


        if pos_acc == 0 {
            return Some((i+1, -vel));
        } else if pos_acc < 0 || vel <= 0{
            return None;
        }

        vel -= 1;
    }

    None
}

fn steps_to_pos(pos: i32) -> Option<i32> {
    let mut acc = 0;

    for i in 0.. {
        acc += i;

        if acc > pos {
            return None
        } else if acc == pos {
            return Some(i);
        }
    }

    None
}


// fn max_x_vel(target_x: &[i32]) -> i32 {
//     let mut traveled: i32 = 0;
//     let mut vel = 0;

//     while traveled.abs() < i32::MAX && !target_x.contains(&traveled) {
//         vel += 1;
//         traveled += vel;
//     }

//     vel
// }

// fn max_y_vel(target_y: i32) -> i32 {

//     if target_y > 0 { panic!(); }

//     (-target_y) - 1
// }

// fn max_y_pos(lowest_y: i32) -> i32 {
//     if lowest_y > 0 { panic!(); }

//     (0..=((-lowest_y) - 1)).sum()
// }
