use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize,usize,usize),
    }

    match (a >= b && b >= c) || (a <= b && b <= c) {
        true => println!("Yes"),
        false => println!("No"),
    }
}
