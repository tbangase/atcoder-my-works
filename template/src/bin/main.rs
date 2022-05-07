use proconio::{input, fastout};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_list: [usize; n],
    }

    let start = Instant::now();

    let mut count = 0;

    count += 1;

    println!("{:?}", a_list);
    println!("{}", count);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
