use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n-1],
    }

    let p: Vec<isize> = p.iter().map(|&v| v as isize - 2).collect();

    let mut target = *p.iter().last().unwrap();
    let mut count = 1;

    while target >= 0 {
        target = p[target as usize];
        count += 1;
    }

    println!("{}", count);
}
