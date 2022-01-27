use std::collections::BinaryHeap;

use self::PodType::*;

fn main() {
    let pods = vec![
        Pod::new(B, 0, 0),
        Pod::new(A, 0, 1),
        Pod::new(C, 1, 0),
        Pod::new(D, 1, 1),
        Pod::new(B, 2, 0),
        Pod::new(C, 2, 1),
        Pod::new(D, 3, 0),
        Pod::new(A, 3, 1),
    ];
    let moves = solve(pods);

    // println!("{}", calc_moves(3, 1, 4, 9));

    // print(&pods);
    println!("{}", moves);
}

fn solve(pods: Vec<Pod>) -> u64 {
    let mut nodes = BinaryHeap::new();
    nodes.push(GameState {
        cost: 0,
        pods: pods.clone(),
    });

    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();

        if node.cost % 10 == 0 {
            println!("Checking {}", node.cost);
            print(&node.pods);
        }

        if is_done(&node.pods) {
            return node.cost;
        }

        for i in 0..node.pods.len() {
            if node.pods[i].typ.next_target(&node.pods).is_none() {
                continue;
            }

            let pos = node.pods[i].pos;
            let next_target = node.pods[i].typ.next_target(&pods).unwrap();

            if pos.0 != 4 && pos.1 == 1 && pod_at((pos.0, 0), &node.pods).is_some() {
                continue;
            }
            if next_target.0 == pos.0 && pos.1 == 1 {
                continue;
            }

            if pod_at(next_target, &node.pods).is_none() && !(next_target.0 == pos.0 && pos.1 == 0)
            {
                let cost_add = calc_moves(pos, next_target) * node.pods[i].typ.move_cost();
                let mut next_state = node.pods.clone();
                next_state[i].pos = next_target;
                nodes.push(GameState {
                    cost: node.cost + cost_add,
                    pods: next_state,
                })
            } else if pos.0 != 4 {
                let mut hallway_offsets = vec![-1, 1];

                if pos.0 == 0 {
                    hallway_offsets.push(-2);
                }
                if pos.0 == 3 {
                    hallway_offsets.push(2);
                }

                for o in hallway_offsets {
                    let hallway_pos = (4, (room_hallway_index(pos.0) as i64 + o) as usize);
                    if pod_at(hallway_pos, &node.pods).is_some() {
                        continue;
                    }
                    let cost_add = calc_moves(pos, hallway_pos) * node.pods[i].typ.move_cost();
                    let mut next_state = node.pods.clone();
                    next_state[i].pos = hallway_pos;
                    nodes.push(GameState {
                        cost: node.cost + cost_add,
                        pods: next_state,
                    })
                }
            }
        }
    }

    0
}

fn is_done(pods: &Vec<Pod>) -> bool {
    pods.iter().all(|p| p.typ.next_target(pods).is_none())
}

fn calc_moves((src_r, src_i): (usize, usize), (dest_r, dest_i): (usize, usize)) -> u64 {
    let mut ret: usize;

    if dest_r == 4 {
        ret = src_i + 2;
    } else {
        if src_r == 4 {
            ret = (src_i as i64 - room_hallway_index(dest_r) as i64).abs() as usize;
            ret += 1 + dest_i;
        } else {
            ret = (room_hallway_index(src_r) as i64 - room_hallway_index(dest_r) as i64).abs()
                as usize;
            ret += 2 + src_i + dest_i;
        }
    }

    if ret == 0 {
        panic!()
    }

    ret as u64
}

fn room_hallway_index(room: usize) -> usize {
    2 + (2 * room)
}

fn pod_at((room, index): (usize, usize), pods: &Vec<Pod>) -> Option<usize> {
    (0..pods.len())
        .filter(|p| pods[*p].pos == (room, index))
        .next()
}

fn type_at((room, index): (usize, usize), pods: &Vec<Pod>) -> Option<PodType> {
    match pod_at((room, index), pods) {
        Some(p) => Some(pods[p].typ),
        None => None,
    }
}

fn print(pods: &Vec<Pod>) {
    let hallway: String = (0..11)
        .map(|i| {
            if let Some(p) = pod_at((4, i), pods) {
                pods[p].typ.to_string()
            } else {
                ".".to_owned()
            }
        })
        .collect();
    println!("{}", hallway);

    println!();

    let top: String = (0..=3)
        .map(|i| {
            if let Some(p) = pod_at((i, 0), pods) {
                pods[p].typ.to_string()
            } else {
                " ".to_owned()
            }
        })
        .collect();
    println!(
        "  {}  ",
        top.chars().flat_map(|i| [i, ' ']).collect::<String>()
    );

    let bot: String = (0..=3)
        .map(|i| {
            if let Some(p) = pod_at((i, 1), pods) {
                pods[p].typ.to_string()
            } else {
                " ".to_owned()
            }
        })
        .collect();
    println!(
        "  {}  ",
        bot.chars().flat_map(|i| [i, ' ']).collect::<String>()
    );

    println!("-----------------");
    println!();
}

#[derive(PartialEq, Eq)]
struct GameState {
    pods: Vec<Pod>,
    cost: u64,
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (u64::MAX - self.cost).partial_cmp(&(u64::MAX - other.cost))
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Pod {
    typ: PodType,
    pub pos: (usize, usize),
}

impl Pod {
    fn new(typ: PodType, r: usize, i: usize) -> Self {
        Self { typ, pos: (r, i) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PodType {
    A,
    B,
    C,
    D,
}

impl PodType {
    fn dest(&self) -> usize {
        match *self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn next_target(&self, pods: &Vec<Pod>) -> Option<(usize, usize)> {
        if let Some(a) = type_at((self.dest(), 1), pods) {
            if a == *self {
                if let Some(a) = type_at((self.dest(), 0), pods) {
                    if a == *self {
                        None
                    } else {
                        Some((self.dest(), 0))
                    }
                } else {
                    Some((self.dest(), 0))
                }
            } else {
                Some((self.dest(), 1))
            }
        } else {
            Some((self.dest(), 1))
        }
    }

    fn move_cost(&self) -> u64 {
        match *self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn to_string(&self) -> String {
        match *self {
            A => "A".to_owned(),
            B => "B".to_owned(),
            C => "C".to_owned(),
            D => "D".to_owned(),
        }
    }
}
