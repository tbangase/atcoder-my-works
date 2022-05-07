use proconio::{input, fastout, marker::*};

use std::time::Instant;

// 0. Star: 9 
// NP Problem
// Do your own best
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(u64, u64, u64); n],
    }

    let start = Instant::now();

    println!("{:?}", s);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
