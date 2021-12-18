use proconio::{input, fastout};

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Cheese {
    deliciousness: u64,
    weight: u64,
}

impl PartialOrd for Cheese {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deliciousness.partial_cmp(&other.deliciousness)
    }
}

impl PartialEq for Cheese {
    fn eq(&self, other: &Self) -> bool {
        self.deliciousness == other.deliciousness
    }
}

impl Ord for Cheese {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deliciousness.cmp(&other.deliciousness)
    }
}

impl Eq for Cheese { }

#[fastout]
fn main() {
    input! {
        n: usize,
        w: u64,
    }

    let mut cheeses = vec![];

    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }

        cheeses.push(Cheese{deliciousness: a, weight: b});
    }

    cheeses.sort();
    cheeses.reverse();

    let mut my_cheese_weight = 0;

    let mut index = 0;
    let mut total_delicious = 0;

    while my_cheese_weight < w && index < cheeses.len(){
        let cheese = cheeses[index];

        let weight = (cheese.weight).min(w - my_cheese_weight);
        my_cheese_weight += weight;
        total_delicious += cheese.deliciousness * weight;
        
        index += 1;
    }

    println!("{:?}", total_delicious);
}
