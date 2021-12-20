use proconio::{input, fastout, marker::*};

use std::time::Instant;
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
        cd: [(usize, usize); m],
    }

    let start = Instant::now();

    let mut taka = vec![vec![false; n]; n];
    let mut aoki = vec![vec![false; n]; n];

    for i in 0..n {
        taka[ab[i].0 - 1][ab[i].1 - 1] = true;
        taka[ab[i].1 - 1][ab[i].0 - 1] = true;
        aoki[cd[i].0 - 1][cd[i].1 - 1] = true;
        aoki[cd[i].1 - 1][cd[i].0 - 1] = true;
    }

    // Iterate permutatively
    for p in (0..n).permutations(n) {
        println!("{:?}", p);
    }

    println!("{:?}", taka);
    println!("{:?}", aoki);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

