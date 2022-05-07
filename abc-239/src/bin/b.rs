use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: i64,
    }

    let has_decimal = x % 10 != 0;
    let is_negative = x < 0;
    let mut x = x / 10;

    if is_negative && has_decimal {
        x -= 1;
    }

    println!("{}", x);
}
