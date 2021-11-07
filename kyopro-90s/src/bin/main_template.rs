use proconio::fastout;
use proconio::input;

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (n, l, k): (usize, usize, u32),
        a: [u32; n],
    }

    let start = Instant::now();

    a.sort();
    println!("{:?}", a)
    let duration = start.elapsed();

}
