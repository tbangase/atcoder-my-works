use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        p: [(u32, u32, u32); n],
    }

    let mut currents = vec![];

    for i in 0..n {
        let until_third_day = p[i].0 + p[i].1 + p[i].2;
        currents.push(until_third_day);
    }

    currents.sort();
    currents.reverse();

    for i in 0..n {
        let until_third_day = p[i].0 + p[i].1 + p[i].2;
        let best = until_third_day + 300;

        if best >= currents[k - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

