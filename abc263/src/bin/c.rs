use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    select_and_print_ge(vec![], n, m);
}

fn select_and_print_ge(numbers: Vec<usize>, n: usize, m: usize) {
    if numbers.len() == n {
        for num in numbers {
            print!("{} ", num);
        }
        println!();
        return
    }

    let biggest = *numbers.last().unwrap_or(&0);
    for num in (biggest + 1)..=m {
        let mut numbers = numbers.clone();
        numbers.push(num);
        select_and_print_ge(numbers, n, m);
    }
}
