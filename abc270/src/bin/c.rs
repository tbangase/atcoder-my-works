use itertools::Itertools;
use petgraph::{algo, graph::UnGraph};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        (x, y): (u32, u32),
        edges: [(u32, u32); n - 1],
    }

    let g = UnGraph::<(), ()>::from_edges(&edges);
    let path = algo::all_simple_paths::<Vec<_>, _>(&g, x.into(), y.into(), 0, None)
        .collect_vec()
        .first()
        .cloned()
        .unwrap();

    println!("{}", path.iter().map(|x| x.index()).join(" "));
}
