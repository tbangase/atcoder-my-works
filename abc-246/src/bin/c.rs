use proconio::{input, fastout};

use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        (n, k, x): (usize, usize, usize),
        a_list: [usize; n],
    }

    let mut remain_list = BinaryHeap::new();
    let mut coupon_num = k;

    for a in a_list {
        if coupon_num < a / x {
            // クーポンの残り枚数が足りていなかったら
            remain_list.push(a - coupon_num * x);
            coupon_num = 0;
            continue;
        } else {
            // 足りていたら
            remain_list.push(a % x);
            coupon_num -= a / x;
        }
    }

    // 残っているクーポンを大きい順で使う
    let mut total_cost = 0;
    while !remain_list.is_empty() {
        let mut cost = remain_list.pop().unwrap();

        if coupon_num > 0 {
            cost = 0;
            coupon_num -= 1;
        }

        total_cost += cost;
    }
    println!("{}", total_cost);
}
