use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n == 1 || n > 4 {
        println!("Yes");
    } else {
        println!("No");
    }

}
