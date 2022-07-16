use proconio::input;

// TODO: Remove before submit.
use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    // TODO: Remove before submit.
    let start = Instant::now();

    println!("{:?}", s);

    // TODO: Remove before submit.
    println!("Duration: {:?}", start.elapsed());
}
