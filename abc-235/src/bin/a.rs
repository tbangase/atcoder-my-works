use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        abc: Chars,
    }

    let mut total = 0;
    for c in abc {
        total += c.to_digit(10).unwrap();
    }

    let num = total * 100 + total * 10 + total;

    println!("{:?}", num);
}
