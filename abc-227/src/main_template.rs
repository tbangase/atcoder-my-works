use proconio::{input, fastout};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(i32, i32, i32); n],
    }

    let start = Instant::now();

    println!("{:?}", s)
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
