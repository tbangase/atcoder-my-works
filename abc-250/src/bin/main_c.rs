use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        queries: [usize; q],
    }

    // Key: x, Value: index
    let mut index_map = HashMap::new();
    let mut list = (1..=n).collect::<Vec<_>>();

    for i in 1..=n {
        index_map.insert(i, i - 1);
    }

    for x in queries {
        let target_num;
        {
            let index = index_map.get_mut(&x).unwrap();
            let target = if *index == (n - 1) {
                *index - 1
            } else {
                *index + 1
            };
            target_num = list[target];
            list.swap(*index, target);
        }
        let tmp = index_map.get(&x).unwrap().clone();
        *index_map.get_mut(&x).unwrap() = *index_map.get(&target_num).unwrap();
        *index_map.get_mut(&target_num).unwrap() = tmp;
    }

    for i in 0..n {
        print!("{} ", list[i]);
    }
    println!("");
}
