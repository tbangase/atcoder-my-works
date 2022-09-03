use proconio::{fastout, input};

use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        (p, q, r): (u64, u64, u64),
        a: [usize; n],
    }

    let mut p_able = HashMap::new();
    let mut q_able = HashMap::new();
    let mut r_able = HashMap::new();

    let mut memo = HashMap::new();
    let stash_border = p.max(q.max(r));

    for (i, a_i) in a.iter().enumerate() {
        // From: Current value の HashMap
        let mut todo_remove = vec![];
        memo.iter_mut().for_each(|(&k, v)| {
            *v += *a_i;
            if *v as u64 > stash_border {
                todo_remove.push(k);
            }
            if *v as u64 == p {
                p_able.insert(k, i);
            }
            if *v as u64 == q {
                q_able.insert(k, i);
            }
            if *v as u64 == r {
                r_able.insert(k, i);
            }
        });
        todo_remove.iter().for_each(|k| {
            memo.remove(k);
        });
        memo.insert(i, *a_i);
        if *a_i as u64 == p {
            p_able.insert(i, i);
        }
        if *a_i as u64 == q {
            q_able.insert(i, i);
        }
        if *a_i as u64 == r {
            r_able.insert(i, i);
        }
    }

    let mut ans = false;
    for (_, &v) in p_able.iter() {
        if let Some(v) = q_able.get(&(v + 1)) {
            if r_able.get(&(v + 1)).is_some() {
                ans = true;
                break;
            }
        }
    }

    match ans {
        true => println!("Yes"),
        false => println!("No"),
    }
}
