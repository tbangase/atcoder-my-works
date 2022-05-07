use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    if n <= std::i32::MAX as i64 && n >= std::i32::MIN as i64 { 
        println!("Yes");
    } else {
        println!("No");
    }
}
