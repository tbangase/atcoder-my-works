use proconio::input;
 
fn main() {
    input! {
        mut n: i64,
        l: i64,
        r: i64,
    }
    let mut ans = 0;
    // What is mn?
    // mnは現在計算している2進数の位の数
    let mut mn = 1;
    while n > 0 {
        // n & 1 はnと1のビットごとのANDをとってる
        // n & 1 はnの2進数1桁目の値をとってきている
        if n & 1 > 0 {
            // mxについて
            // mnが4ならばmxは7
            // 4の位で表現できるのは4 ~ 7
            let mx = mn * 2 - 1;
            // その中の個数を数えている．
            ans += (r.min(mx) - l.max(mn) + 1).max(0);
        }
        mn *= 2;
        n /= 2;
    }
    println!("{}", ans);
}
