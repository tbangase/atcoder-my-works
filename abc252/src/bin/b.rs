use itertools::Itertools;
use proconio::{input, fastout};

// TODO: Remove before submit.
use std::{time::Instant, collections::HashSet};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        mut a: [usize; n],
        b: [usize; k],
    }

    // TODO: Remove before submit.
    let start = Instant::now();

    let b_set: HashSet<usize> = b.iter().map(|v| *v - 1).collect();

    let eat_que: Vec<(usize, &usize)> = a.iter().enumerate()
        .sorted_by_key(|(_, v)| *v)
        .rev().collect();

    let mut is_include_hated_food = false;
    let first_val = *eat_que.first().unwrap().1;

    for (index, &value) in eat_que.iter() {
        if value != first_val {
            break
        }
        
        if let Some(_) =  b_set.get(index) {
            is_include_hated_food = true;
        };
    }
    
    match is_include_hated_food {
        true => println!("Yes"),
        false => println!("No")
    }

    // TODO: Remove before submit.
    // println!("Duration: {:?}", start.elapsed());
}
