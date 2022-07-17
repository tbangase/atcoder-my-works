use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let m = 2 * n;

    println!("{}", m);

    // calc x
    let first = n % 4;
    let digits = n / 4;
    if first != 0 {
        print!("{}", first);
    }
    for _ in 0..digits {
        print!("4");
    }
    println!();
}
