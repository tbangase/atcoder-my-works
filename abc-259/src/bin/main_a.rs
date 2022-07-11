use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize,
    }

    let ans;
    if m >= x {
        ans = t;
    } else {
        ans = t - (d * (x - m));
    }

    println!("{}", ans);
}
