// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(i32, i32, i32); n],  // Vec<(i32, i32, i32)>
    }

    println!("{:?}", s);
}
