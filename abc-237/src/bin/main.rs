use proconio::{input, fastout, marker::*};

use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        mut n: usize,
        mut s: Chars
    }

    let mut buf = VecDeque::new();
    buf.push_back(n);

    for i in (0..n).rev() {
        if s[i] == 'L' {
            buf.push_back(i);
        } else {
            buf.push_front(i);
        }
    }

    for num in buf {
        print!("{} ", num);
    }

    println!("");
}
