use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut n: i64,
        l: i64,
        r: i64,
    }

    let mut place = 1;
    let mut result = 0;

    while n > 0 {
        // while n has 1 in bits
        if n & 1 > 0 {
            // If watching place is 1
            let max_in_place = place * 2 - 1;
            result += (r.min(max_in_place) - l.max(place) + 1).max(0);
        }
        place <<= 1;
        n >>= 1;
    }
    
    println!("{}", result);
}
