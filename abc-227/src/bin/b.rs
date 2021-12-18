use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [u64; n],
    }

    let mut count = 0;

    'root: for target in s {
        'a: for a in 1..=(target / 7) {
            for b in a..=(target / 7) {
                let area = 4 * a * b + 3 * a + 3 * b;
                if area == target {
                    continue 'root;
                }
                if area > target && b == a {
                    break 'a;
                } else if area > target {
                    break;
                }
            }
        }
        count += 1;
    };

    println!("{}", count);
}
