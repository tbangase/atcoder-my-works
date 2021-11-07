use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: [i32; n],  // Vec<(i32, i32, i32)>
    }

    let mut max = 0;
    let mut total = 0;

    for li in l {
        if li > max {
            total += max;
            max = li;
            continue;
        }
        total += li;
    }

    if total > max {
        println!("Yes")
    } else {
        println!("No")
    }
}
