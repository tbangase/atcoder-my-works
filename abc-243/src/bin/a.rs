use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        v: usize,
        (a, b, c): (usize, usize, usize),
    }

    let last_turn = v % (a + b + c);

    if last_turn >= a {
        if (last_turn - a) >= b {
            println!("T");
        } else {
            println!("M");
        }
    } else {
        println!("F");
    }
}
