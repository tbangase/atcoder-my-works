use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        t: f64,
        x: f64,
    }

    println!("{}", t / x)
}
