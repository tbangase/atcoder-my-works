use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [u64; n],
        x_k: [(u64, usize); q],
    }

    let mut index_list: HashMap<u64, Vec<usize>> = HashMap::new();

    for (i, num) in a.iter().enumerate() {
        if index_list.contains_key(num) {
            if let Some(list) = index_list.get_mut(num) {
                list.push(i);
            }
        } else {
            index_list.insert(*num, vec![i]);
        }
    }

    for (x, k) in x_k {
        let mut result = -1;
        if let Some(appearance) = index_list.get(&x) {
            if let Some(res) = appearance.get(k - 1) {
                result = (res + 1) as i32;
            }
        }
        println!("{}", result);
    }
}
