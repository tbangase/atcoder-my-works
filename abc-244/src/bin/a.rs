use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        _n: usize,
        a: Chars,
    }

    println!("{}", a.iter().last().unwrap());
}
