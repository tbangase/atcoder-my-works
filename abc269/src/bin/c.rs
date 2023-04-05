use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: i64,
    }

    let mut indices = vec![];
    let mut index = 0;
    while n > 0 {
        if n & 1 == 1 {
            indices.push(index);
        }
        n = n >> 1;
        index += 1;
    }

    let combinations = (0..indices.len())
        .flat_map(|i| indices.iter().combinations(i + 1))
        .collect::<Vec<_>>();

    println!("0");
    let mut results = combinations
        .iter()
        .map(|c| {
            c.iter()
                .map(|i| 2i64.pow(**i as u32))
                .fold(0, |acc, x| acc + x)
        })
        .collect::<Vec<_>>();
    results.sort();
    for result in results {
        println!("{}", result);
    }
}
