use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{:02}", n%100);
}
