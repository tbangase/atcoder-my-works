use std::{collections::BTreeSet, io::Write, iter::FromIterator};

fn main() {
    let read = || -> usize {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap_or(0)
    };

    let n = read();
    let mut efficient_nums: BTreeSet<usize> = BTreeSet::from_iter(1..=(2*n+1));

    for _i in 0..=n {
        let target = efficient_nums.iter().next().unwrap_or(&0).to_owned();
        println!("{}", target);
        std::io::stdout().flush().unwrap();

        efficient_nums.remove(&target);

        let num = read();

        efficient_nums.remove(&num);
    }
}
