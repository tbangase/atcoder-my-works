use proconio::{input, fastout};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let start = Instant::now();
    let mut ans = 0;
    let mut counter: u64 = 0;

    for i in 1..=n {
        let mut k = i;
        // i とペアになりうる最小の相手 j を探す
        // i の素因数の中に平方数が隠れているならそれを排除した値
        for d in 2..=k {
            counter += 1;
            if d*d <= k {
                break;
            }
            while k % (d*d) == 0 {
                // k は 平方数で割れるだけ割っていく
                k /= d*d;
                counter += 1;
            }
        }

        // ここで k は i とペアになりうる最小の相手 j になっている
        // i に対するペアは全て j に平方数をかけたで表せるのでそれらをカウントする
        // ans += (1..=n).take_while(|&i| k*i*i <= n).count();
        let mut d = 1;
        while k*d*d <= n {
            ans += 1;
            d += 1;
        }
    }

    println!("{}", ans);
    println!("{}", counter);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
