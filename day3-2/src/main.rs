
mod input;

use input::*;


fn main() {
    let input = INPUT.trim().lines();



    let mut lines_co2: Vec<_> = input.clone().collect();
    let mut lines_oxy: Vec<_> = input.clone().collect();
    let mut index = 0;

    while lines_oxy.len() > 1 || lines_co2.len() > 1 {
        let target_oxy = if pop_count(&lines_oxy, index) * 2 < lines_oxy.len()  { '0' } else { '1' };
        let target_co2 = if pop_count(&lines_co2, index) * 2 < lines_co2.len()  { '1' } else { '0' };

        if lines_oxy.len() > 1 {
            lines_oxy = lines_oxy.into_iter().filter(|l| l.chars().skip(index).next().unwrap() == target_oxy).collect();
        }

        if lines_co2.len() > 1 {
            lines_co2 = lines_co2.into_iter().filter(|l| l.chars().skip(index).next().unwrap() == target_co2).collect();
        }

        index += 1;
    }

    let oxygen = u32::from_str_radix(lines_oxy[0], 2).unwrap();
    let co2 = u32::from_str_radix(lines_co2[0], 2).unwrap();

    println!("{}", oxygen * co2);
}

fn pop_count(lines: &[&str], index: usize) -> usize {
    lines.into_iter().filter(|s| s.chars().skip(index).next().unwrap() == '1').count()
}
