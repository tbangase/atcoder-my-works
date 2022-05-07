use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, x): (usize, u64),
    }

    let mut a_list = vec![];

    for _ in 0..n {
        input! {
            l: usize,
            a: [u64; l],
        }

        a_list.push(a);
    }

    let mut count = 0;

    eval_product(&mut count, 1, 0, &a_list, x);

    println!("{:?}", count);
}

fn eval_product(
    count: &mut u64, 
    current_product: u64, 
    index: usize, 
    a_list: &Vec<Vec<u64>>,
    x: u64
) {
    if index == a_list.len() {
        // evaluate here
        if current_product == x {
            *count += 1;
        }
        return
    }

    if current_product > x {
        return
    }

    for i in 0..a_list[index].len() {
        eval_product(
            count, 
            current_product * a_list[index][i],
            index + 1,
            a_list,
            x);
    }
}
