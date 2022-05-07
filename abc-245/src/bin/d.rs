use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [isize; n + 1],
        c: [isize; n + m + 1],
    }

    let mut b: Vec<isize> = vec![0; m + 1];

    for i in (0..=m).rev() {
        // b(i) = c[i] - b(i-1)a(1) - ... / a[0]
        let mut numerator = c[i + n];
        for j in (0..n).rev() {
            // Check Iterator will be continue or not
            if i + n - j > m {
                break;
            }
            // n > m の時 ?? 
            numerator -= a[j] * b[i + n - j];
        }
        b[i] = numerator / a[n];
    }

    for b_element in b {
        print!("{} ", b_element);
    }
    println!("");
}
