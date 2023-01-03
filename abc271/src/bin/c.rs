use itertools::Itertools;
use proconio::{fastout, input};

use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut stack = {
        let len = a.len();
        a.dedup();
        len - a.len()
    };

    let mut a = VecDeque::from(a);

    let mut current = 0;

    while !a.is_empty() || stack > 1 {
        if !a.is_empty() {
            if current >= a[0] {
                a.pop_front();
                stack += 1;
                continue;
            } else if (current + 1) == a[0] {
                a.pop_front();
                current += 1;
                continue;
            }
        }
        if stack >= 2 {
            stack -= 2;
            current += 1;
        } else {
            a.pop_back();
            if stack > 0 {
                stack -= 1;
                current += 1;
                continue;
            }
            if a.is_empty() {
                break;
            }
            a.pop_back();
            current += 1;
        }
    }

    println!("{}", current);
}
