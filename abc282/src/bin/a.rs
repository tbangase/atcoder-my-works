use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u8,
    }

    let s = (0..k).map(|v| (65u8 + v) as char).collect::<String>();

    println!("{}", s);
}
