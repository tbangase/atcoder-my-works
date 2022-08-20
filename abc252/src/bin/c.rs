use proconio::{input, fastout};

use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        s_list: [String; n],
    }

    let s_vec: Vec<Vec<_>> = s_list.iter().map(|s| s.chars().collect()).collect();
    
    let mut count_map: HashMap<_, _> = (0..10).map(|v| (v, HashSet::new())).collect();

    for t in 0..(n * 10) {
        let mut appeared_map = HashSet::new();
        let t_mod = t % 10;
        for (i, s) in s_vec.iter().enumerate() {
            let c = s.get(t_mod).unwrap().to_digit(10).unwrap() as usize;
            if appeared_map.get(&c).is_none() {
                if let Some(v) = count_map.get_mut(&c) {
                    match v.get(&i) {
                        Some(_) => continue,
                        None => {
                            v.insert(i);
                            appeared_map.insert(c);
                        },
                    }
                    if v.len() >= n {
                        println!("{}", t);
                        return
                    }
                }
            } 
        }
    }
}
