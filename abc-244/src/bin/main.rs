use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (s1, s2, s3): (char, char, char),
        (t1, t2, t3): (char, char, char),
    }

    let mut true_count = 0;

    if s1 == t1 {
        true_count += 1;
    }
    if s2 == t2 {
        true_count += 1;
    }
    if s3 == t3 {
        true_count += 1;
    }
    
    if true_count == 1 {
        println!("No")
    } else {
        println!("Yes")
    }
}
