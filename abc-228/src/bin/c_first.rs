use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        p: [(u32, u32, u32); n],
    }

    let mut loss = vec![];

    for i in 0..n {
        let until_third_day = p[i].0 + p[i].1 + p[i].2;
        loss.push(1200 - until_third_day);
    }

    loss.sort();

    for i in 0..n {
        let until_third_day = p[i].0 + p[i].1 + p[i].2;
        let best = 1200 - (until_third_day + 300);

        let expected = loss.lower_bound(&best);

        loss.binary_search(&best).unwrap();

        if expected < k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x:&T) -> usize;
    fn upper_bound(&self, x:&T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x:&T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                },
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
        
    fn upper_bound(&self, x:&T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal=> {
                    low = mid + 1;
                },
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
