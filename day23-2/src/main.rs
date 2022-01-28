mod input;

use std::{fmt::Display, collections::{BinaryHeap, HashMap}, time::Instant};
use regex::Regex;
use input::*;

const HEIGHT: u8 = 5;
type PodType = u8;
type PodMap = Vec<Option<PodType>>;

static ROOM_INDICES: [u8; 4] = [2, 4, 6, 8];

static POD_COST: [u32; 4] = [1, 10, 100, 1000];
static POD_CHAR: [char; 4] = ['A', 'B', 'C', 'D'];


// From 5 minute search time to 25 secs to now ~500 ms, not perfect but still happy with it.
fn main() {
    let state = State::parse(INPUT.trim());

    println!("{}", solve(state));
}

fn solve(state: State) -> u32 {
    let start = Instant::now();
    let mut heap = BinaryHeap::new();
    heap.push(state);

    while !heap.is_empty() {
        let state = heap.pop().unwrap();

        if state.cost_heuristic == 0 {
            println!("dT: {}", (Instant::now() - start).as_secs_f32());
            return state.cost;
        }

       state.push_successors(&mut heap);
    }
    
    println!("Failed to find a path");
    0
}

fn calc_moves(p1: (u8, u8), p2: (u8, u8)) -> u8 {
    if p1.0 > p2.0 {
        (p1.0 - p2.0) + p1.1 + p2.1
    } else {
        (p2.0 - p1.0) + p1.1 + p2.1
    }
}

fn to_index((x, y): (u8, u8)) -> usize {
    match y {
        0 =>  x as usize,
        _ => to_index_room((x, y))
    }
}

//For when we know we are indexing a room, to avoid an unnecessary branching check
fn to_index_room((x, y): (u8, u8)) -> usize {
    (11 + ((x / 2 - 1) * 4) + (y - 1)) as usize
}

fn to_pos(index: usize) -> (u8, u8) {
    match index {
        0..=10 => (index as u8, 0),
        _ => ((((index - 11) / 4 + 1) * 2) as u8, ((index - 11) % 4 + 1) as u8)
    }
}


struct State{
    pods: PodMap,
    cost: u32,
    cost_heuristic: u32
}   


impl State {
    fn push_successors(&self, heap: &mut BinaryHeap<State>) {
        'o: for pos_i in 0..self.pods.len()  {
            if self.pods[pos_i].is_none() { continue; }

            let pos = &to_pos(pos_i);

            if is_home(self.pods[pos_i].unwrap(), *pos, &self.pods) { continue; }

            if pos.1 > 1 {
                for i in 1..pos.1 {
                    if self.pods[pos_i - i as usize].is_some() { continue 'o; }
                }
            }

            if let Some(home) = next_accessible_home(self.pods[pos_i].unwrap(), &self.pods) {
                if self.reachable_from_x(pos.0).contains(&home.0) {
                    heap.push(self.create_successor(*pos, home, pos_i));
                    continue;
                }
            }

            if pos.1 > 0 {
                for x in self.reachable_from_x(pos.0) {
                    if ROOM_INDICES.contains(&x) { continue; }
                    heap.push(self.create_successor(*pos, (x, 0), pos_i));
                }
            }
         
        }   
    }

    fn create_successor(&self, move_src: (u8, u8), move_dest: (u8, u8), index_src: usize) -> Self {
        let pod = self.pods[index_src].unwrap();
        
        let mut new_pods = self.pods.clone();
        new_pods.swap(to_index(move_dest), index_src);

        let extra_cost = calc_moves(move_src, move_dest) as u32 * POD_COST[pod as usize];

        let h = Self::heuristic_analysis(&new_pods);  

        Self { pods: new_pods, cost: self.cost + extra_cost, cost_heuristic: h }
    }

    fn heuristic_analysis(pods: &PodMap) -> u32 {
        let mut h = 0;

        for i in 0..pods.len() {
            if let Some(pod) = pods[i]{
                let pos = to_pos(i);
                if is_home(pod, pos, &pods) { continue; }
                let home = next_home(pod, &pods); 
                h += calc_moves(pos, home) as u32 * POD_COST[pod as usize];
            }
        }

        h
    }

    fn reachable_from_x(&self, src_x: u8) -> Vec<u8> {
        let mut out = vec![];

        if src_x > 0 { 
            let mut i = (src_x - 1) as usize;
            loop {
                if self.pods[i].is_some() { break; }
                out.push(i as u8);

                if i == 0 { break; } else { i -= 1; }
            }
        }

        if src_x < 10 {
            let mut i = (src_x + 1) as usize;
            loop {
                if self.pods[i].is_some() { break; }
                out.push(i as u8);

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
                    .map(|d| from_dec(d.unwrap().as_str())),
            )
            .collect::<HashMap<_, _>>();
        let pods = (0..27).map(|p| pods.get(&to_pos(p)).map_or(None, move |f| Some(*f))).collect::<Vec<_>>();

        Self { pods, cost: 0, cost_heuristic: 1 }
    }
}


    fn is_home(pod: PodType, (x, y): (u8, u8), pods: &PodMap) -> bool {
        if x != ROOM_INDICES[pod as usize] || y == 0 {
            false
        } else if y == HEIGHT-1 {
            true
        } else {
            for i in y+1..HEIGHT {
                if pods[to_index_room((x, i))].map_or(false, |f| f != pod) {
                    return false;
                }
            }

            true
        }
    }

    fn next_accessible_home(pod: PodType, pods: &PodMap) -> Option<(u8, u8)> {
        let x = ROOM_INDICES[pod as usize];
        for y in (1..HEIGHT).rev() {
            match pods[to_index_room((x, y))] {
                None => return Some((x, y)),
                Some(p) if p == pod => continue,
                Some(_) => return None
            }
        }

        None
    }

    fn next_home(pod: PodType, pods: &PodMap) -> (u8, u8) {
        let x = ROOM_INDICES[pod as usize];
        for y in (1..HEIGHT).rev() {
            match pods[to_index_room((x, y))] {
                Some(p) if p == pod => continue,
                _ => return (x, y)
            }
        }

        panic!()
    }

    fn from_dec(dec: &str) -> PodType {
        match dec {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            "D" => 3,
            _ => panic!(),
        }
    }

impl  PartialEq for State  {
    fn eq(&self, other: &Self) -> bool {
        (self.cost + self.cost_heuristic) == (other.cost + other.cost_heuristic)
    }
}

impl  Eq for State  {}

impl  PartialOrd for State  {
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

        writeln!(f, "cost: {}, h: {}", self.cost, self.cost_heuristic)?;

        for y in 0..HEIGHT {
            for i in 0..11 {
                if y != 0 && !ROOM_INDICES.contains(&i) {
                    write!(f, " ")?;
                } else {
                    write!(f, "{}", self.pods[to_index((i, y))].map_or('.', |f| POD_CHAR[f as usize]))?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "-----------")?;
        

        Ok(())
    }
}