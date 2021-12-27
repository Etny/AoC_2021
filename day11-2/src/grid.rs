
pub struct OctoGrid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl OctoGrid {
    pub fn new<'a, T>(dec: T) -> Self
    where
        T: Iterator<Item = &'a str>,
    {
        let vec: Vec<_> = dec.collect();

        let width = vec[0].len();
        let height = vec.len();
        let grid = vec
            .into_iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();

        OctoGrid {
            grid,
            width,
            height,
        }
    }

    pub fn update(&mut self) -> bool {
        let mut to_flash = vec![];

        for i in 0..self.grid.len() {
            to_flash.extend(self.update_octo(i).into_iter());
        }

        let mut flashes = 0;

        while to_flash.len() > 0 {
            flashes += to_flash.len();

            let mut temp_vec = vec![];
            for index in to_flash.into_iter() {
                temp_vec.extend(self.flash(index));
            }

            to_flash = temp_vec;
        }

        if flashes == self.grid.len() {
            true
        } else {
            false
        }
    }

    fn flash(&mut self, index: usize) -> Vec<usize> {
        let mut new_flashes = vec![];
        let (x, y) = self.to_coords(index);

        for dx in -1..=1 {
            for dy in -1..=1 {

                if dx == 0 && dy == 0 {
                    continue;
                }

                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                
                if new_x < 0
                || new_x >= self.width as i32
                || new_y < 0
                || new_y >= self.height as i32
                {
                    continue;
                }

                let index = self.to_index(new_x as usize, new_y as usize);
                if self.grid[index] == 0 { continue; }
                new_flashes.extend(self.update_octo(index).into_iter());
            }
        }

        new_flashes
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn update_octo(&mut self, index: usize) -> Option<usize> {
        self.grid[index] += 1;
        if self.grid[index] > 9 {
            self.grid[index] = 0;
            Some(index)
        } else {
            None
        }
    }
}
