use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    let f = |t: usize| -> usize {
        t * t + 2 * t + 3
    };

    let res = f(f(f(t) + t) +f(f(t)));
    
    println!("{}", res);
}
