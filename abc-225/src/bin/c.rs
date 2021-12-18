// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        b: [[u64; m]; n],  // Vec<(i32, i32, i32)>
    }

    for row in 0..n {
        for col in 0..m {
            // 1. check is continuous
            if col != 0 && b[row][col] != b[row][col - 1] + 1 {
                println!("No");
                return;
            }
            // 2. check no multiple of 7 except last col
            if col != m - 1 && b[row][col] % 7 == 0 {
                println!("No");
                return;
            }
            // 3. check 7 added from above number
            if row != 0 && b[row][col] != b[row - 1][col] + 7 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes")
}
