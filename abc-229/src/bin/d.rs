use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String,
        k: usize,
    }

    let s = s.chars().collect::<Vec<_>>();

    println!("{}", max_island(s, k));
}

fn max_island(s: Vec<char>, k: usize) -> usize {
    let mut bridge_count = 0;
    let mut right = 0;
    let mut left = 0;
    let mut max = 0;

    while right < s.len() {
        if '.' == s[right] {
            bridge_count += 1;
            if bridge_count > k {
                while '.' != s[left] {
                    left += 1;
                }
                left += 1;
                bridge_count -= 1;
            }
        }
        max = max.max(right - left + 1);
        right += 1;
    }
    max
}

