// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mat: [(i32, i32); n - 1],  // Vec<(i32, i32, i32)>
    }

    let mut is_star = true;
    let origin;

    let a0 = mat[0].0;
    let b0 = mat[0].1;

    let a1 = mat[1].0;
    let b1 = mat[1].1;

    if a0 == a1 {
        origin = a0
    } else if a0 == b1 {
        origin = a0
    } else if b0 == a1 {
        origin = b0
    } else if b0 == b1 {
        origin = b0
    } else {
        println!("No");
        return;
    };

    for i in 2..n - 1 {
        let a = mat[i].0;
        let b = mat[i].1;

        if a != origin && b != origin {
            is_star = false;
            break;
        }
    }

    match is_star {
        true => println!("Yes"),
        false => println!("No"),
    }
}
