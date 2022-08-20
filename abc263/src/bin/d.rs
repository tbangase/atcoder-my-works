use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        (l, r): (i64, i64),
        a: [i64; n],
    }

    let sum: i64 = a.iter().sum();

    let mut l_reducible = vec![0];
    let mut reduced = 0;
    let mut min = 0;
    for &a_i in a.iter() {
        reduced += l - a_i;
        min = reduced.min(min);
        l_reducible.push(min);
    }

    let mut r_reducible = vec![0];
    let mut reduced = 0;
    let mut min = 0;
    for &a_i in a.iter().rev() {
        reduced += r - a_i;
        min = reduced.min(min);
        r_reducible.push(min);
    }

    let ans = l_reducible.iter().enumerate()
        .fold(0, |min, (i, &v)| min.min(v + r_reducible[n - i]));

    println!("{}", sum + ans);
}
