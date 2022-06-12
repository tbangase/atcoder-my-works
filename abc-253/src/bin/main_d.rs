use proconio::input;

// use std::time::Instant;

// #[fastout]
fn main() {
    input! {
        (n, a, b): (u64, u64, u64),
    }

    // let start = Instant::now();

    // n以下で aの倍数でも bの倍数でもない数の総和
    // n以下の総和 = n(n+1)/2
    // n以下で、aの倍数またはbの倍数の数の総和

    let sum = n * (n + 1) / 2;

    let a_n = n / a;
    let a_multi_sum = a_n * (a_n + 1) / 2 * a;

    let b_n = n / b;
    let b_multi_sum = b_n * (b_n + 1) / 2 * b;

    let lcm = a * b / gcd(a, b);
    let lcm_n = n / lcm;
    let lcm_multi_sum = lcm_n * (lcm_n + 1) / 2 * lcm; 

    // println!("Sum    : {sum}");
    // println!("LCM Sum: {lcm_multi_sum}");
    // println!("A Sum  : {a_multi_sum}");
    // println!("B Sum  : {b_multi_sum}");

    let total = sum + lcm_multi_sum - a_multi_sum - b_multi_sum;

    println!("{}", total);

    // let duration = start.elapsed();
    // println!("Duration: {:?}", duration);
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a
    } else {
        return gcd(b, a % b)
    }
}
