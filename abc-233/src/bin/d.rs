use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, k): (usize, i64),
        a_list: [i64; n],
    }

    let mut cumlative_sum = vec![0];
    let mut sum = 0;

    for a in a_list {
        sum += a;
        cumlative_sum.push(sum);
    }

    let mut count = 0;
    let mut map: HashMap<i64, usize> = HashMap::new();

    for r in 0..n {
        match map.get_mut(&cumlative_sum[r]) {
            Some(val) => *val += 1,
            None => {
                map.insert(cumlative_sum[r], 1);
            },
        };
        count += map.get(&(cumlative_sum[r+1] - k)).unwrap_or(&0);
    }

    println!("{}", count);
}
