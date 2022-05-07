use proconio::{input, fastout};

use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_s: [usize; n]
    }

    let mut set = HashSet::new();

    for a in a_s {
        set.insert(a);
    }

    println!("{:?}", set.len());
}
