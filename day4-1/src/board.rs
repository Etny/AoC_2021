
pub struct Board {
    nums: Vec<u32>,
    marked: Vec<bool>
}

impl Board {
    pub fn new<'a, T>(lines: T) -> Self 
        where T: IntoIterator<Item = &'a str>
    {
        let mut nums = Vec::new();
        let marked = vec![false; 25];

        for num_line in lines {
            for num in num_line.to_string().split_ascii_whitespace() {
                nums.push(num.parse().unwrap());
            }
        }

        Board{
            nums,
            marked
        }
    }

    pub fn mark(&mut self, number: u32) -> Option<u32> {
        if let Some(index) = self.nums.iter().position(|n| *n == number){
            self.marked[index] = true;

            if self.check_win() {
                let mut total = 0;

                for (i, num) in self.nums.iter().enumerate() {
                    if !self.marked[i] { total += num; }
                }

                total *= number;

                return Some(total);
            }
        }

        None
    }

    fn check_win(&self) -> bool {
        if self.marked.chunks(5).any(|f| f.iter().all(|m| *m)) { return true; }

        'outer: for x in 0..5 {
            for y in 0..5 {
                if !self.marked[x + (y * 5)] { 
                    continue 'outer;  
                }
            }
            return true;
        }

        false
    }
}