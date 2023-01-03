use proconio::{fastout, input};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        mut a: [usize; k],
    }

    let mut dp = HashMap::new();

    for i in 1..=n {
        let max = a
            .iter()
            .map(|a_i| {
                if i < *a_i {
                    return 0;
                };
                i - *dp.get(&(i - a_i)).unwrap_or(&0)
            })
            .max()
            .unwrap();
        dp.insert(i, max);
    }

    println!("{}", dp.get(&n).unwrap());
}
