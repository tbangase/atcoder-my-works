use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy_list: [(isize, isize); n]
    }

    let get_length = |p1: (isize, isize), p2: (isize, isize)| -> f64 {
        let power = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2);
        (power as f64).sqrt()
    };

    let mut max = 0 as f64;
    
    for i in 0..(n-1) {
        for j in (i+1)..n {
            let length = get_length(xy_list[i], xy_list[j]);
            if length > max {
                max = length;
            }
        }
    }

    println!("{}", max);
}
