pub struct HeightMap {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

impl HeightMap {
    pub fn new(lines: Vec<&str>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let map = lines
            .into_iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();

        HeightMap { map, width, height }
    }

    pub fn get_basins(&self) -> impl Iterator<Item = u32> {
        let mut basin_map = vec![0usize; self.map.len()];

        let mut all = (0..self.map.len())
            .filter(|i| self.is_low_point(*i).is_some())
            .map(|i| self.check_basin(self.to_coords(i), i + 1, &mut basin_map))
            .collect::<Vec<_>>();

        all.sort();

        all.into_iter().rev().take(3)
    }

    fn check_basin(&self, coords: (usize, usize), low: usize, basin: &mut Vec<usize>) -> u32 {
        let mut res = 1;
        basin[self.to_index(coords.0, coords.1)] = low;

        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let dx = coords.0 as i32 + offset.0;
            let dy = coords.1 as i32 + offset.1;
            if dx < 0 || dy < 0 || dx >= self.width as i32 || dy >= self.height as i32 {
                continue;
            }

            if basin[self.to_index(dx as usize, dy as usize)] != 0 {
                continue;
            }
            if self.get(dx as usize, dy as usize) == Some(9) {
                continue;
            }

            res += self.check_basin((dx as usize, dy as usize), low, basin);
        }

        res
    }

    fn is_low_point(&self, index: usize) -> Option<u8> {
        let (x, y) = self.to_coords(index);
        let this = self.get(x, y);

        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let dx = x as i32 + offset.0;
            let dy = y as i32 + offset.1;
            if dx < 0 || dy < 0 || dx >= self.width as i32 || dy >= self.height as i32 {
                continue;
            }

            let other = self.get(dx as usize, dy as usize);
            if other <= this {
                return None;
            }
        }

        self.get(x, y)
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.map[self.to_index(x, y)])
        }
    }
}
