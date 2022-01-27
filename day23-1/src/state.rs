use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::Pod;

fn calc_moves((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> u64 {
    if x1 > x2 {
        ((x1 - x2) + y1 + y2) as u64
    } else {
        ((x2 - x1) + y1 + y2) as u64
    }
}

lazy_static! {
    static ref ROOM_POS: Vec<usize> = vec![2, 4, 6, 8];
}

#[derive(PartialEq, Eq)]
pub struct State {
    pub pods: HashMap<(usize, usize), Pod>,
    pub cost: u64,
    pub cost_heuristic: u64,
}

impl State {
    pub fn successors(&self) -> Vec<State> {
        let mut successors = vec![];

        for pos in self.pods.keys() {
            if let Some(target) = self.pods[pos].next_target(*pos, &self.pods) {
                if pos.1 == 2 && self.pods.contains_key(&(pos.0, 1)) {
                    continue;
                }

                if let None = self.pods.get(&target) {
                    if self.possible_hallway_locations(pos.0).contains(&target.0) {
                        successors.push(self.create_successor(*pos, target));
                    }
                }

                if pos.1 > 0 {
                    for x in self.possible_hallway_locations(pos.0) {
                        if ROOM_POS.contains(&x) {
                            continue;
                        }
                        successors.push(self.create_successor(*pos, (x, 0)));
                    }
                }
            }
        }

        successors
    }

    fn possible_hallway_locations(&self, src_x: usize) -> Vec<usize> {
        let mut out = vec![];

        if src_x > 0 {
            let mut i = src_x - 1;
            loop {
                if self.pods.contains_key(&(i, 0)) {
                    break;
                }
                out.push(i);

                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }

        if src_x < 10 {
            let mut i = src_x + 1;
            loop {
                if self.pods.contains_key(&(i, 0)) {
                    break;
                }
                out.push(i);

                if i >= 10 {
                    break;
                } else {
                    i += 1;
                }
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
        Self {
            pods: new_pods,
            cost: self.cost + extra_cost,
            cost_heuristic: h,
        }
    }

    fn heuristic_analysis(pods: &HashMap<(usize, usize), Pod>) -> u64 {
        let mut total = 0;

        let mut done = HashSet::new();

        for key in pods.keys() {
            if let Some(target) = pods[key].next_target(*key, pods) {
                let target = if done.contains(&target) {
                    (target.0, 1)
                } else {
                    done.insert(target);
                    target
                };
                total += calc_moves(*key, target) * pods[key].move_cost();
            }
        }

        total
    }

    pub fn parse(dec: &str) -> Self {
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

        Self {
            pods,
            cost: 0,
            cost_heuristic: 1,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (u64::MAX - (self.cost + self.cost_heuristic))
            .partial_cmp(&(u64::MAX - (other.cost + other.cost_heuristic)))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
