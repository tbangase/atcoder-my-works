use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let mut escaped = false;

    let res = s
        .chars()
        .map(|c| {
            if c == '"' {
                escaped = !escaped;
                return c;
            } else {
                if c == ',' && !escaped {
                    return '.';
                } else {
                    return c;
                }
            }
        })
        .collect::<String>();

    println!("{}", res);
}
