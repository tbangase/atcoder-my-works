use itertools::Itertools;
use proconio::marker::Usize1;
 
fn main() {
    proconio::input! {
        (n, k): (usize, Usize1),
        p: [usize; n],
    }
    let mut b = vec![false; n + 1];
    p[..k].iter().for_each(|&x| b[x] = true);
    let mut i = 0;
    println!(
        "{}",
        p[k..].iter()
            .map(|&x| {
                b[x] = true;
                if x > i {
                    i += 1;
                    while !b[i] {
                        i += 1;
                    }
                }
                i
            })
            .join(" ")
    );
}
