use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut map = HashMap::new();

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut target = None;

    for (&c, &count) in map.iter() {
        if count == 1 {
            target = Some(c)
        }
    }

    match target {
        Some(c) => println!("{}", c),
        None => println!("-1"),
    }
}
