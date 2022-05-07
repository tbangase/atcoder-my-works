use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    let mut result = std::i64::MAX;
    let mut j = 10i64.pow(6);

    for i in 0..10i64.pow(6) {
        while f(i, j) >= n && j >= 0 {
            result = result.min(f(i, j));
            j -= 1;
        }
    }


    println!("{:?}", result);
}

fn f(a: i64, b: i64) -> i64{
    a.pow(3) + a.pow(2) * b + a * b.pow(2) + b.pow(3)
}
