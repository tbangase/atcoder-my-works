use proconio::{input, marker::*};

use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        points: [(usize, usize); n],
        s: Chars,
    }

    let mut left_max = HashMap::new();
    let mut right_min = HashMap::new();

    let yes = || { println!("Yes"); std::process::exit(0) };

    for (i, (x, y)) in points.iter().enumerate() {
        // 衝突判定
        if s[i] == 'R' {
            // left_maxより小さかったら衝突
            if let Some(val) = left_max.get(y) {
                if x < val { yes() }
            }
        } else {
            // right_minより大きかったら衝突
            if let Some(val) = right_min.get(y) {
                if x > val { yes() }
            }
        }

        // 更新
        if s[i] == 'R' {
            // 高さに対して
            // right_minより小さかったら更新
            if let Some(val) = right_min.get_mut(y) {
                *val = *x.min(val)
            } else {
                right_min.insert(y, *x);
            }
        } else {
            if let Some(val) = left_max.get_mut(y) {
                *val = *x.max(val)
            } else {
                left_max.insert(y, *x);
            }
            
        }
    }

    println!("No");
}
