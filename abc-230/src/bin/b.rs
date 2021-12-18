use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut x_count = 0;
    let mut o_count = 0;
    let mut repeat = 0;
    let mut is_first = true;
    let mut result = true;

    for c in s {
        if c == 'x' {
            x_count += 1;
            if x_count > 2 {
                result = false;
                break;
            }
            o_count = 0;
        } else {
            if x_count != 0 || o_count != 0 {
                repeat += 1;
            }
            if is_first {
                is_first = false;
                x_count = 0;
                continue;
            }

            o_count += 1;

            if o_count > 1 || x_count != 2 || repeat > 100000 {
                result = false;
                break;
            }
            x_count = 0;
        }
    }

    match result {
        true => println!("Yes"),
        false => println!("No"),
    }
}
