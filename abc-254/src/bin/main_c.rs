use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        mut a: [u64; n],
    }

    let mut res = true;
    let mut list_by_k = vec![vec![]; k];

    for i in 0..n {
        let index = i % k;
        list_by_k[index].push(a[i]);
    }

    for i in 0..k {
        list_by_k[i].sort();
    }

    let mut prev_val = 0;

    for i in 0..=(n/k) {
        for j in 0..k {
            if let Some(list) = list_by_k.get(j) {
                if let Some(val) = list.get(i) {
                    if prev_val > *val {
                        res = false;
                    }
                    prev_val = *val;
                }
            }
        }
    }

    match res {
        true => println!("Yes"),
        false => println!("No"),
    }
}
