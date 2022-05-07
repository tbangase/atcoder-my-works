use proconio::{input, fastout};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (a, n): (u64, u64),
    }

    let start = Instant::now();
    
    let mut current = 1;
    let mut multiply_count = 0;

    while current.digits() < n.digits() {
        current *= a;
        multiply_count += 1;
    }

    while current.digits() == n.digits() {
        println!("Current: {}, Target: {}, Count: {}" , current, n, multiply_count);
        let mut rotate_count = 0;
        for _ in 0..(current.digits()) {
            current = current.rotate();
            rotate_count += 1;
            if current == n {
                println!("{}", multiply_count + rotate_count);
                return
            }

        }
        current *= a;
        multiply_count += 1;
    }

    println!("{}", -1);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

pub trait CountDigits<T> {
    fn digits(&self) -> u32;
}

impl CountDigits<u64> for u64 {
    fn digits(&self) -> u32 {
        let mut count = 0;
        let mut target = *self;
        while target > 0 {
            count += 1;
            target /= 10;
        }
        count
    }
}

pub trait RotateDigits<T> {
    fn rotate(&self) -> T;
}

impl RotateDigits<u64> for u64 {
    fn rotate(&self) -> u64 {
        if *self == 0 {return 0}
        let ones = self % 10;
        let lefts = self / 10;
        lefts + ones * 10_u64.pow(self.digits() - 1)
    }
}

#[test]
fn digits_u64_test() {
    let test_num = 12345;
    assert_eq!(5, test_num.digits());
    let test_num = 1;
    assert_eq!(1, test_num.digits());
    let test_num = 0;
    assert_eq!(0, test_num.digits());
}

#[test]
fn rotate_u64_test() {
    let test_num = 12345;
    assert_eq!(51234, test_num.rotate());
    let test_num = 472938;
    assert_eq!(847293, test_num.rotate());
    let test_num = 1;
    assert_eq!(1, test_num.rotate());
    let test_num = 0;
    assert_eq!(0, test_num.rotate());
}
