use num::integer::lcm;
use proconio::input;

fn main() {
    input! {
        (n, a, b): (u64, u64, u64),
    }

    let sum = n * (n + 1) / 2;

    let a_n = n / a;
    let a_multi_sum = a_n * (a_n + 1) / 2 * a;

    let b_n = n / b;
    let b_multi_sum = b_n * (b_n + 1) / 2 * b;

    let lcm = lcm(a,b);
    let lcm_n = n / lcm;
    let lcm_multi_sum = lcm_n * (lcm_n + 1) / 2 * lcm; 

    let total = sum + lcm_multi_sum - a_multi_sum - b_multi_sum;

    println!("{}", total);
}
