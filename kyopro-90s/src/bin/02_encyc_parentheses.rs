use proconio::{input, fastout};

use std::time::Instant;

// 2. Star: 3
// Encyclopedia of Parentheses
// bit全探索
#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let start = Instant::now();

    if n % 2 == 0 {
        for i in 0..(1 << n) {
            let mut candidate = String::new();
            for j in (0..n).rev() {
                if i & (1 << j) == 0 {
                    candidate.push('(');
                } else {
                    candidate.push(')');
                }
            }
            if check_candidate(candidate.as_str()) {
                println!("{}", candidate);
            }
        }
    }

    println!("{:?}", n);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

fn check_candidate(candidate: &str) -> bool {
    let mut deps: i32 = 0;
    for c in candidate.chars() {
        match c {
            '(' => deps += 1,
            ')' => deps -= 1,
            _ => (),
        }
        if deps < 0 {
            return false;
        }
    }
    if deps == 0 {
        true
    } else {
        false
    }
}
