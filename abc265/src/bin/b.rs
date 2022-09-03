use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, m, mut t): (usize, usize, isize),
        a: [isize; n - 1],
        x_y: [(usize, isize); m],
    }

    let bonus_map: HashMap<_, _> = x_y.into_iter().collect();

    for i in 0..n - 1 {
        let bonus = bonus_map.get(&(i + 1)).unwrap_or(&0);
        t -= a[i] - *bonus;
        if t <= 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
