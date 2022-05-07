use petgraph::graph::{NodeIndex, UnGraph};
use proconio::{input, fastout, marker::*};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        x: [usize; n],
        connections: [(usize, usize); n - 1],
        query: [(usize, usize); q],
    }

    let start = Instant::now();

    let mut edges = vec![];

    for connection in connections {
        edges.push((NodeIndex::new(connection.0), NodeIndex::new(connection.1)));
    }

    let graph = UnGraph::<i32, ()>::from_edges(edges.iter());

    println!("{:?}", graph);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
