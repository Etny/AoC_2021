mod input;


use std::{fmt::Display, collections::BinaryHeap, time::Instant};
use rustc_hash::FxHashMap;
use regex::Regex;
use lazy_static::lazy_static;
use input::*;

const HEIGHT: u64 = 5;
type PodMap = FxHashMap<(u64, u64), PodType>;

lazy_static! {
    static ref ROOM_INDICES: Vec<u64> = vec![2, 4, 6, 8]; 
}

fn main() {
    let state = State::parse(INPUT.trim());


    println!("{}", solve(state));
}

fn solve(state: State) -> u64 {
    let start = Instant::now();
    let mut heap = BinaryHeap::new();
    heap.push(state);

    while !heap.is_empty() {
        let state = heap.pop().unwrap();

        // println!("{}", state);

        if state.cost_heuristic == 0 {
            println!("dT: {}", (Instant::now() - start).as_secs_f32());
            println!("{}", state);
            return state.cost;
        }

        // heap.extend(state.push_successors());
       state.push_successors(&mut heap);
    }
    
    println!("Failed to find a path");
    0
}

    fn calc_moves(p1: (u64, u64), p2: (u64, u64)) -> u64 {
        if p1.0 > p2.0 {
            (p1.0 - p2.0) + p1.1 + p2.1
        } else {
            (p2.0 - p1.0) + p1.1 + p2.1
        }
    }


struct State {
    pods: PodMap,
    to_move: Vec<(u64, u64)>,
    cost: u64,
    cost_heuristic: u64
}   

#[derive(PartialEq, Eq, Clone)]
enum PodType {
    A, B, C, D
}

impl State {
    fn push_successors(&self, heap: &mut BinaryHeap<State>) /*-> Vec<Self>*/{
        // let mut heap = vec![];
        for pos in &self.to_move {
            if self.pods[pos].is_home(*pos, &self.pods) { continue; }

            if let Some(home) = self.pods[pos].next_accessible_home(&self.pods) {
                if self.reachable_from_x(pos.0).contains(&home.0) {
                    heap.push(self.create_successor(*pos, home, true));
                    // continue;
                }
            }
            if pos.1 > 0 {
                for x in self.reachable_from_x(pos.0) {
                    if ROOM_INDICES.contains(&x) { continue; }
                    heap.push(self.create_successor(*pos, (x, 0), false));
                }
            }
         
        }   
        // heap
    }

    fn create_successor(&self, move_src: (u64, u64), move_dest: (u64, u64), moved_home: bool) -> Self {
        let new_pods = self.pods.iter().map(|f| if *f.0 == move_src { (move_dest, f.1.clone()) } else { (*f.0, f.1.clone()) }).collect();

        let pod = &self.pods[&move_src];
        let extra_cost = calc_moves(move_src, move_dest) * pod.move_cost();

        let h = Self::heuristic_analysis(&new_pods);

        let mut to_move: Vec<_> = self.to_move.iter().filter(|r| **r != move_src).map(|r| *r).collect();

        if moved_home {
            to_move = to_move.into_iter().filter(|r| r.0 != move_dest.0).collect();
        } else {
            to_move.push(move_dest);
        }

        if move_src.1 != 0 && move_src.1 != HEIGHT-1 {
            to_move.push((move_src.0, move_src.1 + 1));
        }

        Self { pods: new_pods, to_move, cost: self.cost + extra_cost, cost_heuristic: h }
    }

    fn heuristic_analysis(pods: &PodMap) -> u64 {
        let mut h = 0;

        for pos in pods.keys() {
            if pods[pos].is_home(*pos, &pods) { continue; }
            let home = pods[pos].next_home(&pods); 
            h += calc_moves(*pos, home) * pods[pos].move_cost();
        }

        h
    }

    fn reachable_from_x(&self, src_x: u64) -> Vec<u64> {
        let mut out = vec![];

        if src_x > 0 { 
            let mut i = src_x - 1;
            loop {
                if self.pods.contains_key(&(i, 0)) { break; }
                out.push(i);

                if i == 0 { break; } else { i -= 1; }
            }
        }

        if src_x < 10 {
            let mut i = src_x + 1;
            loop {
                if self.pods.contains_key(&(i, 0)) { break; }
                out.push(i);

                if i >= 10 { break; } else { i += 1; }
            }
        }


        out
    }

    fn parse(dec: &str) -> Self {
        let reg = Regex::new(&include_str!("input_regex.txt")).unwrap();

        let captures = reg.captures(dec).unwrap();
        let positions = (1..HEIGHT).flat_map(|f| ROOM_INDICES.iter().map(move |r| (*r, f))).collect::<Vec<_>>();
        let pods = positions
            .into_iter()
            .zip(
                captures
                    .iter()
                    .skip(1)
                    .map(|d| PodType::from_dec(d.unwrap().as_str())),
            )
            .collect::<FxHashMap<_, _>>();
        let to_move: Vec<_> = ROOM_INDICES.iter().map(move |r| (*r, 1)).collect();

        Self { pods, to_move, cost: 0, cost_heuristic: 1 }
    }
}

impl PodType {

    fn is_home(&self, (x, y): (u64, u64), pods: &PodMap) -> bool {
        if x != self.dest_x() || y == 0 {
            false
        } else if y == HEIGHT-1 {
            true
        } else {
            for i in y+1..HEIGHT {
                if pods.get(&(x, i)).map_or(false, |f| *f != *self) {
                    return false;
                }
            }

            true
        }
    }

    fn next_accessible_home(&self, pods: &PodMap) -> Option<(u64, u64)> {
        let x = self.dest_x();
        for y in (1..HEIGHT).rev() {
            match pods.get(&(x, y)) {
                None => return Some((x, y)),
                Some(p) if *p == *self => continue,
                Some(_) => return None
            }
        }

        None
    }

    fn next_home(&self, pods: &PodMap) -> (u64, u64) {
        let x = self.dest_x();
        for y in (1..HEIGHT).rev() {
            match pods.get(&(x, y)) {
                Some(p) if *p == *self => continue,
                _ => return (x, y)
            }
        }

        (0, 0)
    }

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

    fn dest_x(&self) -> u64 {
        match *self {
            Self::A => 2,
            Self::B => 4,
            Self::C => 6,
            Self::D => 8 
        }
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

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (self.cost + self.cost_heuristic) == (other.cost + other.cost_heuristic)
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((other.cost + other.cost_heuristic).cmp(&(self.cost + self.cost_heuristic)))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.cost_heuristic).cmp(&(self.cost + self.cost_heuristic))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        writeln!(f, "to_move: {:?}", self.to_move)?;
        writeln!(f, "cost: {}, h: {}", self.cost, self.cost_heuristic)?;

        for y in 0..HEIGHT {
            for i in 0..11 {
                if y != 0 && !ROOM_INDICES.contains(&i) {
                    write!(f, " ")?;
                } else {
                    write!(f, "{}", self.pods.get(&(i, y)).map_or('.', |f| f.to_char()))?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "-----------")?;
        

        Ok(())
    }
}