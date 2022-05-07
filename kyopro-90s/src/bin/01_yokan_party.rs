use proconio::{input, fastout};

// 1. Star: 4
// Yokan Party
// 答えで二分探索
#[fastout]
fn main() {
    input! {
        (n, l, k): (usize, usize, usize),
        a: [usize; n],
    }

    let mut low = 0;
    let mut high = l;

    let mut pieces = vec![];
    let mut prev = 0;
    for a_i in a.iter() {
        pieces.push(a_i - prev);
        prev = *a_i;
    }
    pieces.push(l - a.last().unwrap());

    while low != high {
        let mid = (low + high) / 2;
        match cut_for_check(mid, k, &pieces[..]) {
            true => {
                // なぜmid + 1??
                low = mid + 1;
            },
            false => {
                high = mid;
            }
        }
    }

    println!("{:?}", low - 1);
}

fn cut_for_check(min_cut: usize, k: usize, a: &[usize]) -> bool {
    let mut k = k;
    let mut current_block = 0;
    for a_i in a {
        current_block += a_i;
        if current_block >= min_cut && k != 0 {
            k -= 1;
            current_block = 0;
        }
        if current_block >= min_cut {
            return true
        }
    }
    false
}
