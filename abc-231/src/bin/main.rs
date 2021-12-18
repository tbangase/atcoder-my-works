use proconio::{input, fastout};

use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        (_n, m): (usize, usize),
        a_bs: [(usize, usize); m],
    }

    let mut neighbors: HashMap<usize, HashSet<usize>> = HashMap::new();

    for a_b in a_bs {
        neighbors.entry(&a_b.0).or_insert(|| {

        });
        // match neighbors.get_mut(&a_b.0) {
        //     Some(v) => {
        //         v.insert(a_b.1);
        //         if v.len() > 2 {
        //             println!("No");
        //             return
        //         }
        //     }
        //     None => {
        //         neighbors.insert(
        //             a_b.0,
        //             vec![a_b.1].into_iter().collect()
        //         );
        //     }
        // }

        match neighbors.get_mut(&a_b.1) {
            Some(v) => {
                v.insert(a_b.0);
                if v.len() > 2 {
                    println!("No");
                    return
                }
            }
            None => {
                neighbors.insert(
                    a_b.1,
                    vec![a_b.0].into_iter().collect()
                );
            }
        }
    }

    println!("Yes");
}

