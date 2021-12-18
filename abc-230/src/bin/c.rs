use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (_, a, b): (u64, isize, isize),
        (p, q, r, s): (isize, isize, isize, isize)
    }

    for i in p..=q {
        for j in r..=s {
            if i == -1 * j + a + b || i == j + a - b {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

}
