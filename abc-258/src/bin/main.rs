use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, x): (usize, usize),
        abs: [(usize, usize); n]
    }

    let mut over_head = 0;
    let mut min = std::usize::MAX;
    for (i, (a, b)) in abs.iter().enumerate() {
        over_head += a + b;
        let stayed_score = over_head + b * (x - i - 1);
        if min > stayed_score {
            min = stayed_score;
        }
    }

    println!("{}", min);
}
