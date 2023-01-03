use petgraph::graph::UnGraph;
use proconio::{fastout, input};

// TODO: Remove before submit.
use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        edges: [(usize, usize); m],
    }

    // TODO: Remove before submit.
    let start = Instant::now();

    let mut graph = UnGraph::<(), ()>::new_undirected();
    // graph.add_node()

    println!("{:?}", s);

    // TODO: Remove before submit.
    println!("Duration: {:?}", start.elapsed());
}
