mod input;
mod state;

use std::collections::{BinaryHeap, HashMap};

use input::*;
use state::*;

fn main() {
    let state = State::parse(INPUT.trim());
    println!("{:?}", solve(state));
}

fn solve(state: State) -> u64 {
    let mut nodes = BinaryHeap::new();
    nodes.push(state);

    while !nodes.is_empty() {
        let state = nodes.pop().unwrap();

        if state.cost_heuristic == 0 {
            return state.cost;
        }

        nodes.extend(state.successors());
    }

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
            Self::D => 1000,
        }
    }

    fn dest_x(&self) -> usize {
        match *self {
            Self::A => 2,
            Self::B => 4,
            Self::C => 6,
            Self::D => 8,
        }
    }

    fn next_target(
        &self,
        (x, y): (usize, usize),
        state: &HashMap<(usize, usize), Self>,
    ) -> Option<(usize, usize)> {
        let dest = self.dest_x();

        if x == dest && y == 2 {
            None
        } else {
            if state.get(&(dest, 2)).map_or(false, |f| *f == *self) {
                if x == dest && y == 1 {
                    None
                } else {
                    Some((dest, 1))
                }
            } else {
                Some((dest, 2))
            }
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
        }
    }
}
