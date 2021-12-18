use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        list: [String; n],
    }

    let mut candidates: HashMap<String, usize> = HashMap::new();

    for s in list {
        match candidates.get_mut(&s) {
            Some(val) => *val = *val + 1,
            None => {
                candidates.insert(s, 1);
            },
        }
    }

    let mut max = 0;
    let mut winner = "".to_string();

    for (name, votes) in candidates {
        if votes > max {
            max = votes;
            winner = name;
        }
    }

    println!("{}", winner);
}
