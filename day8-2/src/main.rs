mod combinations;
mod entry;
mod input;

use std::collections::HashMap;

use combinations::*;
use entry::*;
use input::*;


//Brute forced... not proud of this one TBH
fn main() {
    let comb = Combinations::new("abcdefg".chars().collect::<Vec<char>>());

    let mut conversions = HashMap::new();
    conversions.insert(vec![2, 5], 1);
    conversions.insert(vec![0, 2, 3, 4, 6], 2);
    conversions.insert(vec![0, 2, 3, 5, 6], 3);
    conversions.insert(vec![1, 2, 3, 5], 4);
    conversions.insert(vec![0, 1, 3, 5, 6], 5);
    conversions.insert(vec![0, 1, 3, 4, 5, 6], 6);
    conversions.insert(vec![0, 2, 5], 7);
    // 8 Excluded
    conversions.insert(vec![0, 1, 2, 3, 5, 6], 9);
    conversions.insert(vec![0, 1, 2, 4, 5, 6], 0);

    let mut possible = HashMap::new();
    for k in conversions.keys() {
        possible.entry(k.len()).or_insert(vec![]).push(k.to_vec());
    }

    let mut entries = vec![];

    for line in INPUT.trim().lines() {
        entries.push(Entry::new(line.to_string()));
    }

    let mut total = 0;

    for c in comb {
        for entry in &mut entries {
            if let Some(output) = entry.try_combination(&c, &possible, &conversions) {
                total += output;
            }
        }

        entries = entries.into_iter().filter(|e| !e.done).collect();
    }

    println!("{}", total)
}
