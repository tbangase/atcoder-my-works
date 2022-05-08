use proconio::{input, fastout};

use std::collections::HashSet;
// use std::{time::Instant, collections::HashSet};

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    // let start = Instant::now();

    let mut p = 2;
    let mut count = 0;
    let mut num_sets = HashSet::new();

    loop {
        if !is_prime(p) {
            p += 1;
            continue;
        }
        let mut q = p + 1;
        let mut k = calc_k(p, q);

        if k > n { break; }

        while k <= n {
            // println!("P: {}, Q: {}", p, q);
            if is_prime(q) && num_sets.get(&q).is_none() {
                count += 1;
                num_sets.insert(k);
            }
            q += 1;
            k = calc_k(p, q);
        }
        p += 1;
    }

    println!("{}", count);
    // let duration = start.elapsed();
    // println!("Duration: {:?}", duration);
}

fn calc_k(p: u64, q: u64) -> u64 {
    p * q.pow(3) 
}

fn is_prime(n: u64) -> bool {
    if n < 4 {
      n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
      false
    } else {
      let max_p = (n as f64).sqrt().ceil() as u64;
      match (5..=max_p).step_by(6).find(|p| n % p == 0 || n % (p+2) == 0) {
        Some(_) => false,
        None => true
      }
    }
}

