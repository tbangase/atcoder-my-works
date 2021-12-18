use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, d): (usize, usize),
    }

    let mut range_list: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        input! {
            (l, r): (usize, usize)
        }
        
        range_list.push((l, r));
    }

    range_list.sort_by_key(|k| k.1);

    let mut x = range_list[0].1;
    let mut result = 1;

    for range in range_list.into_iter() {
        if x + d - 1 < range.0 {
            result += 1;
            x = range.1;
        }
    }

    println!("{}", result);
}
