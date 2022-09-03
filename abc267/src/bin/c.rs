use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }

    let s = vec![0]
        .iter()
        .chain(a.iter())
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect::<Vec<_>>();

    let init = a[0..m]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &val)| acc + (i + 1) as i64 * val);

    let index_a_list = (0..n - m)
        .scan(init, |state, i| {
            *state -= s[i + m] - s[i];
            *state += m as i64 * a[i + m];
            Some(*state)
        })
        .collect::<Vec<_>>();
    let ans = init.max(*index_a_list.iter().max().unwrap_or(&std::i64::MIN));

    println!("{:?}", ans);
}
