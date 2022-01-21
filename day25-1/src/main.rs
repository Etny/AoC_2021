mod input;

use input::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut grid = Grid::parse(INPUT.trim().lines());

    let steps = grid.get_steps();
    println!("{}", steps);
}

struct Grid {
    data: HashMap<(usize, usize), Cucumber>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone)]
struct Cucumber {
    moves_east: bool,
}

impl Grid {
    fn get_steps(&mut self) -> u32 {
        let mut steps = 0;

        let mut data = self.data.clone();
        let mut to_move = data.keys().map(|k| *k).collect::<Vec<_>>();
        let mut blocking = HashMap::new();

        while !to_move.is_empty() {
            let mut next_move = vec![];
            let mut moved = HashSet::new();

            for pos in to_move.iter().filter(|p| self.data[*p].moves_east) {
                self.update_cucumber(*pos, &mut data, &mut moved, &mut blocking, &mut next_move)
            }

            self.data = data.clone();

            //Any down-moving cucumber that becomes unblocked by an east moving cucumber has to be updated immediately, rather than next tick
            let (next_move_east, mut next_move_down) =
                next_move.iter().partition(|c| self.data[*c].moves_east);
            next_move = next_move_east;
            to_move.append(&mut next_move_down);

            for pos in to_move
                .iter()
                .filter(|p| self.data.get(*p).map_or(false, |c| !c.moves_east))
            {
                self.update_cucumber(*pos, &mut data, &mut moved, &mut blocking, &mut next_move)
            }

            self.data = data.clone();
            to_move = next_move;
            steps += 1;
        }

        steps
    }

    fn update_cucumber(
        &self,
        pos: (usize, usize),
        data: &mut HashMap<(usize, usize), Cucumber>,
        moved: &mut HashSet<(usize, usize)>,
        blocking: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
        next_move: &mut Vec<(usize, usize)>,
    ) {
        let cucumber = data.remove(&pos).expect(&format!("{:?}", pos));

        let next_pos = cucumber.next_pos(pos, &self);

        match self.data.get(&next_pos) {
            Some(_) => {
                data.insert(pos, cucumber);
                if !moved.contains(&next_pos) {
                    blocking.entry(next_pos).or_insert(vec![]).push(pos);
                } else {
                    next_move.push(pos);
                }
            }
            None => {
                data.insert(next_pos, cucumber);
                next_move.push(next_pos);
                moved.insert(pos);
                if let Some(mut blocked) = blocking.remove(&pos) {
                    next_move.append(&mut blocked);
                }
            }
        }
    }

    fn parse<'a, T>(dec: T) -> Self
    where
        T: Iterator<Item = &'a str>,
    {
        let mut width = 0;
        let mut height = 0;

        let mut data = HashMap::new();

        for l in dec {
            let chars = l.chars().collect::<Vec<_>>();
            for x in 0..chars.len() {
                match chars[x] {
                    '>' => data.insert((x, height), Cucumber::new(true)),
                    'v' => data.insert((x, height), Cucumber::new(false)),

                    _ => None,
                };
            }

            height += 1;
            width = l.len();
        }

        Self {
            data,
            width,
            height,
        }
    }
}

impl Cucumber {
    fn next_pos(&self, pos: (usize, usize), grid: &Grid) -> (usize, usize) {
        match self.moves_east {
            true => ((pos.0 + 1) % grid.width, pos.1),
            false => (pos.0, (pos.1 + 1) % grid.height),
        }
    }

    fn new(moves_east: bool) -> Self {
        Self { moves_east }
    }
}

// impl Display for Cucumber {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", if self.moves_east { ">" } else { "v" })
//     }
// }

// impl Display for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(f)?;
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 if self.data.contains_key(&(x, y)) {
//                     write!(f, "{}", self.data[&(x, y)])?
//                 } else {
//                     write!(f, ".")?
//                 }
//             }
//             writeln!(f)?
//         }
//         writeln!(f)?;
//         writeln!(f, " -------------")?;

//         Ok(())
//     }
// }
