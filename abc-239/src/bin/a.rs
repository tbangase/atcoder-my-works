use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        h: usize,
    }

    let h = h as f64;

    println!("{:?}", (h * (12800000.0 + h)).sqrt());
}
