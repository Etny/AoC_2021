use crate::input::*;
use lazy_static::lazy_static;
use regex::Regex;


//A fast, but input-specific solution. Didn't think of this on my own, hugely inspired by https://github.com/MarcelRobitaille/2021-advent-of-code/blob/main/day_24/src/main.rs
pub fn solve_fast(do_min: bool) {
    let mut blocks = INPUT
        .trim()
        .split("inp w")
        .filter_map(|b| {
            if b.trim().len() > 0 {
                Some(Block::from_dec(b.trim()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let out = solve(&mut blocks, do_min).iter().fold(0, |acc, i| (acc * 10) + i);

    println!("{}", out);
}

fn solve(blocks: &mut Vec<Block>, do_min: bool) -> Vec<i64> {
    if blocks.is_empty() || blocks[0].is_pop() {
        return vec![];
    }

    let push = blocks.remove(0);
    
    let mut nums = solve(blocks, do_min);

    let pop = blocks.remove(0);

    let num_push = match do_min {
        true => 1.max(1 - push.val() - pop.val()),
        false => 9.min(9 - push.val() - pop.val())
    };

    let num_pop = num_push + push.val() + pop.val();

    let mut ret = vec![num_push];
    ret.append(&mut nums);
    ret.push(num_pop);
    ret.append(&mut solve(blocks, do_min));

    ret
}

#[derive(Debug)]
enum Block {
    Push(i64),
    Pop(i64),
}

impl Block {
    fn from_dec(dec: &str) -> Self {
        lazy_static! {
            static ref REGEX_BLOCK: Regex = Regex::new(include_str!("block_regex.txt")).unwrap();
        }

        let captures = REGEX_BLOCK.captures(dec).unwrap();

        if captures[1].eq("1") {
            Self::Push(captures[3].parse().unwrap())
        } else {
            Self::Pop(captures[2].parse().unwrap())
        }
    }

    fn val(&self) -> i64 {
        match *self {
            Self::Push(i) => i,
            Self::Pop(i) => i
        }
    }

    fn is_pop(&self) -> bool {
        match *self {
            Self::Pop(_) => true,
            _ => false
        }
    }
}
