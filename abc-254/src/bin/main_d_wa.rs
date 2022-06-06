use proconio::{input, fastout};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let start = Instant::now();

    let mut res = 0;

    let squart = (n as f64).sqrt() as u64;

    // pattern 1
    // 同じ数 もしくはそれに平方数かけたパターン
    for i in 0..n {
        res += 1;
        for j in 2..squart {
            // if i == ((i as f64).sqrt() as u64).pow(2) { continue }
            if i * j.pow(2) > n { break }
            // ?
            res += 2;
        }
    }
    
    // pattern 2
    // square numbers 
    res += (squart - 1) * squart;

    // pattern 3
    // 平方数 * 平方数 ... が n より小さい場合はそれを使える


    println!("{}", res);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
