mod input;
mod state;



use std::collections::{BinaryHeap, HashMap, BTreeMap};
use std::time::Instant;

use rustc_hash::FxHashMap;
use state::*;
use input::*;

fn main() {
    let state = State::parse(INPUT.trim());
    println!("{:?}", solve(state));
    // println!("{}", state);

}

fn solve(state: State) -> u64 {
    let mut nodes = BinaryHeap::new();
    nodes.push(state);

    let start = Instant::now();

    while !nodes.is_empty() {
        let state = nodes.pop().unwrap();

        // if state.cost % 100 == 0 {
        //     println!("{}", state);
        // }

        // println!("{}", state);
        // println!("{}", State::heuristic_analysis(&state.pods));
        // println!("{:?}", state.pods[&(4, 1)].next_target((4, 1), &state.pods));

        if state.cost_heuristic == 0 { println!("{}", (Instant::now() - start).as_secs_f32()); println!("{}", state); return state.cost; }

        nodes.extend(state.successors());
        // state.successors(&mut nodes);
    }

    println!(":(");
    0
}


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn from_dec(dec: &str) -> Self {
        match dec {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            _ => panic!(),
        }
    }

    fn move_cost(&self) -> u64 {
        match *self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000
        }
    }

    fn dest_x(&self) -> usize {
        match *self {
            Self::A => 2,
            Self::B => 4,
            Self::C => 6,
            Self::D => 8 
        }
    }


    fn next_target_1(&self, (x, y): (usize, usize), state: &FxHashMap<(usize, usize), Self>) -> Result<Option<(usize, usize)>, ()> {
        let dest = self.dest_x();

        for i in (1..HEIGHT).rev() { 
            if x == dest && y == i { return Err(()); }
            // if state.get(&(dest, i)).is_none() {
            //     return Ok(Some((dest, i)));
            // } else if *state.get(&(dest, i)).unwrap() != *self {
            //     return Ok(None);
            // }

            match state.get(&(dest, i)) {
                None => return Ok(Some((dest, i))),
                Some(a) if *a != *self => return Ok(None),
                _ => ()
            }
        }

        Err(())
    }

    fn next_target(&self, (x, y): (usize, usize), state: &FxHashMap<(usize, usize), Self>) -> Option<(usize, usize)> {
        let dest = self.dest_x();

        for i in (1..HEIGHT).rev() { 
            if state.get(&(dest, i)).map_or(false, |f| *f == *self) {
                if x == dest && y == i {
                    return None;
                }
            } else {
                return Some((dest, i));
            }
        }

        None
    }

    pub fn to_char(&self) -> char {
        match *self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D'
        }
    }
}

