use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let a = s[0].to_string().parse::<usize>().unwrap();
    let b = s[2].to_string().parse::<usize>().unwrap();

    println!("{}", a * b);
}
