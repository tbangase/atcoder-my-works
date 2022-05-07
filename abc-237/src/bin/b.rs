use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        a: [[usize; w]; h],
    }

    for i in 0..w {
        for j in 0..h {
            print!("{} ", a[j][i]);
        }
        println!("");
    }
}
