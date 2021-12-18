use proconio::{input, fastout, marker::*};

#[allow(unused_variables)]
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut count: i64 = 1;
    let mut current: char = ' ';

    let mut res: i64 = 0;

    for c in s {
        match current == c {
            true => count += 1,
            false => {
                res += count * (count - 1) / 2;
                current = c;
                count = 1;
            },
        }
    }

    res += count * (count - 1) / 2;

    println!("{}", res);
}
