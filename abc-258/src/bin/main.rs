use proconio::{input, fastout};
use petgraph::prelude::*;

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
        (q, x): (usize, usize),
        w: [usize; n],
    }

    let start = Instant::now();

    let mut g = DiGraph::<(), ()>::new();

    // TODO: Check if add_node is necessary or not
    for _ in 0..n {
        g.add_node(());
    }

    g.add_edge(1.into(), 2.into(), ());

    println!("Graph nodes count = {}", g.node_count());
    println!("Graph = {:#?}", g);

    println!("{:?}", w);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
