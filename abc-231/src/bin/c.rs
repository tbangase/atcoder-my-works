use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        mut a: [usize; n],
        queries: [usize; q],
    }

    a.sort();

    for query in queries {
        let res = a.lower_bound(&query);
        println!("{}", n - res);
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
