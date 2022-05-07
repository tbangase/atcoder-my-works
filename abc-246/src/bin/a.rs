use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        points: [(isize, isize); 3],
    }

    let mut x_map = HashMap::new();
    let mut y_map = HashMap::new();

    for (x_i, y_i) in points {
        *x_map.entry(x_i).or_insert(0) += 1;
        *y_map.entry(y_i).or_insert(0) += 1;
    }

    let (mut x, mut y) = (0, 0);

    for (k,v) in x_map.iter() {
        if *v == 1 {
            x = *k;
        }
    }
    for (k,v) in y_map.iter() {
        if *v == 1 {
            y = *k;
        }
    }

    println!("{} {}", x, y);
}
