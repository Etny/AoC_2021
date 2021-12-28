use std::collections::{HashSet, BinaryHeap};

pub struct RiskMap {
    map: Vec<u32>,
    width: usize,
}

impl RiskMap {
    pub fn new<'a, T>(dec: T) -> Self
    where
        T: Iterator<Item = &'a str>,
    {
        let full_dec: Vec<_> = dec.collect();

        let part_width = full_dec[0].len();
        let part_height = full_dec.len();
        let map_part: Vec<_> = full_dec
            .into_iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();

        let mut map: Vec<_> = vec![];

        for row_outer in 0..5 { 
            for row_inter in 0..part_height {
                map.extend((0..part_width * 5).map(|i| {
                    let new = map_part[(row_inter * part_width) + i % part_width] + row_outer + (i / part_width) as u32;
                    match new {
                        0..=9 => new,
                        _ => new % 9
                    }
                }));
            }
        }

        RiskMap { map, width: part_width * 5 }
    }

    pub fn dijkstra(&self) -> u32 {
        let mut distance = BinaryHeap::new();
        let mut visited = HashSet::new();

        distance.push((u32::MAX, 0));

        loop {
            let (dist, closest) = distance.pop().unwrap();
            if closest == self.map.len() - 1 { return u32::MAX - dist }
            if visited.contains(&closest) { continue; }

            for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some(index) = self.check_offset(closest, offset) {
                    distance.push((dist - self.map[index], index));
                }
            }

            visited.insert(closest);
        }

    }

    fn check_offset(&self, base: usize, offset: (i32, i32)) -> Option<usize> {
        let new_x = (base % self.width) as i32 + offset.0;
        let new_y = (base / self.width) as i32 + offset.1;

        if new_x < 0 || new_y < 0 {
            None
        } else {
            let new = (new_y * self.width as i32 + new_x) as usize;

            if new >= self.map.len() {
                None
            } else {
                Some(new)
            }
        }
    }
}
