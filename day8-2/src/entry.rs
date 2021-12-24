use std::collections::HashMap;

pub struct Entry {
    signal: Vec<String>,
    output: Vec<String>,
    pub done: bool
}

impl Entry {
    pub fn new(dec: String) -> Self {
        let split: Vec<_> = dec.split(" | ").collect();

        let signal = split[0].split(' ').map(|s| s.to_string()).collect();
        let output = split[1].split(' ').map(|s| s.to_string()).collect();

        Entry { signal, output, done: false }
    }

    pub fn try_combination(
        &mut self,
        comb: &Vec<char>,
        possible: &HashMap<usize, Vec<Vec<usize>>>,
        conversions: &HashMap<Vec<usize>, i32>,
    ) -> Option<i32> {
        for digit in &self.signal {
            if digit.len() >= 7 {
                continue;
            }

            let mut display: Vec<_> = digit
                .chars()
                .map(|c| comb.iter().position(|p| p == &c).unwrap())
                .collect();
            display.sort();
            if !possible[&digit.len()].contains(&display) {
                return None;
            }
        }

        let mut output_num = 0;

        for digit in &self.output {
            let mut val = 8;

            if digit.len() < 7 {
                let mut display: Vec<_> = digit
                    .chars()
                    .map(|c| comb.iter().position(|p| p == &c).unwrap())
                    .collect();
                display.sort();

                if !possible[&digit.len()].contains(&display) {
                    return None;
                } else {
                    val = conversions[&display];
                }
            }

            output_num = output_num * 10 + val;
        }

        self.done = true;

        Some(output_num)
    }
}
