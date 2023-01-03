use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (u8, u8),
        strings: [String; n],
    }

    let all_solvable = 2u8.pow(m.into()) - 1;

    let solvable_list = strings
        .iter()
        .map(|s| {
            let mut flags: u8 = 0;
            for c in s.chars() {
                flags <<= 1;
                if c == 'o' {
                    flags |= 1;
                }
            }
            flags
        })
        .collect::<Vec<u8>>();

    let res = solvable_list
        .iter()
        .combinations(2)
        .filter(|comb| {
            let pair_solvable = comb[0] | comb[1];
            pair_solvable == all_solvable
        })
        .count();
    println!("{}", res);
}
