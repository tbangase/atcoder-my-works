use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
        (d, e, f): (usize, usize, usize),
        x: usize,
    }

    let a_status = x % (a + c);
    let b_status = x % (d + f);
    let a_length = if a_status < a {
        x / (a + c) * b * a + a_status * b
    } else {
        (x / (a + c) + 1)* b * a
    };

    let b_length = if b_status < b {
        x / (d + f) * e * d + b_status * e
    } else {
        (x / (d + f) + 1)* e * d
    };

    if a_length < b_length {
        println!("Aoki")
    } else if a_length > b_length {
        println!("Takahashi")
    } else {
        println!("Draw")
    }
}
