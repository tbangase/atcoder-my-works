use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s1: String,
        s2: String,
    }

    for i in 0..2 {
        if '.' == s1.chars().nth(i).unwrap()
          && '.' == s2.chars().nth((i + 1) % 2).unwrap() {
            println!("No");
            return
        }
    }

    println!("Yes");
}
