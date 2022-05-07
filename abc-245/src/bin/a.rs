use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (usize,usize,usize,usize),
    }

    let tak = a * 60 + b;
    let aok = c * 60 + d;

    match tak > aok {
        true => println!("Aoki"),
        false => println!("Takahashi"),
    }
}
