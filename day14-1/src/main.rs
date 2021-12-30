mod input;
use std::collections::HashMap;

use input::*;

fn main() {
    let mut rules = HashMap::new();

    for line in INPUT_RULES.trim().lines() {
        let rule_dec = line.trim().split(" -> ").collect::<Vec<_>>();
        rules.insert(rule_dec[0].to_string(), rule_dec[1]);
    }

    let polymer = INPUT_TEMPLATE.to_string();
    
    let mut occurences = HashMap::new();

    //count initial characters;
    for c in polymer.chars() {
        *occurences.entry(c).or_insert(0) += 1;
    }

    for pair in polymer.chars().collect::<Vec<_>>().windows(2).map(|c| c.iter().collect::<String>()) {
        run(pair, &rules, &mut occurences, 0, 10);
    }
   
    let mut occurences_sorted = occurences.values().collect::<Vec<_>>();
    occurences_sorted.sort();

    let output = *occurences_sorted.last().unwrap() - *occurences_sorted.first().unwrap();

    println!("{:?}", occurences);

    println!("{}", output);

}

fn run(mut case: String, rules: &HashMap<String, &str>, occs: &mut HashMap<char, u64>, iteration: u32, limit: u32) {
    if iteration >= limit { return; }
    if !rules.contains_key(&case) { return; }

    *occs.entry(rules[&case].chars().next().unwrap()).or_insert(0) += 1;
    case.insert_str(1, rules[&case]);


    for new_pair in case.chars().collect::<Vec<_>>().windows(2).map(|c| c.iter().collect::<String>()) {
        run(new_pair, rules, occs, iteration+1, limit)
    }
   
}
