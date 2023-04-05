use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 10],
    }

    let mut a = 10;
    let mut b = 0;
    let mut c = 10;
    let mut d = 0;

    for (i, s_i) in s.iter().enumerate() {
        s_i.chars().enumerate().for_each(|(j, s_ij)| {
            if s_ij == '#' {
                a = a.min(i);
                b = b.max(i);
                c = c.min(j);
                d = d.max(j);
            }
        });
    }

    println!("{} {}", a + 1, b + 1);
    println!("{} {}", c + 1, d + 1);
}
