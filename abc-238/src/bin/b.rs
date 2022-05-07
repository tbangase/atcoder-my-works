use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_s: [usize; n],
    }

    let mut cuts = vec![0, 360];

    let mut total = 0;
    for a in a_s.iter() {
        total += a;
        cuts.push(total % 360);
    }

    cuts.sort();
    
    let mut max = 0;
    for i in 1..=(n + 1) {
        let diff_deg = cuts[i] - cuts[i-1];
        if max < diff_deg {
            max = diff_deg;
        };
    }

    println!("{}", max);
}
