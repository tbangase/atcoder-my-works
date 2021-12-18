use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    let mut a: i64 = 1;

    let mut count: i64 = 0;

    while n >= a * a * a {
        let b_max = n / a;
        let mut b: i64 = a;
        loop {
            let c_count = b_max / b - b + 1;
            if c_count <= 0 {
                break;
            }
            count += c_count;
            b+=1;
        }
        a+=1;
    };

    println!("{}", count);
}
