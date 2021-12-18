use proconio::{input, fastout, marker::*};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        mut a: Chars,
        mut b: Chars,
    }

    let start = Instant::now();

    a.sort();
    b.sort();

    loop {
        let mut target = '0';
        let index = match a.binary_search(&target) {
            Ok(val) => val,
            Err(val) => val,
        };
        target += 1;
        break;
    }



    for c in a {
        print!("{}", c);
    }
    println!("");

    for c in b {
        print!("{}", c);
    }
    println!("");

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
