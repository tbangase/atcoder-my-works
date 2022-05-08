use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        (r, c): (usize, usize),
    }
    let mut ans = 4;

    if r == 1 {
        ans -= 1;
    }
    if r == h {
        ans -= 1;
    }
    if c == 1 {
        ans -= 1;
    }
    if c == w {
        ans -= 1;
    }

    println!("{}", ans);
}
