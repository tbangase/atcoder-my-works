use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (a,b): (isize, isize),
    }

    if (a - b).abs() == 1 || (a - b).abs() == 9 {
        println!("Yes");
    } else {
        println!("No");
    }
}
