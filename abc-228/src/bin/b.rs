use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, x): (usize, usize),
        a: [usize; n],
    }

    let mut knows = vec![false; n];

    knows[x - 1] = true;

    let mut count = 1;
    let mut next = x;

    loop {
        next = a[next - 1];
        if knows[next - 1] == false {
            count += 1;
            knows[next - 1] = true;
        } else {
            break;
        }
    }

    println!("{}", count);
}
