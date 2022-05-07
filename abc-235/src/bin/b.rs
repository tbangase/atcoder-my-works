use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }

    let mut current = 0;

    for i in 0..n {
        if current < h[i] {
            current = h[i];
        } else {
            break;
        }
    }

    println!("{:?}", current);
}
