mod input;
use std::collections::HashMap;

use input::*;


//Struggled a bit with this one weirldy enough, even though it is basically identical to #6, which I found very easy
fn main() {
    let mut rules = HashMap::new();

    for line in INPUT_RULES.trim().lines() {
        let rule_dec = line.trim().split(" -> ").collect::<Vec<_>>();
        rules.insert(rule_dec[0].to_string(), rule_dec[1]);
    }    

    let occurences = run(INPUT_TEMPLATE.to_string(), &rules, 40);

    let min_occ = occurences.values().min().unwrap();
    let max_occ = occurences.values().max().unwrap();

    println!("{}", max_occ - min_occ);
}

fn run(template: String, rules: &HashMap<String, &str>, steps: u32) -> HashMap<char, u64> {
    let mut count = HashMap::new();

    for i in template.chars().collect::<Vec<_>>().windows(2) {
        *count.entry(i.iter().collect::<String>()).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut next_count = HashMap::new();

        for (key, count) in count {
            if rules.contains_key(&key) {
                let mut chars = key.chars();
                let left_pair = chars.next().unwrap().to_string() + rules[&key];
                let right_pair =  rules[&key].to_string() + &chars.next().unwrap().to_string();
                *next_count.entry(left_pair).or_insert(0) += count;
                *next_count.entry(right_pair).or_insert(0) += count;
            } else {
                next_count.insert(key, count);
            }
        }

        count = next_count;
    }

    let mut letter_count = HashMap::new();

    for (key, count) in count {
        *letter_count.entry(key.chars().next().unwrap()).or_insert(0) += count;
    }

    *letter_count.entry(template.chars().last().unwrap()).or_insert(0) += 1;

    letter_count 
}
