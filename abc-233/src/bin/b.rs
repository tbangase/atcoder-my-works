use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        (mut l, mut r): (usize, usize),
        mut s: Chars,
    }

    l = l - 1;
    r = r - 1;

    let mut partial = vec![];

    for i in l..=r {
        partial.push(s[i]);
    }

    for i in l..=r {
        s[i] = partial[r - i];
    }

    for c in s {
        print!("{}",c);
    }

    println!("");
}
