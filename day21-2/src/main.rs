use std::collections::HashMap;

// Happy with this one
fn main() {
    let rolls = (1..=3)
        .flat_map(|i| (1..=3).flat_map(move |j| (1..=3).map(move |k| i + j + k)))
        .collect::<Vec<_>>();

    let mut score = HashMap::new();
    score.insert([0, 0], 1);

    let mut universes = HashMap::new();
    universes.insert([3, 7], score);

    let mut won: [u64; 2] = [0, 0];

    while !universes.is_empty() {
        let mut uni_next = HashMap::new();

        for p in 0..=1 {
            for key in universes.keys() {
                let scores = universes.get(key).unwrap();
                for roll in &rolls {
                    let mut new_pos = *key;
                    new_pos[p] = (key[p] + roll) % 10;

                    for s in scores.keys() {
                        let mut score_new = *s;
                        score_new[p] = s[p] + 1 + new_pos[p];

                        if score_new[p] >= 21 {
                            won[p] += scores[s];
                        } else {
                            *uni_next
                                .entry(new_pos)
                                .or_insert(HashMap::new())
                                .entry(score_new)
                                .or_insert(0) += scores[s];
                        }
                    }
                }
            }
            universes = uni_next;
            uni_next = HashMap::new();
        }
    }

    println!("{}", won[0].max(won[1]));
}
