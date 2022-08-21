use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, mut m, k): (usize, usize, usize),
        mut a: [usize; n],
    }

    let mut ans = 0;

    for i in (0..31).rev() {
        // aを降順にする
        a.sort_by(|a, b| b.cmp(a));

        // 1を左にiビットシフトさせて2のi乗を求める
        let t = 1 << i;
        let mut tot = 0;

        // 大きい順でk番目の要素に対して、必要な操作実行回数を計算する
        for &a_i in &a[..k] {
            if a_i >= t {
                continue;
            }
            tot += t - a_i;
        }

        let neo_a = if m >= tot {
            // 必要な操作実行回数がm以下なら
            ans += t;
            m -= tot;
            a .iter()
                // 2のi乗以下 もしくは 2進数i桁目が1になっている要素だけ残す
                .filter_map(|&b_i| if b_i.max(t) & t == t { Some(b_i.max(t)) } else { None })
                // t以下のa_iは2のi乗になってるがなぜうまく機能する?
                .collect::<Vec<_>>()
        } else {
            a.clone()
        };

        // "bの各要素に対して、t - 1とのANDをとる" = "2のi乗で割ったあまりをとる"
        a = neo_a.iter().map(|v| v & (t - 1)).collect::<Vec<_>>();
    }
    println!("{}", ans);
}
