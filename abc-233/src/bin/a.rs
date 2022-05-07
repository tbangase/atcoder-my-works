use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: i32,
        y: i32,
    }

    let diff = y - x;

    if diff > 0 {
        if diff % 10 == 0 {
            println!("{}", diff / 10);
        } else {
            println!("{}", diff / 10 + 1);
        }
    } else {
        println!("0");
    }
}
