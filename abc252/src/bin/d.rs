use proconio::{fastout, input};

const UPPER_LIMIT: usize = 200000;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0; UPPER_LIMIT + 1];

    for &a_i in a.iter() { cnt[a_i] += 1 }
    for i in 1..=UPPER_LIMIT { cnt[i] += cnt[i - 1] }

    let mut ans: i64 = 0;
    for &a_i in a.iter() {
        ans += cnt[a_i - 1] * (cnt[UPPER_LIMIT] - cnt[a_i]);
    }

    println!("{}", ans);
}
