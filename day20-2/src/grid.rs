pub struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
    default: u8
}

impl Grid {

    pub fn new(center: Vec<u8>, center_width: usize, default: u8) -> Self {
        let width = center_width + 2;
        let height = center.len() / center_width + 2;

        let mut data = vec![default; width];

        for c in center.chunks(center_width) {
            data.push(default);
            data.extend(c);
            data.push(default);
        }
        data.extend(vec![default; width]);
        Self { data, width, height, default }
    }

    pub fn next(self, alg: &Vec<u8>) -> Self {

        let mut new_data = vec![];

        for y in 0..(self.height as i32){
            for x in 0..(self.width as i32) {
                let mut res: usize = 0;

                for i in 0..9 {
                    let dx = (i % 3) - 1;
                    let dy = (i / 3) - 1;
                    res = (res << 1) | self.sample(x + dx, y + dy) as usize;
                }

                new_data.push(alg[res]);
            }
        }

        let new_default = if self.default == 0 { alg[0] } else { alg[511] };

        Self::new(new_data, self.width, new_default)
    }

    fn sample(&self, x: i32, y: i32) -> u8 {
        if x < 0 || x >= self.width as i32 { return self.default; }
        if y < 0 || y >= self.height as i32 { return self.default; }

        self.data[(x as usize) + ((y as usize) * self.width)]
    }

    pub fn pop_count(&self) -> usize {
        self.data.iter().filter(|d| **d == 1).count()
    }

}
