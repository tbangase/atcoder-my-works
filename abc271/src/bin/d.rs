use proconio::{fastout, input};

// TODO: Remove before submit.
use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (n, s): (usize, usize),
        cards: [(usize, usize); n],
    }

    // TODO: Remove before submit.
    let start = Instant::now();

    // Choices
    // 1. recursive function (dfs: depth first search)
    // 2. dp
    // how to solve
    // global search

    println!("{:?}", s);
    println!("{:?}", cards);

    // TODO: Remove before submit.
    println!("Duration: {:?}", start.elapsed());
}
