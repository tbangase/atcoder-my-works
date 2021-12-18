use proconio::{input, fastout};

#[fastout]
fn main() {
    const LIST_NUM: usize = 1048576;
    input! {
        q: usize,
    }

    let mut list = vec![-1; LIST_NUM];

    for _ in 0..q {
        input! {
            (t, x): (usize, i64)
        }

        if t == 1 {
            let mut h = (x % LIST_NUM as i64) as usize;

            while list[h] != -1 {
                h += 1;
                h = h % LIST_NUM;
            }

            list[h] = x;
        } else if t == 2 {
            let h = (x % (LIST_NUM as i64)) as usize;
            println!("{}", list[h]);
        }
    }
}
