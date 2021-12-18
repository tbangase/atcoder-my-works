use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }

    if n > 41 {
        n += 1;
    }

    println!("AGC{:0>3}", n);
}
