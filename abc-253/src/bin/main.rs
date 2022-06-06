use proconio::{input, fastout, marker::*};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    let start = Instant::now();

    // n以下で aの倍数でも bの倍数でもない数の総和
    // n以下の総和 = n(n+1)/2
    // n以下で、aの倍数またはbの倍数の数の総和

    let sum = n * (n + 1) / 2;
    let mut a_multi_sum = 0;
    for i in 0..n {
        if i * a > sum {
            break;
        }
        a_multi_sum += i * a;
    }

    let mut b_multi_sum = 0;
    for i in 0..n {
        if i * b > sum {
            break;
        }
        b_multi_sum += i * b;
    }

    println!("{}", total);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
