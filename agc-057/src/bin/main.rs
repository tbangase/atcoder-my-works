use proconio::{input, fastout};

use std::{time::Instant, collections::HashSet};

#[fastout]
fn main() {
    input! {
        t: usize,
        cases: [(usize, usize); t],
    }

    println!("{:?}", cases);
    let start = Instant::now();

    for (l, r) in cases.iter() {
        let mut count = 0;
        let mut used = HashSet::new();

        for i in (*l..=*r).rev() {
            if None == used.get(&i.to_string()) {
                count += 1;
                // Add substrings of i
                let i_str = i.to_string();

                for i in 1..i_str.len() {
                    
                }

                used.insert(i_str);
            }
        }
        println!("{}", count);
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
