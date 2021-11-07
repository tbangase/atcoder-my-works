use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: i32,
        s: [(i32, i32, i32); n],
    }

    println!("{:?}", s)
}
