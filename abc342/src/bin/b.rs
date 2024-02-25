use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        abs: [(usize, usize); q],
    }

    let order_map = p
        .iter()
        .enumerate()
        .map(|(i, pi)| (*pi, i))
        .collect::<std::collections::HashMap<_, _>>();

    for ab in abs {
        let a = order_map.get(&ab.0).unwrap();
        let b = order_map.get(&ab.1).unwrap();
        println!("{}", if a < b { ab.0 } else { ab.1 });
    }
}
