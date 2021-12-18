use proconio::{input, fastout};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32, i32); n],
    }

    let start = Instant::now();

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
