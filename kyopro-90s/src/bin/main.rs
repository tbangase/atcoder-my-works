use proconio::fastout;
use proconio::input;

use std::time::Instant;
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        (n, l, k): (usize, u32, usize),
        mut a: [u32; n],
    }

    let start = Instant::now();

    // 2分探索で特定していく
    // その得点を取得可能かは貪欲法で調べていく。

    a.upper_bound(&(k as u32));
    

    println!("{:?}", points);
    let duration = start.elapsed();
}



pub trait BinarySearch<T> {
    fn lower_bound(&self, x:&T) -> usize;
    fn upper_bound(&self, x:&T) -> usize;
}

impl<T: Ord> BinarySearch<T> for Vec<T> {
    fn lower_bound(&self, x:&T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            // Check if 1. can cut or 2. not.
            for item in self.iter() {
                
            }
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
