use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut k: u64,
    }

    let mut res: String = "".to_string();

    while k > 0 {
        if k % 2 != 0 {
            res = format!("2{}", res);
            k = k - 1;
            k = k / 2;
        } else {
            res = format!("0{}", res);
            k = k / 2;
        }
    }

    println!("{}", res);
}

