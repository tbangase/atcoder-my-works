use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            (a, s): (i64, i64),
        }
        let condition_1 = (2 * a) <= s;
        let condition_2 = (a & (s - 2 * a)) == 0;
        if  condition_1 && condition_2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
