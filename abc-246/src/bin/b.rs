use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (a, b) : (f64, f64)
    }

    let norm = a.powi(2) + b.powi(2);
    let norm = norm.sqrt();

    let a = a / norm;
    let b = b / norm;

    println!("{} {}", a, b);
}
