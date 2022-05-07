use proconio::{input, fastout};

use std::time::Instant;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_s: [usize; n],
    }

    let start = Instant::now();

    let mut stack = vec![];
    let mut current_state = (0,0);
    let mut count = 0;

    for a in a_s {
        count += 1;
        stack.push(a);
        if current_state.0 != a {
            current_state.0 = a;
            current_state.1 = 1;
        } else {
            current_state.1 += 1;
            if current_state.1 == current_state.0 {
                count -= current_state.0;
                for _ in 0..current_state.0 {
                    stack.pop();
                }
                // current_stateの更新
            }
        }
        println!("{:?}", count);
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
