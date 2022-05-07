use proconio::{input, fastout};

const TAKA: &str = "Takahashi";
const AOKI: &str = "Aoki";

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize)
    }

    let mut non_primitive_seq_count = 0;

    let mut is_taka_won = false;

    for i in (a + c)..=(b + d) {
        if is_primitive(i) {
            non_primitive_seq_count = 0;
        } else {
            non_primitive_seq_count += 1;
            if d - c + 1 <= non_primitive_seq_count {
                is_taka_won = true;
                break;
            }
        }
    }

    match is_taka_won {
        true => println!("{}", TAKA),
        false => println!("{}", AOKI),
    }
}

fn is_primitive(target: usize) -> bool {
    let root = (target as f64).sqrt().floor() as usize;
    for i in 2..=root {
        if target % i == 0 {
            return false
        }
    }
    true
}
