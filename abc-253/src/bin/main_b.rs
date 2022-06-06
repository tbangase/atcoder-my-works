use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        (h, _w): (usize, usize),
        s_list: [Chars; h],
    }

    let mut pieces = vec![];
    
    for (i, s) in s_list.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if *c == 'o' {
                pieces.push((i, j));
            }
        }
    }

    let length = (pieces[0].0 as isize - pieces[1].0 as isize).abs() + (pieces[0].1 as isize - pieces[1].1 as isize).abs();

    println!("{}", length);
}
