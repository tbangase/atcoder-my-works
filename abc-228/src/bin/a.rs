use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    }

    let mut res = false;

    if s < t {
        if s <= x && x < t {
            res = true;
        } 
    } else {
        if s <= x || x < t {
           res = true; 
        }
    }

    match res {
        true => println!("Yes"),
        false => println!("No"),
    }
}
