use proconio::{input, fastout};

use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        p_list: [usize; n],
    }

    let mut status = vec![-2; n];
    let mut play_ground: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for i in 0..n {
        let p = p_list[i];
        let turn = i;

        let target = if let Some((&key, _)) = play_ground.range(p..).next() {
            key
        } else {
            p
        };

        if let Some(mut list) = play_ground.remove(&target) {
            list.push(target);
            if list.len() >= k - 1 {
                status[p - 1] = turn as i32;
                for num in list {
                    status[num - 1] = turn as i32;
                } 
            } else {
                play_ground.insert(p, list);
            }
        } else {
            if k <= 1 {
                status[p - 1] = turn as i32;
            } else {
                play_ground.insert(p, vec![]);
            }
        };
    }

    for turn in status {
        println!("{}", turn + 1);
    }
}
