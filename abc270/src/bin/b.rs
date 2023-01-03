use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y, z): (i32, i32, i32)
    }

    let res = if x * y < 0 || x.abs() < y.abs() {
        // case 1. y を通らずに直接xにいける y.sign() != x.sign() or x.abs() < y.abs()
        x.abs()
    } else if z * y > 0 && z.abs() < y.abs() {
        // case 2. y の前にzを通ってxにいける case 1 以外かつ z.sign() = y.sign(), z.abs() < y.abs()
        x.abs()
    } else if z * y < 0 {
        // case 3. 回り道してzを通ってxにいける case 1,2 以外かつ z.sign() != y.sign()
        x.abs() + z.abs() * 2
    } else {
        // case 4. y の先にx, zがあるので辿り着けない
        -1
    };

    println!("{}", res);
}
