pub struct HeightMap{
    map: Vec<u8>,
    width: usize,
    height: usize
}

impl HeightMap {
    pub fn new(lines: Vec<&str>) -> Self{
        let width = lines[0].len();
        let height = lines.len();
        let map = lines.into_iter().flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8)).collect();

        HeightMap{ map, width, height }
    }

    pub fn get_low_points(&self) -> Vec<u8> {
        (0..self.map.len()).map(|i| self.is_low_point(i)).flatten().collect()
    }

    fn is_low_point(&self, index: usize) -> Option<u8> {
        let (x, y) = self.to_coords(index);
        let this = self.get(x, y);

        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let dx = x as i32 + offset.0;
            let dy = y as i32 + offset.1;
            if dx < 0 || dy < 0 || dx >= self.width as i32 || dy >= self.height as i32 { continue; }

            let other = self.get(dx as usize, dy as usize);
            if other <= this { return None; }
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
        Some(self.map[self.to_index(x, y)])
    }
}