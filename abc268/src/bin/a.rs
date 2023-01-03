use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c, d, e): (i32, i32, i32, i32, i32),
    }

    let set = vec![a, b, c, d, e]
        .into_iter()
        .collect::<std::collections::HashSet<_>>();

    println!("{}", set.len());
}
