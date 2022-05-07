use proconio::{input, fastout};
use std::collections::HashSet;
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_s: [usize; n],
        b_s: [usize; n],
    }

    let mut c_1 = 0;
    let mut c_2 = 0;

    let a_list: HashSet<usize> = HashSet::from_iter(a_s.iter().cloned());

    for i in 0..n {
        if a_s[i] == b_s[i] {
            c_1 += 1;
            continue;
        }
        if a_list.contains(&b_s[i]) {
            c_2 += 1;
        }
    }

    println!("{}", c_1);
    println!("{}", c_2);
}
