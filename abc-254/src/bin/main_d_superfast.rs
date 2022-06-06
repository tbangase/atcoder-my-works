use proconio::{fastout, input};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let start = Instant::now();

    let mut ans = 0;
    let mut eq = vec![1; 1 + n]; 


    // why this is so fast??
    let mut counter: u64 = 0;

    for i in 1..=n {
        if eq[i] == 1 {
            for j in 2..=n {
                if i * j * j > n {
                    break;
                }
                eq[i] += 1;
                eq[i * j * j] = 0;
                counter += 1;
            }
        }
    }
    for i in 1..=n {
        ans += eq[i] * eq[i];
        counter += 1;
    }
    println!("{}", ans);
    println!("{}", counter);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
