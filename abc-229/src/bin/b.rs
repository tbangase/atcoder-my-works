use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut a: String,
        mut b: String,
    }

    let min_len = if a.len() < b.len() { a.len() } else { b.len() };

    let mut is_easy: bool = true;

    for i in 0..min_len {
        let a_char = a.chars().nth(a.len() - i - 1).unwrap();
        let b_char = b.chars().nth(b.len() - i - 1).unwrap();

        let added = a_char.to_string().parse::<i32>().unwrap()
            + b_char.to_string().parse::<i32>().unwrap();

        if added >= 10 {
            is_easy = false;
        }
    }

    println!("{}", if is_easy { "Easy" } else { "Hard" });

}
