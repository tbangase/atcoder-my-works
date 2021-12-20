use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut diff_root = s[0] as i32 - t[0] as i32;
    if diff_root < 0 {
        diff_root = 26 + diff_root;
    }

    for i in 1..s.len() {
        let mut diff = s[i] as i32 - t[i] as i32;
        if diff < 0 {
            diff = 26 + diff;
        }
        if diff != diff_root {
            println!("No");
            return
        }

    }

    println!("Yes");
}
