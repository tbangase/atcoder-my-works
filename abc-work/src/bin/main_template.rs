use proconio::fastout;
use proconio::input;

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: i32,
        s: [(i32, i32, i32); n],
    }

    let start = Instant::now();

    println!("{:?}", s)
    let duration = start.elapsed();

}
