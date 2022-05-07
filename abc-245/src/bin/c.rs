use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, isize),
        a_list: [isize; n],
        b_list: [isize; n],
    }

    let mut a_route = Some(a_list.get(0).unwrap());
    let mut b_route = Some(b_list.get(0).unwrap());

    let mut result = true;

    for i in 0..n {
        // K以下かつより高い方を選んでいく
        let a = a_list.get(i).unwrap();
        let b = b_list.get(i).unwrap();

        // とりうるルートは選択肢として残しておく
        let mut can_be_a = false;
        let mut can_be_b = false;

        if let Some(prev_a) = a_route {
            if (*a - *prev_a).abs() <= k {
                can_be_a = true;
            }
            if (*b - *prev_a).abs() <= k {
                can_be_b = true;
            }
        }
        if let Some(prev_b) = b_route {
            if (*a - *prev_b).abs() <= k {
                can_be_a = true;
            }
            if (*b - *prev_b).abs() <= k {
                can_be_b = true;
            }
        }

        if can_be_a {
            a_route = Some(a);
        } else {
            a_route = None;
        }

        if can_be_b {
            b_route = Some(b);
        } else {
            b_route = None;
        }

        if a_route == None && b_route == None {
            result = false;
            break;
        }
    }
    match result {
        true => println!("Yes"),
        false => println!("No"),
    }
}
