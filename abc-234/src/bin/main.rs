use proconio::{input, fastout};
use std::{collections::BinaryHeap, iter::FromIterator, cmp::Reverse};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        p: [usize; n]
    }

    let mut heap = BinaryHeap::from_iter(
        p[0..k].into_iter().map(Reverse)
    );

    if let Some(Reverse(val)) = heap.peek() {
        println!("{}", val);
    }
    for i in k..n {
        let mut minimum = 0;
        if let Some(Reverse(val)) = heap.peek() {
            minimum = **val;
        }

        if p[i] > minimum {
            heap.pop();
            heap.push(Reverse(&p[i]));
        }

        if let Some(Reverse(val)) = heap.peek() {
            println!("{}", val);
        }
    }
}
