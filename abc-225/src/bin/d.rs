// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (_, query_num): (usize, usize),
    }

    let mut linked_info: HashMap<usize, usize> = HashMap::new();
    let mut rev_linked_info: HashMap<usize, usize> = HashMap::new();

    for _ in 0..query_num {
        input! {
            distinct: usize
        }

        if distinct == 1 {
            input! {
                x: usize,
                y: usize,
            }
            linked_info.insert(x, y);
            rev_linked_info.insert(y, x);
        } else if distinct == 2 {
            input! {
                x: usize, y: usize
            }
            linked_info.remove(&x);
            rev_linked_info.remove(&y);
        } else if distinct == 3 {
            input! {
                x: usize
            }

            let lead = get_lead(&rev_linked_info, x);

            let chains = get_chains(&linked_info, lead);

            print!("{} ", chains.len());

            for train in chains.iter().rev() {
                print!("{} ", train);
            }
            println!("")
        }
    }
}

fn get_chains(linked_info: &HashMap<usize, usize>, lead: usize) -> Vec<usize> {
    if let Some(val) = linked_info.get(&lead) {
        let mut chains = get_chains(linked_info, *val);
        chains.push(lead);
        chains
    } else {
        vec![lead]
    }
}

fn get_lead(rev_info: &HashMap<usize, usize>, x: usize) -> usize {
    if let Some(val) = rev_info.get(&x) {
        get_lead(&rev_info, *val)
    } else {
        x
    }
}
