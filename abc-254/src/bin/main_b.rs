use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut list = vec![1];
    println!("1");

    for _ in 1..n {
        let mut new_list = vec![];
        new_list.push(list[0]);
        for j in 0..(list.len() - 1) {
            new_list.push(list[j] + list[j + 1]);
        }
        new_list.push(list[list.len() - 1]);
        list = new_list;
        for num in list.iter() {
            print!("{} ", num);
        }
        println!("");
    }
}
