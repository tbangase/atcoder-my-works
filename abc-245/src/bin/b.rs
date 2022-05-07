use proconio::{input, fastout};

use std::{collections::HashSet, iter::FromIterator};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a: HashSet<usize> = HashSet::from_iter(a.iter().cloned());

    let mut i = 0;

    loop {
        if let None = a.get(&i) {
            break;
        }
        i += 1;
    }

    println!("{}", i);
}
