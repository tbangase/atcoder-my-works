use proconio::{input, fastout};

use std::time::Instant;
use petgraph::graph::UnGraph;

#[derive(Clone, Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self { x, y, r }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn is_cross(&self, other: &Self) -> bool {
        if self.distance(other.x, other.y) > (self.r + self.y) {
            false 
        } else {
            true
        }
    }

    pub fn distance(&self, x: f64, y: f64) -> f64 {
        ((self.x - x).powi(2) + (self.y - y).powi(2)).sqrt()
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: (f64, f64),
        t: (f64, f64),
    }

    let start = Instant::now();

    let mut circles = vec![];
    let mut edges = vec![];

    let mut s_index = std::usize::MAX;
    let mut t_index = std::usize::MAX;

    for i in 0..n {
        input! {
            (x, y, r): (f64, f64, f64)
        }

        let circle = Circle::new(x, y, r);

        for (j, exist_c) in circles.iter().enumerate() {
            if circle.is_cross(exist_c) {
                edges.push((i, j));
            }
        }

        if circle.distance(s.0, s.1) == circle.r() {
            s_index = i;
        }

        if circle.distance(t.0, t.1) == circle.r() {
            t_index = i;
        }
        
        circles.push(circle);
    }

    let g = UnGraph::<usize, ()>::from_edges(&edges[..]);

    println!("{:?}", s);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
