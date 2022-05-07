use proconio::{input, fastout};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        mut n: u64,
    }

    let start = Instant::now();

    loop {
        if check_has(n) {
            break;
        }
        n += 1;
    }

    println!("{:?}", n);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

fn check_has(n: u64) -> bool {
    // 3乗してとどくところからスタート
    // 割り切れる値を取ってくる
    // 足したらその割り切れる値で、a^2 + b^2が商になる数字を調べる
    // 最低が商を超えたら打ち切り
    if n == 0 {
        return true;
    }
    for i in 1..=n {
        let i = i as u64;
        if i * i > n {
            break;
        }
        // if i.pow(3) < n {
        //     continue;
        // }
        if n % i == 0 {
            let product = n / i;
            let mut min = 0;
            for j in 0..=((i as usize)/2) {
                let j = j as u64;
                min = j.pow(2) + (i - j).pow(2);
                if min == product {
                    return true
                }
            }
            if min > product {
                break;
            }
        }
    }
    false
}
