mod input;

use regex::Regex;
use lazy_static::lazy_static;
use std::collections::{HashMap, BinaryHeap};

use input::*;

fn main() {
    let state = State::parse(INPUT.trim());
    println!("{:?}", solve(state));
}

fn solve(state: State) -> u64 {
    let mut nodes = BinaryHeap::new();
    nodes.push(state);


    while !nodes.is_empty() {
        let state = nodes.pop().unwrap();

        if state.is_complete() { return state.cost; }

    }

    0
}

#[derive(PartialEq, Eq)]
struct State {
    pods: HashMap<(usize, usize), Pod>,
    cost: u64
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Pod {
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
}

impl State {

    fn is_complete(&self) -> bool {
        lazy_static! {
            static ref COMPLETE_MAP: HashMap<(usize, usize), Pod> = {
                let mut m = HashMap::new();
                m.insert((2, 1), Pod::A);
                m.insert((2, 2), Pod::A);
                m.insert((4, 1), Pod::B);
                m.insert((4, 2), Pod::B);
                m.insert((6, 1), Pod::C);
                m.insert((6, 2), Pod::C);
                m.insert((8, 1), Pod::D);
                m.insert((8, 2), Pod::D);
                m
            };
        }

        self.pods == *COMPLETE_MAP
    }

    fn parse(dec: &str) -> Self {
        let reg = Regex::new(&include_str!("input_regex.txt")).unwrap();

        let captures = reg.captures(dec).unwrap();
        let positions: [(usize, usize); 8] = [
            (2, 1),
            (4, 1),
            (6, 1),
            (8, 1),
            (2, 2),
            (4, 2),
            (6, 2),
            (8, 2),
        ];
        let pods = positions
            .into_iter()
            .zip(
                captures
                    .iter()
                    .skip(1)
                    .map(|d| Pod::from_dec(d.unwrap().as_str())),
            )
            .collect::<HashMap<_, _>>();

        Self { pods, cost: 0 }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (u64::MAX - self.cost).partial_cmp(&(u64::MAX - other.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
