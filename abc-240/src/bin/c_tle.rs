use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, x): (usize, usize),
        choises: [(usize, usize); n],
    }

    match find(0, &choises, 0, x) {
        true => println!("Yes"),
        false => println!("No"),
    }
}

fn find(index: usize, choises: &Vec<(usize, usize)>, current: usize, x: usize) -> bool {
    let choise = choises[index];
    let next_1 = current + choise.0;
    let next_2 = current + choise.1;

    if index == choises.len() - 1 {
        if next_1 == x || next_2 == x {
            return true
        } else {
            return false
        }
    }

    if next_1 < x {
        if find(index + 1, choises, next_1, x) {
            return true
        }
    } else {
        return false
    }

    if next_2 < x {
        if find(index + 1, choises, next_2, x) {
            return true
        }
    } else {
        return false
    }

    false
}
