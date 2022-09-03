use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize),
        n: usize,
    }

    if 3 * x < y {
        println!("{}", n * x);
    } else {
        println!("{}", n / 3 * y + n % 3 * x);
    }
}
