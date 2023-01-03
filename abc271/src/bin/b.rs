use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut seq_list = vec![];

    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }

        seq_list.push(a);
    }

    for _ in 0..q {
        input! {
            (s, t): (usize, usize),
        }

        println!("{}", seq_list[s - 1][t - 1]);
    }
}
