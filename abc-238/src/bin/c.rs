use proconio::{input, fastout};

const MOD: u64 = 998244353;
const MODULO_2: u64 = 499122177;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let digits = check_digits(n);

    let mut count: u64 = 0;

    let number = (n - 10u64.pow((digits - 1) as u32) + 1) % MOD;
    
    count = (count + (number * (number + 1)) % MOD * MODULO_2) % MOD;

    for i in (1..digits).rev() {
        let number = (10u64.pow(i as u32) - 10u64.pow((i - 1) as u32)) % MOD;
        count = (count + (number * (number + 1)) % MOD * MODULO_2) % MOD;
    }

    println!("{}", count);
}

fn check_digits(x: u64) -> usize {
    let mut x = x;
    let mut count = 0;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    count 
}

