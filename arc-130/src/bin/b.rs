use proconio::{input, fastout};
use std::cmp::Ordering;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Query {
    types: usize,
    index: usize,
    color: usize,
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl Eq for Query { }

#[fastout]
fn main() {
    input! {
        (height, width, colors, query_num): (u64, u64, usize, usize),
    }

    let mut queries: HashMap<(usize, usize), Query> = HashMap::new();

    for i in 0..query_num {
        input! {
            (t, n, c): (usize, usize, usize)
        }

        let query = Query {
            types: t,
            index: i,
            color: c - 1,
        };

        queries.insert((t, n), query);
    }

    let mut queries = queries.values().cloned().collect::<Vec<Query>>();

    queries.sort();
    queries.reverse();

    let mut row_count = 0;
    let mut col_count = 0;

    let mut color_counts = vec![0; colors];

    for query in &queries {
        if query.types == 1 {
            color_counts[query.color] += width - col_count;
            row_count += 1;
        } else {
            color_counts[query.color] += height - row_count;
            col_count += 1;
        }
        color_counts[query.color];
    }

    for color in color_counts {
        print!("{} ", color);
    }
    println!("");
}
