use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    let mut i = 1;

    // 1. i = 1 ~ sqrt(n)
    while i < n / i {
        let plus = n / i;
        ans += n / i;
        println!("Phase 1: {}", plus);
        i += 1;
    }

    let mut k = 1;
    // 2. i = sqrt(n) ~ n
    while k * k <= n {
        let plus =n / k - n / (k + 1); 
        ans += (n / k - n / (k + 1)) * k;
        println!("Phase 2: {}", plus);
        k += 1;
    }

    println!("{:?}", ans);
}
