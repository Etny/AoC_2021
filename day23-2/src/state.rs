use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::Pod;

fn manhattan_distance(src: (usize, usize), dest: (usize, usize)) -> u64 {
    let (x1, y1) = (src.0 as i64, src.1 as i64);
    let (x2, y2) = (dest.0 as i64, dest.1 as i64);

    ((x1 - x2).abs() + (y1 - y2).abs()) as u64
}

fn calc_moves((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> u64 {
    if x1 > x2 {
        ((x1 - x2) + y1 + y2) as u64
    } else {
        ((x2 - x1) + y1 + y2) as u64
    }
}

pub const HEIGHT: usize = 5;

lazy_static! {
    static ref ROOM_POS: Vec<usize> = vec![2, 4, 6, 8];
}

#[derive(PartialEq, Eq)]
pub struct State {
    pub pods: HashMap<(usize, usize), Pod>,
    pub cost: u64,
    pub cost_heuristic: u64
}


impl State {

    pub fn successors(&self) -> Vec<State> {
        let mut successors = vec![];


        'o: for pos in self.pods.keys() {
            if let Some(target) = self.pods[pos].next_target(*pos, &self.pods) {
                if pos.1 > 0 {
                    if pos.1 > 1 {
                        for i in 1..pos.1 {
                            if self.pods.get(&(pos.0, i)).is_some() { continue 'o; }
                        }
                    }

                    for x in self.possible_hallway_locations(pos.0) {
                        if ROOM_POS.contains(&x) { continue; }
                        successors.push(self.create_successor(*pos, (x, 0)));
                    }
                }

                if let None = self.pods.get(&target) {
                    if self.possible_hallway_locations(pos.0).contains(&target.0) {
                        successors.push(self.create_successor(*pos, target));
                    }
                }
            }
        }
        
        successors
    }

    // fn validate(&self) -> bool {
    //     for pos in self.pods.keys() {
    //         if pos.1 == 1 && !self.pods.contains_key(&(pos.0, 2)) {
    //             return false;
    //         }
    //     }

    //     true
    // }

    fn possible_hallway_locations(&self, src_x: usize) -> Vec<usize> {
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

    pub fn create_successor(&self, move_src: (usize, usize), move_dest: (usize, usize)) -> Self {
        let mut new_pods = self.pods.clone();

        let pod = new_pods.remove(&move_src).unwrap();
        let extra_cost = calc_moves(move_src, move_dest) * pod.move_cost();

        new_pods.insert(move_dest, pod);
        let h = Self::heuristic_analysis(&new_pods);
        let s = Self { pods: new_pods, cost: self.cost + extra_cost, cost_heuristic: h };

        // if !s.validate() { println!("{}", self); panic!(); }
        s
    }

    pub fn heuristic_analysis(pods: &HashMap<(usize, usize), Pod>) -> u64 {
        let mut total = 0;

        let mut done = HashSet::new();

        for key in pods.keys() {
            if let Some(mut target) = pods[key].next_target(*key, pods) {
                while done.contains(&target) {
                    target = (target.0, target.1 - 1);
                }
                done.insert(target);

                if key.0 == target.0 && target.1 > key.1 {
                    for i in 1..=target.1 {
                        if let Some(pod) = pods.get(&(target.0, i)) {
                            total += ((i as u64) * 2 + 2) * pod.move_cost();
                        }
                    }
                }

                total += calc_moves(*key, target) * pods[key].move_cost();
            }
        }

        total
    }

    // pub fn is_complete(&self) -> bool {
    //     lazy_static! {
    //         static ref COMPLETE_MAP: HashMap<(usize, usize), Pod> = {
    //             let mut m = HashMap::new();
    //             m.insert((2, 1), Pod::A);
    //             m.insert((2, 2), Pod::A);
    //             m.insert((4, 1), Pod::B);
    //             m.insert((4, 2), Pod::B);
    //             m.insert((6, 1), Pod::C);
    //             m.insert((6, 2), Pod::C);
    //             m.insert((8, 1), Pod::D);
    //             m.insert((8, 2), Pod::D);
    //             m
    //         };
    //     }

    //     self.pods == *COMPLETE_MAP
    // }

    pub fn parse(dec: &str) -> Self {
        let reg = Regex::new(&include_str!("input_regex.txt")).unwrap();

        let captures = reg.captures(dec).unwrap();
        let positions = (1..HEIGHT).flat_map(|f| ROOM_POS.iter().map(move |r| (*r, f))).collect::<Vec<_>>();
        let pods = positions
            .into_iter()
            .zip(
                captures
                    .iter()
                    .skip(1)
                    .map(|d| Pod::from_dec(d.unwrap().as_str())),
            )
            .collect::<HashMap<_, _>>();

        Self { pods, cost: 0, cost_heuristic: 1 }
    }

  
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (u64::MAX - (self.cost + self.cost_heuristic)).partial_cmp(&(u64::MAX - (other.cost + other.cost_heuristic)))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        writeln!(f, "cost: {}, h: {}", self.cost, self.cost_heuristic)?;

        for y in 0..HEIGHT {
            for i in 0..11 {
                if y != 0 && !ROOM_POS.contains(&i) {
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

