// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let cs: Vec<char> = s.chars().collect();

    let mut same_count = 0;

    if cs[0] == cs[1] {
        same_count += 1
    };
    if cs[0] == cs[2] {
        same_count += 1
    };
    if cs[1] == cs[2] {
        same_count += 1
    };

    match same_count {
        0 => println!("6"),
        1 => println!("3"),
        2 => println!("1"),
        3 => println!("1"),
        _ => println!("Err"),
    }
}
