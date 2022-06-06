use proconio::input;

use std::collections::HashMap;

fn main() {
    input! {
        q: usize,
    }

    let mut dataset = DataSet::new();

    for _ in 0..q {
        input! {
            mode: usize
        }

        match mode {
            1 => dataset.add(),
            2 => dataset.subtract(),
            3 => dataset.output(),
            _ => ()
        }
    }
}

#[derive(Debug)]
struct DataSet {
    pub s: HashMap<u64, u64>,
    pub min: u64,
    pub max: u64,
}

impl DataSet {
    pub fn new() -> Self {
        Self { s: HashMap::new(), min: 0, max: 0 }
    }

    pub fn add(&mut self) {
        input! {
            x: u64
        }

        *self.s.entry(x).or_insert(0) += 1;
        
        if x > self.max || x < self.min {
            self.get_min_max();
        }
    }

    pub fn subtract(&mut self) {
        input! {
            x: u64,
            c: usize,
        }

        for _ in 0..c {
            match self.s.get_mut(&x) {
                Some(val) => {
                    if *val != 1 {
                        *val -= 1;
                    } else {
                        self.s.remove(&x);
                        if self.max == x || self.min == x {
                            self.get_min_max();
                        }
                    }
                },
                None => return
            };
        }
        
    }

    pub fn get_min_max(&mut self) {
        let mut vec: Vec<_> = self.s.keys().cloned().collect();
        vec.sort();
        if vec.len() > 0 {
            self.min = vec[0];
            self.max = vec[vec.len() - 1];
        } else {
            self.min = 0;
            self.max = 0;
        }
    }

    pub fn output(&mut self) {
        println!("{}", self.max - self.min);
    }
}

