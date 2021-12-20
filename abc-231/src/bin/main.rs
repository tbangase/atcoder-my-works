use petgraph::unionfind::UnionFind;
use proconio::{input, fastout};

use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a_bs: [(usize, usize); m],
    }

    let mut neighbors = UnionFind::new(n);

    for a_b in a_bs {
        if neighbors.equiv(a_b.0, a_b.1)  {
            let representative = neighbors.find(2);
            println!("Representative of 2: {}", representative);
        }
        neighbors.union(a_b.0, a_b.1);

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

        // match neighbors.get_mut(&a_b.1) {
        //     Some(v) => {
        //         v.insert(a_b.0);
        //         if v.len() > 2 {
        //             println!("No");
        //             return
        //         }
        //     }
        //     None => {
        //         neighbors.insert(
        //             a_b.1,
        //             vec![a_b.0].into_iter().collect()
        //         );
        //     }
        // }
    }

    println!("Yes");
}

