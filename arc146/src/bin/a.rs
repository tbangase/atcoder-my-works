use proconio::{input, fastout};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut iter = a.iter().rev();
    let mut nums = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];

    nums.sort_by(|a, b| compare_two_cards(a, b));
    let mut iter = nums.iter().rev();

    println!("{}{}{}", iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap());
}

fn compare_two_cards(a: &usize, b: &usize) -> Ordering {
    let mut a_digit = a.digits();
    let mut b_digit = b.digits();

    while a_digit > 0 && b_digit > 0 {
        let a_current = a.digit_number(a_digit);
        let b_current = b.digit_number(b_digit);

        if a_current != b_current {
            return a_current.partial_cmp(&b_current).unwrap_or(Ordering::Equal);
        }
        a_digit -= 1;
        b_digit -= 1;
    }

    Ordering::Equal
}

trait Digit {
    fn digits(&self) -> usize;
    fn digit_number(&self, n: usize) -> usize;
}

impl Digit for usize {
    fn digits(&self) -> usize {
        let mut digits = 0;
        let mut n = *self;
        while n > 0 {
            n /= 10;
            digits += 1;
        }
        digits
    }
    fn digit_number(&self, n: usize) -> usize {
        let mut num = *self;
        for _ in 1..n {
            num /= 10;
        }
        num % 10
    }
}
