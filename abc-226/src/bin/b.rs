use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut a_list = HashSet::new();

    for _ in 0..n {
        input! {
            l: usize,
            a: [u64; l]
        }

        a_list.insert(a);
    }

    println!("{}", a_list.len());

}

