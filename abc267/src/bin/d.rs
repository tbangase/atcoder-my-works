use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
    }

    for _ in 0..(n - m) {
        let s = a
            .iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<_>>();
        let sum_all = *s.last().unwrap_or(&0);
        let (index, _max) =
            s.iter()
                .enumerate()
                .fold((0, std::i64::MIN), |(index, max), (i, &s_i)| {
                    let loss = s_i - sum_all - a[i] * (i + 1) as i64;
                    if loss >= max {
                        (i, loss)
                    } else {
                        (index, max)
                    }
                });
        a.remove(index);
    }

    println!(
        "{:?}",
        a.iter()
            .enumerate()
            .fold(0, |acc, (i, v)| { acc + (i + 1) as i64 * v })
    );
}
