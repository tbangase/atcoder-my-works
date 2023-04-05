use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (i32, i32, i32, i32),
    }

    println!("{}", (a + b) * (c - d));
    println!("Takahashi");
}
