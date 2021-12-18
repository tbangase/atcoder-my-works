use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        peoples: i64,
        cards: i64,
        starts: i64,
    }

    let mut finish = (starts + cards - 1) % peoples;
    if finish == 0 {
        finish = peoples;
    }

    println!("{}", finish);
}
