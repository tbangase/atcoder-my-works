use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, q, x): (usize, usize, usize),
        w: [usize; n],
    }

    let mut w_index = 0;
    let mut box_index = 0usize;
    let mut x_over_index = HashMap::new();
    let box_init;
    let mut box_iter = vec![];
    let mut box_count = 0;
    let mut current_box_usage = 0;

    x_over_index.insert(n-1, 0);

    loop {
        box_count += 1;

        current_box_usage += w.get(w_index).unwrap_or_else(|| &0);

        if current_box_usage >= x {
            box_index += 1;
            box_iter.push(box_count);
            box_count = 0;
            current_box_usage = 0;

            if let Some(&loop_start_b_index) = x_over_index.get(&w_index) {
                box_init = box_iter[0..loop_start_b_index].to_vec();
                box_iter = box_iter[loop_start_b_index..].to_vec();
                // println!("Box Loop start index {}", loop_start_b_index);
                // println!("End with Index: {}", w_index);
                // println!("X over Index: {:?}", x_over_index);
                break;
            }

            // println!("Box index {} end with w_index {}", box_index, w_index);
            x_over_index.insert(w_index, box_index);
        }

        if w_index == n - 1 {
            w_index = 0;
        } else {
            w_index += 1;
        }
    }

    // println!("Box init: {:?}", box_init);
    // println!("Box iter: {:?}", box_iter);

    let init_len = box_init.len() as u64;
    let iter_len = box_iter.len() as u64;

    for _ in 0..q {
        input! {
            k: u64
        }

        if k <= init_len {
            let i = (k - 1) as usize;
            println!("{}", box_init.get(i).unwrap_or(&0));
        } else {
            let k = k - 1;
            let k = if k != 0 {
                (k - init_len) % iter_len
            } else {
                k
            };

            println!("{}", box_iter.get(k as usize).unwrap_or(&0));
        }
    }
}
