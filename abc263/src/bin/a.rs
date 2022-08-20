use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        cards: [usize; 5],
    }

    let mut map = HashMap::new();

    for &card in cards.iter() {
        *map.entry(card).or_insert(0) += 1;
    }

    if map.len() == 2 {
        for (_, &v) in map.iter() {
            if v == 2 || v == 3 {
                println!("Yes");
                return 
            }
        }
    }

    println!("No");
}
