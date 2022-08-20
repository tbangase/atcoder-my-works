use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", (n as u8) as char);
}
